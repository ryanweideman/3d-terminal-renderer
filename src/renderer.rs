use nalgebra::{Matrix4, Point2, Point3};

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::geometry;
use crate::graphics;
use crate::math;
use crate::world_objects::{Light, PointLight, AmbientLight};

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

                            let a = 0.5;
                            let b = 0.3;

                            let distance = (origin - point_camera_space).coords.magnitude();
                            let attenuation = 1.0 / (1.0 + a * distance + b * distance * distance);

                            diffuse_intensity * attenuation * light.get_intensity()
                        },
                        Light::AmbientLight(ambient_light) => {
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

            let r = correct_color(color.r as f64);
            let g = correct_color(color.g as f64);
            let b = correct_color(color.b as f64);

            //let r = color.r as u8;
            //let g = color.g as u8;
            //let b = color.b as u8;

            screen_buffer[y][x] = graphics::rgb_to_ansi256(r, g, b);
        }
    }

    cached_projection_results.clone()
}
