use nalgebra::{Matrix4, Point2, Point3, Rotation3};

pub struct Camera {
    origin: Point3<f32>
}

impl Camera {
    pub fn new(origin: Point3<f32>) -> Self {
        Camera {
            origin: origin
        }
    }

    pub fn get_transform(&self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.origin.coords);
        let transform = translation;

        transform
    }
}