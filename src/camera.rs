use nalgebra::{Matrix4, Point3, Rotation3, Vector3};
use crate::keyboard::{Keyboard, Keys}; 

pub struct Camera {
    origin: Point3<f32>,
    yaw: f32,
    pitch: f32
}

impl Camera {
    pub fn new(origin: Point3<f32>) -> Self {
        Camera {
            origin: origin,
            yaw: 0.0,
            pitch: 0.0
        }
    }

    pub fn get_transform(&self) -> Matrix4<f32> {
        let direction = Vector3::new(
            self.yaw.sin(),
            self.pitch.sin(),
            -self.yaw.cos() * self.pitch.cos(),
        );

        let global_up = Vector3::new(0.0, 1.0, 0.0);

        let right = direction.cross(&global_up).normalize();
        let up = right.cross(&direction);

        let view_matrix = Matrix4::look_at_rh(
            &self.origin,
            &(self.origin + direction),
            &up,
        );

        view_matrix
    }

    pub fn update(&mut self, keyboard: &Keyboard) {
        let mut velocity = Vector3::new(0.0, 0.0, 0.0);
        let mut yaw_velocity : f32 = 0.0;
        let mut pitch_velocity : f32 = 0.0;

        let linear_speed: f32 = 0.35;
        let angular_speed: f32 = 0.06;

        keyboard.pressed_keys.iter()
            .for_each(|key| {
                match key {
                    Keys::W => velocity += Vector3::new(0.0, 0.0, -linear_speed),
                    Keys::A => velocity += Vector3::new(-linear_speed, 0.0, 0.0),
                    Keys::S => velocity += Vector3::new(0.0, 0.0, linear_speed),
                    Keys::D => velocity += Vector3::new(linear_speed, 0.0, 0.0),
                    Keys::C => velocity += Vector3::new(0.0, linear_speed, 0.0),
                    Keys::Space => velocity += Vector3::new(0.0, -linear_speed, 0.0),
                    Keys::Left  => yaw_velocity   += -angular_speed,
                    Keys::Right => yaw_velocity   += angular_speed,
                    Keys::Up    => pitch_velocity += -angular_speed,
                    Keys::Down  => pitch_velocity += angular_speed,
                    _ => {}
                }
            });

        self.yaw   += yaw_velocity;
        self.pitch += pitch_velocity;
        self.pitch = self.pitch.clamp(-1.1, 1.1);

        let rotation = Rotation3::from_euler_angles(0.0, -self.yaw, 0.0);

        self.origin += rotation * velocity;
    }
}