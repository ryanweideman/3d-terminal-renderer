use nalgebra::{Matrix4, Point2, Point3};

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::camera::Camera;
use crate::geometry;
use crate::graphics;
use crate::math;
use crate::world_objects::Light;

pub fn render_geometry(
    screen_buffer: &mut [[u16; SCREEN_WIDTH]; SCREEN_HEIGHT],
    geometry: &Vec<geometry::Triangle3>,
    world_lights: &Vec<Light>,
    camera: &Camera,
    ansi_background_color: u16,
) -> Vec<geometry::ProjectionResult> {
    let view_projection_matrix: Matrix4<f64> = camera.get_view_projection_matrix();
    let view_projection_matrix_inverse = view_projection_matrix.try_inverse().unwrap();

    let mut z_buffer: [[f64; SCREEN_WIDTH]; SCREEN_HEIGHT] =
        [[f64::MAX; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut projection_buffer: [[usize; SCREEN_WIDTH]; SCREEN_HEIGHT] =
        [[usize::MAX; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut cached_projection_results = Vec::with_capacity(geometry.len());

    for triangle in geometry {
        // world cords -> camera coords -> ndc -> screen coords
        let projection_results = geometry::project_triangle(
            triangle,
            &view_projection_matrix,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        );

        for projection_result in &projection_results {
            cached_projection_results.push(*projection_result);
            let projection_result_index = cached_projection_results.len() - 1;

            let (x_min, y_min, x_max, y_max) = projection_result
                .screen_bounding_box
                .get_screen_constrained_bounds(SCREEN_WIDTH, SCREEN_HEIGHT);

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

            let p_ndc =
                geometry::screen_to_ndc(&pixel, SCREEN_WIDTH, SCREEN_HEIGHT).to_homogeneous();
            let point_world_space_homogeneous = view_projection_matrix_inverse * p_ndc;
            let point_world_space =
                point_world_space_homogeneous.xyz() / point_world_space_homogeneous.w;

            let light_intensity = world_lights
                .iter()
                .map(|light| match light {
                    Light::PointLight(point_light) => {
                        let origin = point_light.get_origin();
                        let light_norm = (origin - point_world_space).coords.normalize();
                        let diffuse_intensity = light_norm.dot(&projection_result.normal).max(0.0);

                        let distance = (origin - point_world_space).coords.magnitude();
                        let attenuation = 1.0
                            / (1.0
                                + point_light.get_linear_attenuation() * distance
                                + point_light.get_quadratic_attenuation() * distance * distance);

                        diffuse_intensity * attenuation * light.get_intensity()
                    }
                    Light::AmbientLight(_) => light.get_intensity(),
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
                    let a = if v < 95 {
                        return 0;
                    } else {
                        (1 + (v - 95) / 40).min(5)
                    };
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

                let mut diffuse_error =
                    |x: i16, y: i16, r_error: i16, g_error: i16, b_error: i16, factor: i16| {
                        if x >= 0
                            && y >= 0
                            && x < ((SCREEN_WIDTH) as i16)
                            && y < ((SCREEN_HEIGHT) as i16)
                        {
                            r_dithering_errors[y as usize][x as usize] += r_error / factor;
                            g_dithering_errors[y as usize][x as usize] += g_error / factor;
                            b_dithering_errors[y as usize][x as usize] += b_error / factor;
                        }
                    };

                // Diffuse errors to neighboring pixels according to the dithering pattern
                let cx = x as i16;
                let cy = y as i16;
                diffuse_error(cx + 1, cy, re, ge, be, 4);
                diffuse_error(cx + 2, cy, re, ge, be, 8);
                diffuse_error(cx - 2, cy + 1, re, ge, be, 16);
                diffuse_error(cx - 1, cy + 1, re, ge, be, 8);
                diffuse_error(cx, cy + 1, re, ge, be, 4);
                diffuse_error(cx + 1, cy + 1, re, ge, be, 8);
                diffuse_error(cx + 2, cy + 1, re, ge, be, 16);

                if color.r == color.g && color.g == color.b {
                    let u = ((ra as u16 + ga as u16 + ba as u16) / 3) as u8;
                    screen_buffer[y][x] = graphics::rgb_to_ansi256(u, u, u);
                } else {
                    screen_buffer[y][x] = graphics::rgb_to_ansi256(ra, ga, ba);
                }
            } else {
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
