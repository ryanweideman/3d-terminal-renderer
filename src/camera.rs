use nalgebra::{Matrix4, Orthographic3, Perspective3, Point3, Rotation3, Vector3};
use std::f64::consts::PI;

use crate::config::Config;
use crate::keyboard::Keys;
/*
pub struct Camera {
    origin: Point3<f64>,
    yaw: f64,
    pitch: f64,
    linear_speed: f64,
    angular_speed: f64,
    orbit_mode: bool,
    projection_matrix: Matrix4<f64>,
}*/

pub trait Camera {
    fn get_view_projection_matrix(&self) -> Matrix4<f64>;
}

pub struct StaticPerspectiveCamera {
    origin: Point3<f64>,
    yaw: f64,
    pitch: f64,
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
impl StaticPerspectiveCamera {
    pub fn new(
        origin: Point3<f64>,
        yaw: f64,
        pitch: f64,
        aspect_ratio: f64,
        fov: f64,
        near_plane: f64,
        far_plane: f64,
    ) -> Self {
        let projection_matrix =
            Perspective3::new(aspect_ratio, fov, near_plane, far_plane).to_homogeneous();
        StaticPerspectiveCamera {
            origin: origin,
            yaw,
            pitch,
            projection_matrix,
        }
    }
}

impl Camera for StaticPerspectiveCamera {
    fn get_view_projection_matrix(&self) -> Matrix4<f64> {
        get_view_projection_matrix(&self.projection_matrix, self.origin, self.yaw, self.pitch)
    }
}

pub struct StaticPerspectiveCameraBuilder {
    origin: Point3<f64>,
    yaw: f64,
    pitch: f64,
    aspect_ratio: f64,
    fov: f64,
    near_plane: f64,
    far_plane: f64,
}

impl StaticPerspectiveCameraBuilder {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            yaw: -std::f64::consts::PI / 2.0,
            pitch: -0.4,
            fov: 1.25,
            aspect_ratio: 1.6,
            near_plane: 0.1,
            far_plane: 100.0,
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
    pub fn build(&mut self) -> StaticPerspectiveCamera {
        StaticPerspectiveCamera::new(
            self.origin,
            self.yaw,
            self.pitch,
            self.aspect_ratio,
            self.fov,
            self.near_plane,
            self.far_plane,
        )
    }
}

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
            origin: origin,
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
        get_view_projection_matrix(&self.projection_matrix, self.origin, self.yaw, self.pitch)
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

impl ControllablePerspectiveCameraBuilder {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            yaw: -std::f64::consts::PI / 2.0,
            pitch: -0.4,
            fov: 1.25,
            aspect_ratio: 1.6,
            near_plane: 0.1,
            far_plane: 100.0,
            linear_speed: 1.5,
            angular_speed: 1.0,
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

/*
impl Camera {
    pub fn new(config: &Config) -> Self {
        let origin = Point3::new(
            config.camera_origin[0],
            config.camera_origin[1],
            config.camera_origin[2],
        );
        /*
        let projection_matrix = Perspective3::new(
            config.aspect_ratio,
            config.fov,
            config.near_plane,
            config.far_plane,
        )
        .to_homogeneous();
                */
        let projection_matrix = Orthographic3::new(
            -3.0,
            3.0,
            -3.0,
            3.0,
            config.near_plane,
            config.far_plane
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

    pub fn update(&mut self, pressed_keys: &[Keys], delta_time: f64) {
        let mut velocity = Vector3::new(0.0, 0.0, 0.0);
        let mut yaw_velocity: f64 = 0.0;
        let mut pitch_velocity: f64 = 0.0;

        if !pressed_keys.is_empty() {
            self.orbit_mode = false;
        }

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
*/
fn get_view_projection_matrix(
    projection_matrix: &Matrix4<f64>,
    origin: Point3<f64>,
    yaw: f64,
    pitch: f64,
) -> Matrix4<f64> {
    let direction = Vector3::new(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    );

    let global_up = Vector3::new(0.0, 1.0, 0.0);

    let right = direction.cross(&global_up).normalize();
    let up = right.cross(&direction);

    let view_matrix = Matrix4::look_at_rh(&origin, &(origin + direction), &up);

    projection_matrix * view_matrix
}
