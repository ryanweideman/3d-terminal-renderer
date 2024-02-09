mod constants;
mod geometry;
mod graphics;
mod math;
mod model_loader;
mod renderer;
mod world_loader;

use nalgebra::{Vector3, Matrix3x4, Point3, Rotation3, Unit};
use std::{time};

use constants::{SCREEN_WIDTH, SCREEN_HEIGHT, TARGET_FPS};
use geometry::{Cube};

fn main() {
    ctrlc::set_handler(move || {
        graphics::move_cursor(0, 0, SCREEN_HEIGHT, 5);
        graphics::show_cursor();
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    graphics::clear_screen();
    graphics::hide_cursor();

    let model_loader = model_loader::ModelLoader::new("models/");
    let cube_model = model_loader.get_model("cube.json").unwrap();
    let square_model = model_loader.get_model("square.json").unwrap();
    world_loader::load_world("world.json");

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

    let mut screen_buffer : [[u16; SCREEN_WIDTH] ; SCREEN_HEIGHT] 
        = [[ansi_background_color ; SCREEN_WIDTH] ; SCREEN_HEIGHT]; 

    loop {
        if start_time.elapsed() < delay_duration {
            continue;
        }

        screen_buffer = [[ansi_background_color ; SCREEN_WIDTH] ; SCREEN_HEIGHT]; 

        start_time = time::Instant::now();
        graphics::reset_cursor();

        theta -= 0.05;

        // Define the rotation using Rotation3
        let cube_rotation_axis = Unit::new_normalize(Vector3::new(1.7, 3.0, 0.0)); // Rotate around the Y axis
        let cube_rotation = Rotation3::from_axis_angle(&cube_rotation_axis, theta);
        let cube = Cube {
            origin : Point3::new(0.0, 0.0, -2.5),
            rotation : cube_rotation
        };
        let cube_geometry = geometry::transform_model(&cube.origin, &cube.rotation, &cube_model);

        let square_rotation = Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0);
        let square_geometry = geometry::transform_model(&Point3::new(-2.0, 1.0, -4.0), &square_rotation, &square_model);
        let geometry = [cube_geometry, square_geometry].concat();
        //let geometry = cube_geometry;

        renderer::render_geometry(
            &mut screen_buffer,
            &geometry, 
            &projection_matrix, 
            &projection_matrix_inverse, 
            &camera_transform, 
            ansi_background_color);

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
