use crate::entity;
use crate::geometry::Color;
use crate::light;
use crate::model_loader::ModelLoader;
use nalgebra::{Matrix4, Point3, Rotation3, Unit, Vector3};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct JsonWorldData {
    objects: Vec<JsonObject>,
    lights: Vec<JsonLight>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum JsonObject {
    Square {
        model: String,
        origin: [f64; 3],
        rotation_axis: [f64; 3],
        rotation_angle: f64,
        scale: f64,
    },
    Rectangle {
        model: String,
        origin: [f64; 3],
        rotation_axis: [f64; 3],
        rotation_angle: f64,
        width: f64,
        height: f64,
        color: [u8; 3],
    },
    SpinningObject {
        model: String,
        origin: [f64; 3],
        rotation_axis: [f64; 3],
        rotation_angle: f64,
        angular_velocity: f64,
        scale: f64,
    },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum JsonLight {
    PointLight {
        origin: [f64; 3],
        intensity: f64,
        linear_attenuation: f64,
        quadratic_attenuation: f64,
        color: [u8; 3],
    },
    AmbientLight {
        intensity: f64,
        color: [u8; 3],
    },
}

pub fn load_world<'a>(
    json_string: &'a str,
    model_loader: &'a ModelLoader,
) -> (Vec<entity::Entity<'a>>, Vec<light::Light>) {
    let json_world_data: JsonWorldData = serde_json::from_str(json_string)
        .unwrap_or_else(|_| panic!("Failed to deserialize json {}", json_string));

    let objects = json_world_data
        .objects
        .iter()
        .map(|object| match object {
            JsonObject::Square {
                model,
                origin,
                rotation_axis,
                rotation_angle,
                scale,
            } => entity::Entity::Square(entity::Square {
                model: model_loader.get_model(model),
                origin: Point3::<f64>::new(origin[0], origin[1], origin[2]),
                rotation: Rotation3::<f64>::from_axis_angle(
                    &Unit::new_normalize(Vector3::new(
                        rotation_axis[0],
                        rotation_axis[1],
                        rotation_axis[2],
                    )),
                    *rotation_angle,
                ),
                scale: Matrix4::<f64>::new_scaling(*scale),
            }),
            JsonObject::Rectangle {
                model,
                origin,
                rotation_axis,
                rotation_angle,
                width,
                height,
                color,
            } => entity::Entity::Rectangle(entity::Rectangle {
                model: model_loader.get_model(model),
                origin: Point3::<f64>::new(origin[0], origin[1], origin[2]),
                rotation: Rotation3::<f64>::from_axis_angle(
                    &Unit::new_normalize(Vector3::new(
                        rotation_axis[0],
                        rotation_axis[1],
                        rotation_axis[2],
                    )),
                    *rotation_angle,
                ),
                scale: Matrix4::<f64>::new_nonuniform_scaling(&Vector3::new(*width, *height, 1.0)),
                color: Color::new(color[0], color[1], color[2]),
            }),
            JsonObject::SpinningObject {
                model,
                origin,
                rotation_axis,
                rotation_angle,
                angular_velocity,
                scale,
            } => entity::Entity::SpinningObject(entity::SpinningObject {
                model: model_loader.get_model(model),
                origin: Point3::<f64>::new(origin[0], origin[1], origin[2]),
                rotation_axis: Vector3::<f64>::new(
                    rotation_axis[0],
                    rotation_axis[1],
                    rotation_axis[2],
                ),
                rotation_angle: *rotation_angle,
                rotation_velocity: *angular_velocity,
                scale: Matrix4::<f64>::new_scaling(*scale),
            }),
        })
        .collect();

    let lights = json_world_data
        .lights
        .iter()
        .map(|light| match light {
            JsonLight::PointLight {
                origin,
                intensity,
                linear_attenuation,
                quadratic_attenuation,
                color,
            } => light::Light::PointLight(light::PointLight {
                origin: Point3::<f64>::new(origin[0], origin[1], origin[2]),
                intensity: *intensity,
                linear_attenuation: *linear_attenuation,
                quadratic_attenuation: *quadratic_attenuation,
                color: Color::new(color[0], color[1], color[2]),
            }),
            JsonLight::AmbientLight { intensity, color } => {
                light::Light::AmbientLight(light::AmbientLight {
                    intensity: *intensity,
                    color: Color::new(color[0], color[1], color[2]),
                })
            }
        })
        .collect();

    (objects, lights)
}
