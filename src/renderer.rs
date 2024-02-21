use nalgebra::{Matrix4, Point2, Point3};

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::geometry;
use crate::graphics;
use crate::math;
use crate::world_objects::{Light, PointLight, AmbientLight};
use rand::Rng;

pub fn render_geometry(
    screen_buffer: &mut [[u16; SCREEN_WIDTH]; SCREEN_HEIGHT],
    geometry: &Vec<geometry::Triangle3>,
    world_lights: &Vec<Light>,
    projection_matrix: &Matrix4<f64>,
    projection_matrix_inverse: &Matrix4<f64>,
    camera_transform: &Matrix4<f64>,
    ansi_background_color: u16,
) -> Vec<geometry::ProjectionResult> {
    let lights: Vec<Light> = world_lights
        .iter()
        .map(|world_light| {
            match world_light {
                Light::PointLight(point_light) => {
                    let origin = geometry::transform_world_vertice_to_camera_coords(
                        &point_light.get_origin(),
                        camera_transform,
                    );
                    Light::PointLight(PointLight {
                        origin,
                        intensity: point_light.intensity,
                        color: point_light.color,
                    })
                },
                Light::AmbientLight(ambient_light) => {
                    Light::AmbientLight(AmbientLight {
                        intensity: ambient_light.intensity,
                        color: ambient_light.color,
                    })
                },
            }
        })
        .collect();

    let mut z_buffer: [[f64; SCREEN_WIDTH]; SCREEN_HEIGHT] =
        [[f64::MAX; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut projection_buffer: [[usize; SCREEN_WIDTH]; SCREEN_HEIGHT] =
        [[usize::MAX; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut cached_projection_results = Vec::with_capacity(geometry.len());

    for triangle in geometry {
        // world cords -> camera coords -> ndc -> screen coords
        let projection_results =
            geometry::project_triangle(triangle, &projection_matrix, &camera_transform);

        for projection_result in &projection_results {
            cached_projection_results.push(*projection_result);
            let projection_result_index = cached_projection_results.len() - 1;

            let bounding_box = &projection_result.screen_bounding_box;
            let x_min = bounding_box.x_min.max(0);
            let y_min = bounding_box.y_min.max(0);
            let x_max = bounding_box.x_max.min(SCREEN_WIDTH);
            let y_max = bounding_box.y_max.min(SCREEN_HEIGHT);

            // Rasterize
            for y in y_min..y_max {
                for x in x_min..x_max {
                    let px = (x as f64) + 0.5;
                    let py = (y as f64) + 0.5;
                    let pixel = Point2::new(px, py);

                    if !geometry::is_point_in_triangle(&pixel, &projection_result.screen_triangle) {
                        continue;
                    }

                    let z = graphics::interpolate_attributes_at_pixel(&pixel, &projection_result);

                    // pixel in this triangle is behind another triangle
                    if z >= z_buffer[y][x] {
                        continue;
                    }

                    z_buffer[y][x] = z;
                    projection_buffer[y][x] = projection_result_index;
                }
            }
        }
    }

    let mut r_dithering_errors: [[i16; SCREEN_WIDTH]; SCREEN_HEIGHT] =
        [[0; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut g_dithering_errors: [[i16; SCREEN_WIDTH]; SCREEN_HEIGHT] =
        [[0; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut b_dithering_errors: [[i16; SCREEN_WIDTH]; SCREEN_HEIGHT] =
        [[0; SCREEN_WIDTH]; SCREEN_HEIGHT];

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let projection_result_index = projection_buffer[y][x];
            if projection_result_index == usize::MAX {
                screen_buffer[y][x] = ansi_background_color;
                continue;
            }
            let projection_result = &cached_projection_results[projection_result_index];

            let pixel = Point3::new((x as f64) + 0.5, (y as f64) + 0.5, z_buffer[y][x]);

            let p_ndc = geometry::screen_to_ndc(&pixel).to_homogeneous();
            let point_camera_space_homogeneous = projection_matrix_inverse * p_ndc;
            let point_camera_space =
                point_camera_space_homogeneous.xyz() / point_camera_space_homogeneous.w;

            let light_intensity = lights
                .iter()
                .map(|light| {
                    match light {
                        Light::PointLight(point_light) => {
                            let origin = point_light.get_origin();
                            let light_norm = (origin - point_camera_space).coords.normalize();
                            let diffuse_intensity =
                                light_norm.dot(&projection_result.normal).max(0.0);

                            //let a = 0.5;
                            //let b = 0.3;
                            let a = 0.1;
                            let b = 0.5;

                            let distance = (origin - point_camera_space).coords.magnitude();
                            let attenuation = 1.0 / (1.0 + a * distance + b * distance * distance);

                            diffuse_intensity// * attenuation * light.get_intensity()
                        },
                        Light::AmbientLight(_) => {
                            light.get_intensity()
                        }
                    }
                })
                .sum::<f64>()
                .min(1.0);
                
            let correct_color = |input: f64| -> u8 {
                if input == 0.0 {
                    return input as u8;
                }
                math::scale_range(input * light_intensity, 0.0, 255.0, 95.0, 255.0) as u8
            };

            let color = projection_result.screen_triangle.color;

            let use_dithering = true;
            if use_dithering {
                let r: i16 = (correct_color(color.r as f64) as i16) + r_dithering_errors[y][x];
                let g: i16 = (correct_color(color.g as f64) as i16) + g_dithering_errors[y][x];
                let b: i16 = (correct_color(color.b as f64) as i16) + b_dithering_errors[y][x];

                let calculate_error = |v: i16| -> i16 {
                    let a = if v < 95 { return 0; } else { (1 + (v - 95) / 40).min(5)};
                    let c = (a - 1) * 40 + 95;    
                    v - c
                };
                

                /* Burkess Dithering
                            X   8   4 
                    2   4   8   4   2
                */

                let re = calculate_error(r);
                let ge = calculate_error(g);
                let be = calculate_error(b);

                let ra = r.min(255).max(0) as u8;
                let ga = g.min(255).max(0) as u8;
                let ba = b.min(255).max(0) as u8;

                let in_bounds = |x: i16, y: i16| -> bool {
                    x >= 0 && y >= 0 && x < ((SCREEN_WIDTH) as i16) && y < ((SCREEN_HEIGHT) as i16)
                };

                // Diffuse errors to neighboring pixels according to the dithering pattern
                let cx = x as i16;
                let cy = y as i16;
                if in_bounds(cx + 1, cy) {
                    r_dithering_errors[y][x + 1] += re / 4;
                    g_dithering_errors[y][x + 1] += ge / 4;
                    b_dithering_errors[y][x + 1] += be / 4;
                }
                if in_bounds(cx + 2, cy) {
                    r_dithering_errors[y][x + 2] += re / 8;
                    g_dithering_errors[y][x + 2] += ge / 8;
                    b_dithering_errors[y][x + 2] += be / 8;
                }
                if in_bounds(cx - 2, cy + 1) {
                    r_dithering_errors[y + 1][x - 2] += re / 16;
                    g_dithering_errors[y + 1][x - 2] += ge / 16;
                    b_dithering_errors[y + 1][x - 2] += be / 16;
                }
                if in_bounds(cx - 1, cy + 1) {
                    r_dithering_errors[y + 1][x - 1] += re / 8;
                    g_dithering_errors[y + 1][x - 1] += ge / 8;
                    b_dithering_errors[y + 1][x - 1] += be / 8;
                }
                if in_bounds(cx, cy + 1) {
                    r_dithering_errors[y + 1][x] += re / 4;
                    g_dithering_errors[y + 1][x] += ge / 4;
                    b_dithering_errors[y + 1][x] += be / 4;
                }
                if in_bounds(cx + 1, cy + 1) {
                    r_dithering_errors[y + 1][x + 1] += re / 8;
                    g_dithering_errors[y + 1][x + 1] += ge / 8;
                    b_dithering_errors[y + 1][x + 1] += be / 8;
                }
                if in_bounds(cx + 2, cy + 1) {
                    r_dithering_errors[y + 1][x + 2] += re / 16;
                    g_dithering_errors[y + 1][x + 2] += ge / 16;
                    b_dithering_errors[y + 1][x + 2] += be / 16;
                }

                screen_buffer[y][x] = graphics::rgb_to_ansi256(ra, ga, ba);
            } else {
                
                // Uniform noise dithering experiment
                let mut rng = rand::thread_rng();
                //let a = (rng.gen::<f64>() * 8. - 4.) as i16;
                //let b = (rng.gen::<f64>() * 8. - 4.) as i16;
                //let c = (rng.gen::<f64>() * 8. - 4.) as i16;
                /*
                let r = (correct_color(color.r as f64) as i16) + a;
                let g = (correct_color(color.g as f64) as i16) + b;
                let b = (correct_color(color.b as f64) as i16) + c;
                screen_buffer[y][x] = graphics::rgb_to_ansi256(r.min(255).max(0) as u8, g.min(255).max(0) as u8, b.min(255).max(0) as u8);
                */
/*
                let r = color.r as u8;
                let g = color.g as u8;
                let b = color.b as u8;
                */
                let r = correct_color(color.r as f64);
                let g = correct_color(color.g as f64);
                let b = correct_color(color.b as f64);
                
                screen_buffer[y][x] = graphics::rgb_to_ansi256(r, g, b);
            }
        }
    }

    cached_projection_results.clone()
}
