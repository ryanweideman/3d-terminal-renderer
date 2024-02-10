use nalgebra::{Matrix3x4, Matrix4, Point2, Point3, Point4, Rotation3, Perspective3, Vector3, Unit};
use crate::geometry::{Model, Triangle3};

pub enum Entity<'a> {
    Square(Square<'a>),
    SpinningCube(SpinningCube<'a>),
}

impl<'a> Entity<'a> {
    pub fn update(&mut self, dt: f32) {
        match self {
            Entity::SpinningCube(cube) => cube.update(dt),
            _ => {}
        }
    }

    pub fn get_model(&self) -> Model {
        match self {
            Entity::Square(square) => square.model.clone(),
            Entity::SpinningCube(cube) => cube.model.clone()
        }
    }

    pub fn get_origin(&self) -> Point3<f32> {
        match self {
            Entity::Square(square) => square.origin,
            Entity::SpinningCube(cube) => cube.origin
        }
    }

    pub fn get_rotation(&self) -> Rotation3<f32> {
        match self {
            Entity::Square(square) => square.rotation,
            Entity::SpinningCube(cube) => {
                Rotation3::from_axis_angle(
                    &Unit::new_normalize(Vector3::new(cube.rotation_axis[0], cube.rotation_axis[1], cube.rotation_axis[2])), 
                    cube.rotation_angle)
            }
        }
    }

    pub fn get_scale(&self) -> f32 {
        match self {
            Entity::Square(square) => square.scale,
            Entity::SpinningCube(cube) => cube.scale
        }
    }

    pub fn get_model_geometry(&self) -> Vec<Triangle3> {
        match self {
            Entity::Square(square) => square.model.geometry.clone(),
            Entity::SpinningCube(cube) => cube.model.geometry.clone()
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
    pub scale: f32
}

#[derive(Copy, Clone)]
pub struct SpinningCube<'a> {
    pub model: &'a Model,
    pub origin: Point3<f32>,
    pub rotation_axis: Vector3<f32>,
    pub rotation_angle: f32,
    pub rotation_velocity: f32,
    pub scale: f32
}

impl Updatable for SpinningCube<'_> {
    fn update(&mut self, dt: f32) {
        self.rotation_angle += self.rotation_velocity;
    }
}

#[derive(Copy, Clone)]
pub struct PointLight {
    pub origin: Point3<f32>
}

impl WorldLight for PointLight {}