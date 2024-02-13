use nalgebra::{Matrix4, Point3, Rotation3, Vector3, Unit};
use crate::geometry::{Color, Model, Triangle3};

pub enum Entity<'a> {
    Square(Square<'a>),
    SpinningCube(SpinningCube<'a>),
    Rectangle(Rectangle<'a>)
}

impl<'a> Entity<'a> {
    pub fn update(&mut self, dt: f32) {
        match self {
            Entity::SpinningCube(cube) => cube.update(dt),
            _ => {}
        }
    }

    pub fn get_origin(&self) -> Point3<f32> {
        match self {
            Entity::Square(square) => square.origin,
            Entity::SpinningCube(cube) => cube.origin,
            Entity::Rectangle(rectangle) => rectangle.origin
        }
    }

    pub fn get_rotation(&self) -> Rotation3<f32> {
        match self {
            Entity::Square(square) => square.rotation,
            Entity::SpinningCube(cube) => {
                Rotation3::from_axis_angle(
                    &Unit::new_normalize(Vector3::new(cube.rotation_axis[0], cube.rotation_axis[1], cube.rotation_axis[2])), 
                    cube.rotation_angle)
            },
            Entity::Rectangle(rectangle) => rectangle.rotation
        }
    }

    pub fn get_scale(&self) -> Matrix4<f32> {
        match self {
            Entity::Square(square) => square.scale,
            Entity::SpinningCube(cube) => cube.scale,
            Entity::Rectangle(rectangle) => rectangle.scale
        }
    }

    pub fn get_model_geometry(&self) -> Vec<Triangle3> {
        match self {
            Entity::Square(square) => square.model.geometry.clone(),
            Entity::SpinningCube(cube) => cube.model.geometry.clone(),
            Entity::Rectangle(rectangle) => rectangle.model.geometry.clone()
        }
    }

    pub fn get_maybe_color(&self) -> Option<Color> {
        match self {
            Entity::Rectangle(rectangle) => Some(rectangle.color),
            _ => None
        }
    }
}

trait Updatable {
    fn update(&mut self, dt: f32);
}

trait WorldLight {}

#[derive(Copy, Clone)]
pub struct Cube {
    pub origin: Point3<f32>,
    pub rotation: Rotation3<f32>
}

#[derive(Copy, Clone)]
pub struct Square<'a> {
    pub model: &'a Model,
    pub origin: Point3<f32>,
    pub rotation: Rotation3<f32>,
    pub scale: Matrix4<f32>
}

#[derive(Copy, Clone)]
pub struct Rectangle<'a> {
    pub model: &'a Model,
    pub origin: Point3<f32>,
    pub rotation: Rotation3<f32>,
    pub scale: Matrix4<f32>,
    pub color: Color
}

#[derive(Copy, Clone)]
pub struct SpinningCube<'a> {
    pub model: &'a Model,
    pub origin: Point3<f32>,
    pub rotation_axis: Vector3<f32>,
    pub rotation_angle: f32,
    pub rotation_velocity: f32,
    pub scale: Matrix4<f32>
}

impl Updatable for SpinningCube<'_> {
    fn update(&mut self, _dt: f32) {
        self.rotation_angle += self.rotation_velocity;
    }
}

#[derive(Copy, Clone)]
pub struct PointLight {
    pub origin: Point3<f32>
}

impl WorldLight for PointLight {}