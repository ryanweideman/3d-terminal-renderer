use nalgebra::{Matrix4, Perspective3, Point3};

use crate::camera::Camera;

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
            origin,
            yaw,
            pitch,
            projection_matrix,
        }
    }
}

impl Camera for StaticPerspectiveCamera {
    fn get_view_projection_matrix(&self) -> Matrix4<f64> {
        super::get_view_projection_matrix(
            &self.projection_matrix,
            self.origin,
            self.yaw,
            self.pitch,
        )
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

impl Default for StaticPerspectiveCameraBuilder {
    fn default() -> Self {
        Self::new()
    }
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
