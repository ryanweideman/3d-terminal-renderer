mod buffer;
mod camera;
mod config;
mod geometry;
mod keyboard;
mod math;
mod model_loader;
mod renderer;
mod terminal;
mod world_loader;
mod world_objects;

use include_dir::include_dir;
use std::io;
use std::time;

use buffer::Buffer;

fn main() -> io::Result<()> {
    let config_path = include_str!("../config.json");
    let config = config::load_config(config_path);
    let mut camera = camera::Camera::new(&config);
    let mut keyboard = keyboard::Keyboard::new();

    let model_loader = model_loader::ModelLoader::new(&include_dir!("models/"));
    let (mut entities, lights) =
        world_loader::load_world(include_str!("../demo.json"), &model_loader);

    let mut start_time = time::Instant::now();
    let delay_duration = time::Duration::from_millis((1000.0 / config.target_fps) as u64);

    let mut stdout = io::stdout();
    terminal::init(&mut stdout)?;

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
        let mut screen_buffer = Buffer::<[u8; 3]>::new(
            config.background_color,
            config.screen_width,
            config.screen_height,
        );

        entities
            .iter_mut()
            .for_each(|entity| entity.update(delta_time));
        let world_geometry = entities
            .iter()
            .flat_map(|entity| geometry::transform_entity_model(entity))
            .collect();

        let _projection_results = renderer::render_geometry(
            &mut screen_buffer,
            &world_geometry,
            &lights,
            &camera,
            config.background_color,
        );
        let _processing_time_elapsed = start_time.elapsed();

        terminal::output_screen_buffer(&mut stdout, &screen_buffer, config.use_true_color)?;
        terminal::flush(&mut stdout)?;

        keyboard.clear_all_keys();
        let _total_time_elapsed = start_time.elapsed();
    }

    terminal::destroy(&mut stdout)?;
    Ok(())
}
