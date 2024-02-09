use crate:: geometry::{Color, ModelGeometry, Triangle3};
use nalgebra::{Point3};
use serde::{Deserialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct WorldData {
    objects: Vec<WorldObject>,
    lights: Vec<WorldLight>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum WorldObject {
    Square {
        model: String,
        origin: [[f32; 3]; 3],
        rotation_axis: [f32; 3],
        rotation_angle: f32,
    },
    SpinningCube {
        model: String,
        origin: [[f32; 3]; 3],
        rotation_axis: [f32; 3],
        rotation_angle: f32,
        angular_velocity: f32 
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum WorldLight {
    Point {
        origin: [[f32; 3]; 3]
    }
}

pub fn load_world(world_path: &str, models_path: &str) -> WorldData {
    let file_content = fs::read_to_string(world_path)
        .unwrap_or_else(|_| panic!("Failed to read file at path {}", world_path));

    let world_data: WorldData = serde_json::from_str(&file_content)
        .unwrap_or_else(|_| panic!("Failed to deserialize json at path {}", world_path));
    
    // process world data into ???

    world_data
}