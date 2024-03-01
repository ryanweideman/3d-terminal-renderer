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
use nalgebra::Point3;

use camera::Camera;

use terminal::Terminal;

fn main() -> io::Result<()> {
    let config_path = include_str!("../config.json");
    let config = config::load_config(config_path);

    let mut camera = camera::StaticPerspectiveCameraBuilder::new()
        .origin(Point3::new(0.0, 0.7, 3.0))
        .yaw(-std::f64::consts::PI / 2.0)
        .pitch(-0.4)
        .build();

    let model_loader = model_loader::ModelLoader::new(&include_dir!("models/"));
    let (mut entities, lights) =
        world_loader::load_world(include_str!("../demo.json"), &model_loader);

    let mut start_time = time::Instant::now();
    let delay_duration = time::Duration::from_secs_f64(1.0 / config.target_fps);

    let mut terminal = Terminal::new(
        config.background_color,
        config.aspect_ratio,
        config.use_true_color,
    );
    terminal.init()?;

    loop {
        if start_time.elapsed() < delay_duration {
            std::thread::sleep(delay_duration - start_time.elapsed());
        }
        let delta_time = start_time.elapsed().as_secs_f64();
        start_time = time::Instant::now();

        // Update terminal, camera, and scene entities
        terminal.update()?;
        if terminal.is_ctrl_c_pressed() {
            break;
        }
        //camera.update(&terminal.get_key_presses(), delta_time);
        camera.update(delta_time);
        for entity in &mut entities {
            entity.update(delta_time);
        }

        // Renders the scene to the screen_buffer
        let screen_buffer = terminal.get_mutable_screen_buffer_reference();
        renderer::render_scene(
            screen_buffer,
            &entities,
            &lights,
            &camera,
            config.background_color,
        );

        if config.use_dithering && !config.use_true_color {
            renderer::apply_ansi_256_dithering(screen_buffer);
        }

        terminal.output_screen_buffer()?;
    }

    terminal.destroy()?;
    Ok(())
}
