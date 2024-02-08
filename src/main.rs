mod constants;
mod geometry;
mod graphics;
mod math;

use nalgebra::{Vector3, Matrix3x4, Point2, Point3, Rotation3, Unit};
use std::{time};

use constants::{SCREEN_WIDTH, SCREEN_HEIGHT, TARGET_FPS};
use geometry::{Triangle3, PointLight, Cube};

fn main() {
    ctrlc::set_handler(move || {
        graphics::move_cursor(0, 0, SCREEN_HEIGHT, 5);
        graphics::show_cursor();
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    graphics::clear_screen();
    graphics::hide_cursor();

    let mut start_time = time::Instant::now();
    let delay_duration = time::Duration::from_millis((1000.0 / TARGET_FPS) as u64);
    let ansi_background_color = graphics::rgb_to_ansi256(100, 100, 100);

    // Assume camera is fixed at origin, for now
    let camera_transform = Matrix3x4::<f32>::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0
    );

    let projection_matrix = geometry::get_projection_matrix();
    let projection_matrix_inverse = projection_matrix.try_inverse().unwrap();

    let mut theta : f32 = 0.0;

    let point_light = PointLight {
        origin: Point3::new(2.0, -2.0, 3.0)
    };

    loop {
        if start_time.elapsed() < delay_duration {
            continue;
        }

        start_time = time::Instant::now();
        graphics::reset_cursor();

        let mut screen_buffer : [[u16; SCREEN_WIDTH] ; SCREEN_HEIGHT] 
            = [[ansi_background_color ; SCREEN_WIDTH] ; SCREEN_HEIGHT]; 
        let mut z_buffer : [[f32; SCREEN_WIDTH] ; SCREEN_HEIGHT] 
            = [[f32::MAX ; SCREEN_WIDTH] ; SCREEN_HEIGHT]; 

        theta -= 0.02;

        // Define the rotation using Rotation3
        let rotation_axis = Unit::new_normalize(Vector3::new(1.7, 3.0, 0.0)); // Rotate around the Y axis
        //let rotation_axis = Vector3::y_axis(); // Rotate around the Y axis

        let rotation3 = Rotation3::from_axis_angle(&rotation_axis, theta);
        let cube = Cube {
            origin : Point3::new(0.0, 0.0, -2.5),
            rotation : rotation3
        };

        let geometry : [Triangle3; 12] = geometry::get_cube_geometry(&cube);

        for triangle in &geometry {
            // world cords -> camera coords -> ndc -> screen coords
            let maybe_projection_result = geometry::project_triangle(triangle, &projection_matrix, &camera_transform);
            if maybe_projection_result.is_none() {
                continue;
            }
            let projection_result = maybe_projection_result.unwrap();
            let bounding_box = &projection_result.screen_bounding_box;

            // Rasterize
            for y in bounding_box.y_min..bounding_box.y_max {
                for x in bounding_box.x_min..bounding_box.x_max {
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
                    
                    let p_ndc = geometry::screen_to_ndc(&Point3::new(px, py, z)).to_homogeneous();
                    let point_camera_space_homogeneous = projection_matrix_inverse * p_ndc;
                    let point_camera_space : Point3<f32> = (point_camera_space_homogeneous.xyz() / point_camera_space_homogeneous.w).into();
                    
                    // Transform the point light into camera coordinates
                    let camera_point_light : Point3<f32> = (camera_transform * point_light.origin.to_homogeneous()).into();
                    let light_norm = (camera_point_light - point_camera_space).normalize();

                    let dot = math::round_up_to_nearest_increment(
                        light_norm.dot(&projection_result.normal).max(0.0), 
                        0.2);

                    let correct_color = |input: u8| -> u8 {
                        if input == 0 {
                            return input;
                        } 
                        math::scale_range((input as f32) * dot, 0.0, 255.0, 95.0, 255.0) as u8
                    };

                    let r = correct_color(triangle.color.r);
                    let g = correct_color(triangle.color.g);
                    let b = correct_color(triangle.color.b);

                    screen_buffer[y][x] = graphics::rgb_to_ansi256(r, g, b);
                    z_buffer[y][x] = z;
                }
            }
        }

        let draw_start = time::Instant::now();
        graphics::output_screen_buffer(&screen_buffer);
        let draw_end = time::Instant::now();
        let draw_time_elapsed = (draw_end - draw_start).as_nanos() as f32; 

        let loop_end = time::Instant::now();
        let n = (loop_end - start_time).as_nanos() as f32;
        println!("total time elapsed ms: {:.2}", n / 1000000.0);
        println!("  draw time elapsed ms: {:.2}\n", draw_time_elapsed / 1000000.0);
        println!("  processing time elapsed ms: {:.2}\n", ((loop_end - start_time - (draw_end - draw_start)).as_nanos() as f32) / 1000000.0);

    }
} 
