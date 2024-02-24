use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub target_fps: f64,
    pub screen_width: usize,
    pub screen_height: usize,
    pub background_color: [u8; 3],
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

pub fn load_config(path: &str) -> Config {
    let file_content =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file at path {}", path));

    let config: Config = serde_json::from_str(&file_content)
        .unwrap_or_else(|_| panic!("Failed to deserialize json at path {}", path));

    config
}
