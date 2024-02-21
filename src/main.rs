mod camera;
mod constants;
mod geometry;
mod graphics;
mod keyboard;
mod math;
mod model_loader;
mod renderer;
mod world_loader;
mod world_objects;

use crossterm::{
    cursor::{Hide, Show},
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use nalgebra::Point3;
use std::io;
use std::io::Write;
use std::time;

use constants::{SCREEN_HEIGHT, SCREEN_WIDTH, TARGET_FPS};

fn main() {
    enable_raw_mode().expect("Could not enter terminal raw mode");
    let mut stdout = io::stdout();
    stdout
        .execute(EnterAlternateScreen)
        .expect("Could not enter terminal alternative mode");
    queue!(stdout, Hide).unwrap();

    let mut camera = camera::Camera::new(Point3::new(0.0, 0.01, 3.0));
    let mut keyboard = keyboard::Keyboard::new();

    let model_loader = model_loader::ModelLoader::new("models/");
    let (mut entities, lights) = world_loader::load_world("demo.json", &model_loader);

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
        let current_time = time::Instant::now();
        let delta_time = current_time.duration_since(start_time).as_secs_f64();
        start_time = current_time;

        camera.update(&keyboard, delta_time);
        let mut screen_buffer = [[ansi_background_color; SCREEN_WIDTH]; SCREEN_HEIGHT];

        let camera_transform = camera.get_transform();

        entities.iter_mut().for_each(|entity| entity.update(delta_time));
        let world_geometry = entities
            .iter()
            .map(|entity| geometry::transform_entity_model(&entity))
            .flat_map(|v| v)
            .collect();

        let _projection_results = renderer::render_geometry(
            &mut screen_buffer,
            &world_geometry,
            &lights,
            &projection_matrix,
            &projection_matrix_inverse,
            &camera_transform,
            ansi_background_color,
        );
        let _processing_time_elapsed = start_time.elapsed();

        graphics::output_screen_buffer(&mut stdout, &screen_buffer);
        let _total_time_elapsed = start_time.elapsed();

        /*graphics::print_debug_info(
            &mut stdout,
            &total_time_elapsed,
            &processing_time_elapsed,
            &projection_results,
        );*/
        stdout.flush().unwrap();
        keyboard.clear_all_keys();
    }

    queue!(stdout, Show).unwrap();
    disable_raw_mode().unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();
}
