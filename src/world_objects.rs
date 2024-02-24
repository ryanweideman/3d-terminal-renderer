use crate::geometry::{Color, Model, Triangle3};
use nalgebra::{Matrix4, Point3, Rotation3, Unit, Vector3};

pub enum Entity<'a> {
    Square(Square<'a>),
    SpinningObject(SpinningObject<'a>),
    Rectangle(Rectangle<'a>),
}

impl<'a> Entity<'a> {
    pub fn update(&mut self, dt: f64) {
        if let Entity::SpinningObject(object) = self {
            object.update(dt)
        }
    }

    pub fn get_origin(&self) -> Point3<f64> {
        match self {
            Entity::Square(square) => square.origin,
            Entity::SpinningObject(object) => object.origin,
            Entity::Rectangle(rectangle) => rectangle.origin,
        }
    }

    pub fn get_rotation(&self) -> Rotation3<f64> {
        match self {
            Entity::Square(square) => square.rotation,
            Entity::SpinningObject(object) => Rotation3::from_axis_angle(
                &Unit::new_normalize(Vector3::new(
                    object.rotation_axis[0],
                    object.rotation_axis[1],
                    object.rotation_axis[2],
                )),
                object.rotation_angle,
            ),
            Entity::Rectangle(rectangle) => rectangle.rotation,
        }
    }

    pub fn get_scale(&self) -> Matrix4<f64> {
        match self {
            Entity::Square(square) => square.scale,
            Entity::SpinningObject(object) => object.scale,
            Entity::Rectangle(rectangle) => rectangle.scale,
        }
    }

    pub fn get_model_geometry(&self) -> Vec<Triangle3> {
        match self {
            Entity::Square(square) => square.model.geometry.clone(),
            Entity::SpinningObject(object) => object.model.geometry.clone(),
            Entity::Rectangle(rectangle) => rectangle.model.geometry.clone(),
        }
    }

    pub fn get_maybe_color(&self) -> Option<Color> {
        match self {
            Entity::Rectangle(rectangle) => Some(rectangle.color),
            _ => None,
        }
    }
}

trait Updatable {
    fn update(&mut self, dt: f64);
}

trait WorldLight {}

#[derive(Copy, Clone)]
pub struct Cube {
    pub origin: Point3<f64>,
    pub rotation: Rotation3<f64>,
}

#[derive(Copy, Clone)]
pub struct Square<'a> {
    pub model: &'a Model,
    pub origin: Point3<f64>,
    pub rotation: Rotation3<f64>,
    pub scale: Matrix4<f64>,
}

#[derive(Copy, Clone)]
pub struct Rectangle<'a> {
    pub model: &'a Model,
    pub origin: Point3<f64>,
    pub rotation: Rotation3<f64>,
    pub scale: Matrix4<f64>,
    pub color: Color,
}

#[derive(Copy, Clone)]
pub struct SpinningObject<'a> {
    pub model: &'a Model,
    pub origin: Point3<f64>,
    pub rotation_axis: Vector3<f64>,
    pub rotation_angle: f64,
    pub rotation_velocity: f64,
    pub scale: Matrix4<f64>,
}

impl Updatable for SpinningObject<'_> {
    fn update(&mut self, delta_time: f64) {
        self.rotation_angle += self.rotation_velocity * delta_time;
    }
}

pub enum Light {
    PointLight(PointLight),
    AmbientLight(AmbientLight),
}

impl Light {
    pub fn get_intensity(&self) -> f64 {
        match self {
            Light::PointLight(point_light) => point_light.intensity,
            Light::AmbientLight(ambient_light) => ambient_light.intensity,
        }
    }

    #[allow(dead_code)]
    pub fn get_color(&self) -> Color {
        match self {
            Light::PointLight(point_light) => point_light.color,
            Light::AmbientLight(ambient_light) => ambient_light.color,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PointLight {
    pub origin: Point3<f64>,
    pub intensity: f64,
    pub linear_attenuation: f64,
    pub quadratic_attenuation: f64,
    pub color: Color,
}

impl PointLight {
    pub fn get_origin(&self) -> Point3<f64> {
        self.origin
    }

    pub fn get_linear_attenuation(&self) -> f64 {
        self.linear_attenuation
    }

    pub fn get_quadratic_attenuation(&self) -> f64 {
        self.quadratic_attenuation
    }
}

#[derive(Copy, Clone)]
pub struct AmbientLight {
    pub intensity: f64,
    pub color: Color,
}
