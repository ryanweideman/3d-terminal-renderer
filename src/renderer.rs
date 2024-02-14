use nalgebra::{Matrix4, Point2, Point3};

use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};

use crate::graphics;
use crate::math;
use crate::geometry;
use crate::world_objects;

pub fn render_geometry(
        screen_buffer: &mut [[u16; SCREEN_WIDTH] ; SCREEN_HEIGHT],
        geometry: &Vec<geometry::Triangle3>, 
        projection_matrix: &Matrix4<f32>,        
        projection_matrix_inverse: &Matrix4<f32>, 
        camera_transform: &Matrix4<f32>, 
        ansi_background_color: u16) -> Vec<geometry::ProjectionResult> {

    let point_light = world_objects::PointLight {
        origin: Point3::new(2.0, -2.0, 3.0)
    };

    let camera_point_light = geometry::transform_world_vertice_to_camera_coords(&point_light.origin, camera_transform);

    let mut z_buffer : [[f32; SCREEN_WIDTH] ; SCREEN_HEIGHT] 
        = [[f32::MAX ; SCREEN_WIDTH] ; SCREEN_HEIGHT]; 
    let mut projection_buffer : [[usize; SCREEN_WIDTH] ; SCREEN_HEIGHT] 
        = [[usize::MAX ; SCREEN_WIDTH] ; SCREEN_HEIGHT]; 
    let mut cached_projection_results = Vec::with_capacity(geometry.len());

    for triangle in geometry {
        // world cords -> camera coords -> ndc -> screen coords
        let projection_results = geometry::project_triangle(triangle, &projection_matrix, &camera_transform);

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
                    let px = (x as f32) + 0.5;
                    let py = (y as f32) + 0.5;
                    let pixel = Point2::new(px, py);

                    if !geometry::is_point_in_triangle(&pixel, &projection_result.screen_triangle) {
                        continue;
                    }

                    let (z, _w) = graphics::interpolate_attributes_at_pixel(
                        &pixel, &projection_result);

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

            let pixel = Point3::new(
                (x as f32) + 0.5, 
                (y as f32) + 0.5, 
                z_buffer[y][x]);

            let p_ndc = geometry::screen_to_ndc(&pixel).to_homogeneous();
            let point_camera_space_homogeneous = projection_matrix_inverse * p_ndc;
            let point_camera_space = point_camera_space_homogeneous.xyz() / point_camera_space_homogeneous.w;
            let light_norm = (camera_point_light - point_camera_space).coords.normalize();

            let dot = math::round_up_to_nearest_increment(
                light_norm.dot(&projection_result.normal).max(0.0), 
                0.2);

            let correct_color = |input: u8| -> u8 {
                if input == 0 {
                    return input;
                } 
                math::scale_range((input as f32) * dot, 0.0, 255.0, 95.0, 255.0) as u8
            };

            let color = projection_result.screen_triangle.color;
            let r = correct_color(color.r);
            let g = correct_color(color.g);
            let b = correct_color(color.b);

            screen_buffer[y][x] = graphics::rgb_to_ansi256(r, g, b);
        }
    }

    cached_projection_results.clone()
}