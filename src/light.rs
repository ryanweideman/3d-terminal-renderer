use crate::geometry::Color;
use nalgebra::Point3;

pub enum Light {
    PointLight(PointLight),
    AmbientLight(AmbientLight),
}

impl Light {
    pub fn get_intensity(&self) -> f64 {
        match self {
            Light::PointLight(point_light) => point_light.intensity,
            Light::AmbientLight(ambient_light) => ambient_light.intensity,
        }
    }

    #[allow(dead_code)]
    pub fn get_color(&self) -> Color {
        match self {
            Light::PointLight(point_light) => point_light.color,
            Light::AmbientLight(ambient_light) => ambient_light.color,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PointLight {
    pub origin: Point3<f64>,
    pub intensity: f64,
    pub linear_attenuation: f64,
    pub quadratic_attenuation: f64,
    pub color: Color,
}

impl PointLight {
    pub fn get_origin(&self) -> Point3<f64> {
        self.origin
    }

    pub fn get_linear_attenuation(&self) -> f64 {
        self.linear_attenuation
    }

    pub fn get_quadratic_attenuation(&self) -> f64 {
        self.quadratic_attenuation
    }
}

#[derive(Copy, Clone)]
pub struct AmbientLight {
    pub intensity: f64,
    pub color: Color,
}
