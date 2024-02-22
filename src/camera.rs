use nalgebra::{Matrix4, Perspective3, Point3, Rotation3, Vector3};
use std::f64::consts::PI;

use crate::constants::{ASPECT_RATIO, FAR_PLANE, FOV, NEAR_PLANE};
use crate::keyboard::{Keyboard, Keys};

pub struct Camera {
    origin: Point3<f64>,
    yaw: f64,
    pitch: f64,
    orbit_mode: bool,
    projection_matrix: Matrix4<f64>,
}

/*
    Perspective3 produces a symmetric frustum identical to that used by OpenGL
    Perspective matrix :

    |  f / aspect  0                              0                                 0  |
    |  0           f                              0                                 0  |
    |  0           0   -(far + near) / (far - near)    -2 * far * near / (far - near)  |
    |  0           0                             -1                                 0  |

    where f = 1 / tan(fov / 2)
*/
impl Camera {
    pub fn new(origin: Point3<f64>) -> Self {
        let projection_matrix =
            Perspective3::new(ASPECT_RATIO, FOV, NEAR_PLANE, FAR_PLANE).to_homogeneous();
        Camera {
            origin: origin,
            yaw: -PI / 2.0,
            pitch: 0.0,
            orbit_mode: true,
            projection_matrix: projection_matrix,
        }
    }

    pub fn get_view_projection_matrix(&self) -> Matrix4<f64> {
        let direction = Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );

        let global_up = Vector3::new(0.0, 1.0, 0.0);

        let right = direction.cross(&global_up).normalize();
        let up = right.cross(&direction);

        let view_matrix = Matrix4::look_at_rh(&self.origin, &(self.origin + direction), &up);

        self.projection_matrix * view_matrix
    }

    pub fn update(&mut self, keyboard: &Keyboard, delta_time: f64) {
        let mut velocity = Vector3::new(0.0, 0.0, 0.0);
        let mut yaw_velocity: f64 = 0.0;
        let mut pitch_velocity: f64 = 0.0;

        let linear_speed: f64 = 1.5;
        let angular_speed: f64 = 1.0;

        if !keyboard.pressed_keys.is_empty() {
            self.orbit_mode = false;
        }

        keyboard.pressed_keys.iter().for_each(|key| match key {
            Keys::W => velocity += Vector3::new(0.0, 0.0, -linear_speed),
            Keys::A => velocity += Vector3::new(-linear_speed, 0.0, 0.0),
            Keys::S => velocity += Vector3::new(0.0, 0.0, linear_speed),
            Keys::D => velocity += Vector3::new(linear_speed, 0.0, 0.0),
            Keys::C => velocity += Vector3::new(0.0, -linear_speed, 0.0),
            Keys::Space => velocity += Vector3::new(0.0, linear_speed, 0.0),
            Keys::Left => yaw_velocity += -angular_speed,
            Keys::Right => yaw_velocity += angular_speed,
            Keys::Up => pitch_velocity += angular_speed,
            Keys::Down => pitch_velocity += -angular_speed,
            _ => {}
        });

        if self.orbit_mode {
            self.yaw += angular_speed * delta_time / 4.0;
            self.pitch = -PI / 8.0;
            let d = (self.origin.x * self.origin.x + self.origin.z * self.origin.z).sqrt();
            self.origin = Point3::new(
                d * (self.yaw + PI).cos(),
                self.origin.y,
                d * (self.yaw + PI).sin(),
            );
        } else {
            self.yaw += yaw_velocity * delta_time;
            self.pitch += pitch_velocity * delta_time;
            self.pitch = self.pitch.clamp(-1.5, 1.5);

            let rotation = Rotation3::from_euler_angles(0.0, -self.yaw - PI / 2.0, 0.0);
            self.origin += rotation * velocity * delta_time;
        }
    }
}
