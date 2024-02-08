use crate:: geometry::{Color, ModelGeometry, Triangle3};
use nalgebra::{Point3};
use serde::{Deserialize};
use std::fs;

#[derive(Deserialize, Debug)]
struct GeometryData {
    geometry: Vec<Triangle>,
}

#[derive(Deserialize, Debug)]
struct Triangle {
    vertices: [[f32; 3]; 3],
    color: [u8; 3],
}

fn convert_geometry_data(geometry_data: &GeometryData) -> ModelGeometry {
    let geometry : Vec::<Triangle3> = geometry_data.geometry.iter()
        .map(|triangle| {
            let vertices = triangle.vertices;
            let color = triangle.color;
            Triangle3 {
                vertices: 
                [
                    Point3::new(vertices[0][0], vertices[0][1], vertices[0][2]),
                    Point3::new(vertices[1][0], vertices[1][1], vertices[1][2]),
                    Point3::new(vertices[2][0], vertices[2][1], vertices[2][2])
                ],
                color: Color {
                    r: color[0],
                    g: color[1],
                    b: color[2]
                }
            }
        })
        .collect();

    ModelGeometry {
        geometry: geometry
    }
}

pub fn read_model(path: &str) -> ModelGeometry {
    let file_content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read file at path {}", path));

    // Deserialize the JSON content into the `GeometryData` struct
    let geometry_data: GeometryData = serde_json::from_str(&file_content)
        .unwrap_or_else(|_| panic!("Failed to deserialize json at path {}", path));
    
    let model_geometry : ModelGeometry = convert_geometry_data(&geometry_data);

    model_geometry
}