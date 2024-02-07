mod constants;
mod geometry;
mod graphics;
mod utils;

use nalgebra::{Vector3, Matrix3x4, Point2, Point3, Rotation3, Unit};
use std::{time};

use constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
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
    let delay_duration = time::Duration::from_millis(34);
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
        //theta = std::f32::consts::PI / 4.0 + 0.1;
        //theta = 0.5;

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
            let (camera_v0, camera_v1, camera_v2) = geometry::transform_triangle_to_camera_coords(&triangle, &camera_transform);

            let clip_space_v0 = geometry::camera_coordinates_to_clip_space(&camera_v0, &projection_matrix);
            let clip_space_v1 = geometry::camera_coordinates_to_clip_space(&camera_v1, &projection_matrix);
            let clip_space_v2 = geometry::camera_coordinates_to_clip_space(&camera_v2, &projection_matrix);

            // Transform from clip space coordinates to normlized device coordinates
            let ndc_v0 = geometry::clips_space_to_ndc(&clip_space_v0);
            let ndc_v1 = geometry::clips_space_to_ndc(&clip_space_v1);
            let ndc_v2 = geometry::clips_space_to_ndc(&clip_space_v2);

            // Transform from normalized device coordinates to screen coordinates
            let screen_v0 = geometry::ndc_to_screen(&ndc_v0);
            let screen_v1 = geometry::ndc_to_screen(&ndc_v1);
            let screen_v2 = geometry::ndc_to_screen(&ndc_v2);

            let projected_triangle = Triangle3 {
                geometry: [
                    screen_v0,
                    screen_v1,
                    screen_v2
                ],
                color: triangle.color
            };

            // Get bounding box of the projected triangle
            let (minx, miny, maxx, maxy) = graphics::calculate_bounding_box(&projected_triangle);

            // Rasterize
            for y in miny..maxy {
                for x in minx..maxx {
                    let px = (x as f32) + 0.5;
                    let py = (y as f32) + 0.5;
                    let pixel = Point2::new(px, py);

                    if !graphics::is_point_in_triangle(&pixel, &projected_triangle) {
                        continue;
                    }

                    let (z, _w) = graphics::interpolate_attributes_at_pixel(
                        &pixel, &screen_v0, &screen_v1, &screen_v2, 
                        &clip_space_v0, &clip_space_v1, &clip_space_v2, 
                        &ndc_v0, &ndc_v1, &ndc_v2);

                    // pixel in this triangle is behind another triangle
                    if z >= z_buffer[y][x] {
                        continue;
                    }

                    let p_ndc = geometry::screen_to_ndc(&Point3::new(px, py, z)).to_homogeneous();
                    let point_camera_space_homogeneous = projection_matrix_inverse * p_ndc;
                    let point_camera_space : Point3<f32> = (point_camera_space_homogeneous.xyz() / point_camera_space_homogeneous.w).into();
                    
                    // calculate normal
                    let triangle_norm = (camera_v1 - camera_v0).cross(&(camera_v2 - camera_v0)).normalize();
                    // Transform the point light into camera coordinates
                    let camera_point_light : Point3<f32> = (camera_transform * point_light.origin.to_homogeneous()).into();
                    let light_norm = (camera_point_light - point_camera_space).normalize();

                    let dot = utils::round_up_to_nearest_increment(
                        light_norm.dot(&triangle_norm).max(0.0), 
                        0.2);

                    let correct_color = |input: u8| -> u8 {
                        if input == 0 {
                            return input;
                        } 
                        utils::scale_range((input as f32) * dot, 0.0, 255.0, 95.0, 255.0) as u8
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

        let n = (time::Instant::now() - start_time).as_nanos() as f32;
        println!("total time elapsed ms: {:.2}", n / 1000000.0);
        println!("  draw time elapsed ms: {:.2}\n", draw_time_elapsed / 1000000.0);
    }
} 
