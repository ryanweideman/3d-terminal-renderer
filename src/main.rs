mod camera;
mod constants;
mod geometry;
mod graphics;
mod math;
mod model_loader;
mod keyboard;
mod renderer;
mod world_loader;
mod world_objects;

use crossterm::{
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    }, 
    queue, 
    ExecutableCommand, 
    cursor::{
        Show,
        Hide
    }
};

use nalgebra::{Point3};
use std::{time};
use std::io::{Write};
use std::{io};

use constants::{SCREEN_WIDTH, SCREEN_HEIGHT, TARGET_FPS};

fn main() {
    enable_raw_mode().expect("Could not enter terminal raw mode");
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen).expect("Could not enter terminal alternative mode");
    queue!(stdout, Hide).unwrap();

    let mut camera = camera::Camera::new(Point3::new(0.0, 0.01, 3.0));
    let mut keyboard = keyboard::Keyboard::new();

    let model_loader = model_loader::ModelLoader::new("models/");
    let (mut entities, lights) = world_loader::load_world("world.json", &model_loader);

    let mut start_time = time::Instant::now();
    let delay_duration = time::Duration::from_millis((1000.0 / TARGET_FPS) as u64);
    let ansi_background_color = graphics::rgb_to_ansi256(100, 100, 100);

    let projection_matrix = geometry::get_projection_matrix();
    let projection_matrix_inverse = projection_matrix.try_inverse().unwrap();

    loop {
        keyboard.update();
        if keyboard.is_ctrl_c_pressed() {
            break;
        }

        if start_time.elapsed() < delay_duration {
            continue;
        }
        start_time = time::Instant::now();

        camera.update(&keyboard);
        let mut screen_buffer = [[ansi_background_color ; SCREEN_WIDTH] ; SCREEN_HEIGHT]; 

        let camera_transform = camera.get_transform();

        entities.iter_mut()
            .for_each(|entity| entity.update(0.0));
        let world_geometry = entities.iter()
            .map(|entity| geometry::transform_entity_model(&entity))
            .flat_map(|v| v)
            .collect();

        let projection_results = renderer::render_geometry(
            &mut screen_buffer,
            &world_geometry, 
            &lights,
            &projection_matrix, 
            &projection_matrix_inverse, 
            &camera_transform, 
            ansi_background_color);
        let processing_time_elapsed = start_time.elapsed();

        graphics::output_screen_buffer(&mut stdout, &screen_buffer);
        let total_time_elapsed = start_time.elapsed();

        graphics::print_debug_info(&mut stdout, &total_time_elapsed, &processing_time_elapsed, &projection_results);
        stdout.flush().unwrap();
        keyboard.clear_all_keys();
    }

    queue!(stdout, Show).unwrap();
    disable_raw_mode().unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();
} 
