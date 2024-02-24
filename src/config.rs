use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub target_fps: f64,
    pub screen_width: usize,
    pub screen_height: usize,
    pub background_color: [u8; 3],
    pub use_dithering: bool,
    pub use_true_color: bool,
    pub near_plane: f64,
    pub far_plane: f64,
    pub fov: f64,
    pub camera_origin: [f64; 3],
    pub camera_yaw: f64,
    pub camera_pitch: f64,
    pub camera_linear_speed: f64,
    pub camera_angular_speed: f64,
    pub camera_orbit_mode: bool,
}

pub fn load_config(json_string: &str) -> Config {
    let config: Config = serde_json::from_str(json_string)
        .unwrap_or_else(|_| panic!("Failed to deserialize json {}", json_string));

    config
}
