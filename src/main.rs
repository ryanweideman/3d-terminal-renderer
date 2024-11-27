use std::io;
use std::time;

use include_dir::include_dir;
use nalgebra::Point3;

use lib_terminal_renderer::camera::ControllablePerspectiveCameraBuilder;
use lib_terminal_renderer::model_loaders::JsonModelLoader;
use lib_terminal_renderer::renderer;
use lib_terminal_renderer::scene_loader;
use lib_terminal_renderer::terminal::Terminal;

const BACKGROUND_COLOR: [u8; 3] = [100, 100, 100];
const TARGET_FPS: usize = 20;
const ASPECT_RATIO: f64 = 1.6;
const USE_TRUE_COLOR: bool = true;
const USE_DITHERING: bool = false;

const SCENE_FILE: &str = include_str!("../scenes/demo.json");
const MODEL_DIR: include_dir::Dir = include_dir!("models/");

fn main() -> io::Result<()> {
    let mut camera = ControllablePerspectiveCameraBuilder::new()
        .origin(Point3::new(0.0, 0.7, 3.0))
        .yaw(-std::f64::consts::PI / 2.0)
        .pitch(-0.4)
        .aspect_ratio(ASPECT_RATIO)
        .build();

    let model_loader = JsonModelLoader::new(&MODEL_DIR);
    let (mut entities, lights) =
        scene_loader::load_scene(SCENE_FILE, &model_loader);

    let mut start_time = time::Instant::now();
    let delay_duration = time::Duration::from_secs_f64(1.0 / TARGET_FPS as f64);

    let mut terminal = Terminal::new(BACKGROUND_COLOR, ASPECT_RATIO, USE_TRUE_COLOR);
    terminal.init()?;

    loop {
        // Sleep until the next loop time if needed
        std::thread::sleep(delay_duration.saturating_sub(start_time.elapsed()));

        let delta_time = start_time.elapsed().as_secs_f64();
        start_time = time::Instant::now();

        // Update terminal, camera, and scene entities
        terminal.update()?;
        if terminal.is_ctrl_c_pressed() {
            break;
        }
        camera.update(delta_time, &terminal.get_key_presses());
        for entity in &mut entities {
            entity.update(delta_time);
        }

        // Renders the scene to the screen_buffer
        let screen_buffer = terminal.get_mutable_screen_buffer_reference();
        renderer::render_scene(screen_buffer, &entities, &lights, &camera, BACKGROUND_COLOR);

        if USE_DITHERING && !USE_TRUE_COLOR {
            renderer::apply_ansi_256_dithering(screen_buffer);
        }

        terminal.output_screen_buffer()?;
    }

    terminal.destroy()?;
    Ok(())
}
