mod constants;
mod geometry;
mod graphics;
mod utils;

use constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use geometry::{Triangle3, PointLight, Cube};
use nalgebra::{Vector3, Matrix4, Matrix3x4, Perspective3, Point2, Point3, Point4, Rotation3, Unit};
use std::{time};

fn main() {
    ctrlc::set_handler(move || {
        print!("\x1b[{};{}H", SCREEN_HEIGHT + 5, 0);
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    println!("\x1b[H\x1b[J");
    graphics::clear_screen();
    let mut start_time = time::Instant::now();
    let delay_duration = time::Duration::from_millis(100);
    let ansi_background_color = graphics::rgb_to_ansi256(100, 100, 100);

    // Assume camera is fixed at origin, for now
    let camera_transform : Matrix3x4<f32> = Matrix3x4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0
    );

    /* 
        Perspective3 produces a symmetric frustum identical to that used by OpenGL
        Perspective matrix :

        |  f / aspect  0                              0                                 0  |
        |  0           f                              0                                 0  |
        |  0           0   -(far + near) / (far - near)    -2 * far * near / (far - near)  |
        |  0           0                             -1                                 0  |

        where f = 1 / tan(fov / 2)
    */

    let projection_matrix : Matrix4<f32> = Perspective3::new(
        constants::ASPECT_RATIO, 
        constants::FOV, 
        constants::NEAR_PLANE,
        constants::FAR_PLANE)
        .to_homogeneous();
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

        theta -= 0.15;
        //theta = std::f32::consts::PI / 4.0 + 0.1;
        //theta = 0.5;

        // Define the rotation using Rotation3
        let rotation_axis = Unit::new_normalize(Vector3::new(1.7, 3.0, 0.0)); // Rotate around the Y axis
        //let rotation_axis = Vector3::y_axis(); // Rotate around the Y axis

        let rotation3 = Rotation3::from_axis_angle(&rotation_axis, theta);
        let cube = Cube {
            //origin : Point3::new(0.0, 0.0, -3.0 + f32::sin(phi)),
            origin : Point3::new(0.0, 0.0, -3.0),
            rotation : rotation3
        };

        let geometry : [Triangle3; 12] = geometry::get_cube_geometry(&cube);

        for triangle in &geometry {
            // world cords -> camera coords -> ndc -> screen coords

            // Get world cordaintes 
            let world_v0 = triangle.geometry[0];
            let world_v1 = triangle.geometry[1];
            let world_v2 = triangle.geometry[2];
        
            // Transform world coordinates to camera coordinates
            let camera_v0 : Point3<f32> = (camera_transform * world_v0.to_homogeneous()).into();
            let camera_v1 : Point3<f32> = (camera_transform * world_v1.to_homogeneous()).into();
            let camera_v2 : Point3<f32> = (camera_transform * world_v2.to_homogeneous()).into();
        
            // Transform camera coordinates to clip space coordinates
            let clip_space_v0 : Point4<f32> = (projection_matrix * camera_v0.to_homogeneous()).into();
            let clip_space_v1 : Point4<f32> = (projection_matrix * camera_v1.to_homogeneous()).into();
            let clip_space_v2 : Point4<f32> = (projection_matrix * camera_v2.to_homogeneous()).into();

            // Transform from clip space coordinates to normlized device coordinates
            let ndc_v0 : Point3<f32> = clip_space_v0.xyz() / clip_space_v0.w;
            let ndc_v1 : Point3<f32> = clip_space_v1.xyz() / clip_space_v1.w;
            let ndc_v2 : Point3<f32> = clip_space_v2.xyz() / clip_space_v2.w;

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

                    
                    let mut dot = light_norm.dot(&triangle_norm).max(0.0);

                   // dot = scale_range(dot, 0.0, 1.0, 0.5, 1.0);
                    dot = utils::round_up_to_nearest_increment(dot, 0.2);

                    let mut r = triangle.color.r;
                    if r != 0 {
                        r = utils::scale_range((r as f32) * dot, 0.0, 255.0, 95.0, 255.0) as u8;
                    }
                    let mut g = triangle.color.g;
                    if g != 0 {
                        g = utils::scale_range((g as f32) * dot, 0.0, 255.0, 95.0, 255.0) as u8;
                    }
                    let mut b = triangle.color.b;
                    if b != 0 {
                        b = utils::scale_range((b as f32) * dot, 0.0, 255.0, 95.0, 255.0) as u8;
                    }

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
