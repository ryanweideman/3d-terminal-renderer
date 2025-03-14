use crate::geometry::{Color, Model, Triangle3};
use nalgebra::Vector3;

use nalgebra::Point3;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GeometryData {
    geometry: Vec<Triangle>,
}

#[derive(Deserialize, Debug)]
struct Triangle {
    vertices: [[f64; 3]; 3],
    color: [u8; 3],
}

pub fn load_model(json_string: &str) -> Model {
    let geometry_data: GeometryData = serde_json::from_str(json_string)
        .unwrap_or_else(|_| panic!("Failed to deserialize json {}", json_string));

    let model_geometry: Model = convert_geometry_data(&geometry_data);

    model_geometry
}

fn convert_geometry_data(geometry_data: &GeometryData) -> Model {
    let geometry: Vec<Triangle3> = geometry_data
        .geometry
        .iter()
        .map(|triangle| {
            let vertices = triangle.vertices;
            let color = triangle.color;
            let normal = calculate_triangle_normal(triangle);

            Triangle3 {
                vertices: [
                    Point3::new(vertices[0][0], vertices[0][1], vertices[0][2]),
                    Point3::new(vertices[1][0], vertices[1][1], vertices[1][2]),
                    Point3::new(vertices[2][0], vertices[2][1], vertices[2][2]),
                ],
                color: Color {
                    r: color[0],
                    g: color[1],
                    b: color[2],
                },
                normal: normal,
            }
        })
        .collect();

    Model { geometry }
}

fn calculate_triangle_normal(triangle: &Triangle) -> Vector3<f64> {
    let v0 = Point3::from(triangle.vertices[0]);
    let v1 = Point3::from(triangle.vertices[1]);
    let v2 = Point3::from(triangle.vertices[2]);

    (v1 - v0).cross(&(v2 - v0)).normalize()
}
