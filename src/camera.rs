use nalgebra::{Matrix4, Perspective3, Point3, Rotation3, Vector3};
use std::f64::consts::PI;

use crate::config::Config;
use crate::keyboard::{Keyboard, Keys};

pub struct Camera {
    origin: Point3<f64>,
    yaw: f64,
    pitch: f64,
    linear_speed: f64,
    angular_speed: f64,
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
    pub fn new(config: &Config) -> Self {
        let origin = Point3::new(
            config.camera_origin[0],
            config.camera_origin[1],
            config.camera_origin[2],
        );
        let aspect_ratio = (config.screen_width as f64) / (config.screen_height as f64);
        let projection_matrix = Perspective3::new(
            aspect_ratio,
            config.fov,
            config.near_plane,
            config.far_plane,
        )
        .to_homogeneous();
        Camera {
            origin,
            yaw: config.camera_yaw,
            pitch: config.camera_pitch,
            linear_speed: config.camera_linear_speed,
            angular_speed: config.camera_angular_speed,
            orbit_mode: config.camera_orbit_mode,
            projection_matrix,
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

        if !keyboard.pressed_keys.is_empty() {
            self.orbit_mode = false;
        }

        keyboard.pressed_keys.iter().for_each(|key| match key {
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

        if self.orbit_mode {
            self.yaw += self.angular_speed * delta_time / 4.0;
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
