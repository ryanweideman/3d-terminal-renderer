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

use std::io;
use std::time;

use include_dir::include_dir;

use terminal::Terminal;

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

    let mut terminal = Terminal::new(
        config.background_color,
        config.aspect_ratio,
        config.use_true_color,
    );
    let mut stdout = io::stdout();
    terminal.init(&mut stdout)?;

    loop {
        if start_time.elapsed() < delay_duration {
            continue;
        }
        let current_time = time::Instant::now();
        let delta_time = current_time.duration_since(start_time).as_secs_f64();
        start_time = current_time;

        keyboard.update()?;
        if keyboard.is_ctrl_c_pressed() {
            break;
        }
        camera.update(&keyboard, delta_time);
        for entity in &mut entities {
            entity.update(delta_time);
        }

        let mut screen_buffer = terminal.get_mutable_screen_buffer(&mut stdout);
        renderer::render_scene(
            &mut screen_buffer,
            &entities,
            &lights,
            &camera,
            config.background_color,
        );

        if config.use_dithering && !config.use_true_color {
            renderer::apply_ansi_256_dithering(&mut screen_buffer);
        }

        terminal.output_screen_buffer(&mut stdout)?;
    }

    terminal.destroy(&mut stdout)?;
    Ok(())
}
