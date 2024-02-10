use crate::geometry::{Color, Model, Triangle3};
use crate::model_loader::{ModelLoader};
use crate::world_objects; 
use nalgebra::{Point3, Vector3, Rotation3, Unit};
use serde::{Deserialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct JsonWorldData {
    objects: Vec<JsonObject>,
    lights: Vec<JsonLight>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum JsonObject {
    Square {
        model: String,
        origin: [f32; 3],
        rotation_axis: [f32; 3],
        rotation_angle: f32,
    },
    SpinningCube {
        model: String,
        origin: [f32; 3],
        rotation_axis: [f32; 3],
        rotation_angle: f32,
        angular_velocity: f32 
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum JsonLight {
    Point {
        origin: [[f32; 3]; 3]
    }
}

pub fn load_world<'a>(world_path: &'a str, model_loader: &'a ModelLoader) -> Vec<world_objects::Entity<'a>> {
    let file_content = fs::read_to_string(world_path)
        .unwrap_or_else(|_| panic!("Failed to read file at path {}", world_path));

    let json_world_data: JsonWorldData = serde_json::from_str(&file_content)
        .unwrap_or_else(|_| panic!("Failed to deserialize json at path {}", world_path));
    
    json_world_data.objects.iter()
        .map(|object| {
            match object {
                JsonObject::Square {
                    model,
                    origin,
                    rotation_axis,
                    rotation_angle,
                } => {
                    world_objects::Entity::Square(world_objects::Square {
                        model: &model_loader.get_model(model),
                        origin: Point3::new(origin[0], origin[1], origin[2]),
                        rotation: Rotation3::from_axis_angle(
                            &Unit::new_normalize(Vector3::new(rotation_axis[0], rotation_axis[1], rotation_axis[2])), 
                            *rotation_angle)
                    })
                },
                JsonObject::SpinningCube {
                    model,
                    origin,
                    rotation_axis,
                    rotation_angle,
                    angular_velocity,
                } => {
                    world_objects::Entity::SpinningCube(world_objects::SpinningCube {
                        model: &model_loader.get_model(model),
                        origin: Point3::new(origin[0], origin[1], origin[2]),
                        rotation_axis: Vector3::new(rotation_axis[0], rotation_axis[1], rotation_axis[2]),
                        rotation_angle: *rotation_angle,
                        rotation_velocity: *angular_velocity
                    })
                },
            }
        }).collect()
}