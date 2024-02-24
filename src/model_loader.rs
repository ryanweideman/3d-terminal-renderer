use crate::geometry::{Color, Model, Triangle3};
use nalgebra::Point3;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub struct ModelLoader {
    models: HashMap<String, Model>,
}

impl ModelLoader {
    pub fn new(path: &str) -> Self {
        let mut models = HashMap::new();

        let directory = fs::read_dir(path).expect("Failed to read directory");
        for entry in directory {
            let path = entry.expect("Failed to read directory entry").path();
            if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            let model_geometry = load_model(path.to_str().expect("REASON"));
            let file_name = path.file_name().and_then(|name| name.to_str()).unwrap();

            models.insert(file_name.to_string(), model_geometry);
        }

        ModelLoader { models }
    }

    pub fn get_model(&self, model_name: &str) -> &Model {
        self.models
            .get(model_name)
            .unwrap_or_else(|| panic!("Could not get model of name {}", model_name))
    }
}

#[derive(Deserialize, Debug)]
struct GeometryData {
    geometry: Vec<Triangle>,
}

#[derive(Deserialize, Debug)]
struct Triangle {
    vertices: [[f64; 3]; 3],
    color: [u8; 3],
}

fn load_model(path: &str) -> Model {
    let file_content =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file at path {}", path));

    let geometry_data: GeometryData = serde_json::from_str(&file_content)
        .unwrap_or_else(|_| panic!("Failed to deserialize json at path {}", path));

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
            }
        })
        .collect();

    Model { geometry }
}
