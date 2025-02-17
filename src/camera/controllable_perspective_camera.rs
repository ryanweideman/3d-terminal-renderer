#![allow(clippy::too_many_arguments)]

use std::f64::consts::PI;

use nalgebra::{Matrix4, Perspective3, Point3, Rotation3, Vector3};

use crate::camera::Camera;
use crate::terminal::keyboard::Keys;

pub struct ControllablePerspectiveCamera {
    origin: Point3<f64>,
    yaw: f64,
    pitch: f64,
    linear_speed: f64,
    angular_speed: f64,
    projection_matrix: Matrix4<f64>,
}

impl ControllablePerspectiveCamera {
    pub fn new(
        origin: Point3<f64>,
        yaw: f64,
        pitch: f64,
        aspect_ratio: f64,
        fov: f64,
        near_plane: f64,
        far_plane: f64,
        linear_speed: f64,
        angular_speed: f64,
    ) -> Self {
        let projection_matrix =
            Perspective3::new(aspect_ratio, fov, near_plane, far_plane).to_homogeneous();
        ControllablePerspectiveCamera {
            origin,
            yaw,
            pitch,
            linear_speed,
            angular_speed,
            projection_matrix,
        }
    }

    pub fn update(&mut self, delta_time: f64, pressed_keys: &[Keys]) {
        let mut velocity = Vector3::new(0.0, 0.0, 0.0);
        let mut yaw_velocity: f64 = 0.0;
        let mut pitch_velocity: f64 = 0.0;

        pressed_keys.iter().for_each(|key| match key {
            Keys::W => velocity += Vector3::new(0.0, 0.0, -self.linear_speed),
            Keys::A => velocity += Vector3::new(-self.linear_speed, 0.0, 0.0),
            Keys::S => velocity += Vector3::new(0.0, 0.0, self.linear_speed),
            Keys::D => velocity += Vector3::new(self.linear_speed, 0.0, 0.0),
            Keys::C => velocity += Vector3::new(0.0, -self.linear_speed, 0.0),
            Keys::Space => velocity += Vector3::new(0.0, self.linear_speed, 0.0),
            Keys::Left => yaw_velocity += -self.angular_speed,
            Keys::Right => yaw_velocity += self.angular_speed,
            Keys::Up => pitch_velocity += self.angular_speed,
            Keys::Down => pitch_velocity += -self.angular_speed,
            _ => {}
        });

        self.yaw += yaw_velocity * delta_time;
        self.pitch += pitch_velocity * delta_time;
        self.pitch = self.pitch.clamp(-1.5, 1.5);

        let rotation = Rotation3::from_euler_angles(0.0, -self.yaw - PI / 2.0, 0.0);
        self.origin += rotation * velocity * delta_time;
    }
}

impl Camera for ControllablePerspectiveCamera {
    fn get_view_projection_matrix(&self) -> Matrix4<f64> {
        super::get_view_projection_matrix(
            &self.projection_matrix,
            self.origin,
            self.yaw,
            self.pitch,
        )
    }
}

pub struct ControllablePerspectiveCameraBuilder {
    origin: Point3<f64>,
    yaw: f64,
    pitch: f64,
    aspect_ratio: f64,
    fov: f64,
    near_plane: f64,
    far_plane: f64,
    linear_speed: f64,
    angular_speed: f64,
}

impl Default for ControllablePerspectiveCameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllablePerspectiveCameraBuilder {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            yaw: -std::f64::consts::PI / 2.0,
            pitch: -0.4,
            fov: 1.0,
            aspect_ratio: 0.4,
            near_plane: 0.01,
            far_plane: 1000.0,
            linear_speed: 10.0,
            angular_speed: 2.0,
        }
    }

    #[allow(unused)]
    pub fn origin(&mut self, origin: Point3<f64>) -> &mut Self {
        self.origin = origin;
        self
    }

    #[allow(unused)]
    pub fn yaw(&mut self, yaw: f64) -> &mut Self {
        self.yaw = yaw;
        self
    }

    #[allow(unused)]
    pub fn pitch(&mut self, pitch: f64) -> &mut Self {
        self.pitch = pitch;
        self
    }

    #[allow(unused)]
    pub fn aspect_ratio(&mut self, aspect_ratio: f64) -> &mut Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    #[allow(unused)]
    pub fn fov(&mut self, fov: f64) -> &mut Self {
        self.fov = fov;
        self
    }

    #[allow(unused)]
    pub fn near_plane(&mut self, near_plane: f64) -> &mut Self {
        self.near_plane = near_plane;
        self
    }

    #[allow(unused)]
    pub fn far_plane(&mut self, far_plane: f64) -> &mut Self {
        self.far_plane = far_plane;
        self
    }

    #[allow(unused)]
    pub fn linear_speed(&mut self, linear_speed: f64) -> &mut Self {
        self.linear_speed = linear_speed;
        self
    }

    #[allow(unused)]
    pub fn angular_speed(&mut self, angular_speed: f64) -> &mut Self {
        self.angular_speed = angular_speed;
        self
    }

    #[allow(unused)]
    pub fn build(&mut self) -> ControllablePerspectiveCamera {
        ControllablePerspectiveCamera::new(
            self.origin,
            self.yaw,
            self.pitch,
            self.aspect_ratio,
            self.fov,
            self.near_plane,
            self.far_plane,
            self.linear_speed,
            self.angular_speed,
        )
    }
}
