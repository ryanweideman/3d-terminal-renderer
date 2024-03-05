use nalgebra::{Matrix4, Point3, Vector3};

pub trait Camera {
    fn get_view_projection_matrix(&self) -> Matrix4<f64>;
}

mod controllable_perspective_camera;
pub use controllable_perspective_camera::ControllablePerspectiveCamera;
pub use controllable_perspective_camera::ControllablePerspectiveCameraBuilder;

mod orbiting_perspective_camera;
pub use orbiting_perspective_camera::OrbitingPerspectiveCamera;
pub use orbiting_perspective_camera::OrbitingPerspectiveCameraBuilder;

mod static_perspective_camera;
pub use static_perspective_camera::StaticPerspectiveCamera;
pub use static_perspective_camera::StaticPerspectiveCameraBuilder;

pub(crate) fn get_view_projection_matrix(
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

    let global_up = Vector3::y_axis();

    let right = direction.cross(&global_up).normalize();
    let up = right.cross(&direction);

    let view_matrix = Matrix4::look_at_rh(&origin, &(origin + direction), &up);

    projection_matrix * view_matrix
}
