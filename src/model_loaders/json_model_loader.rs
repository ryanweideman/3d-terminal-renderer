use crate::geometry::{Color, Model, Triangle3};
use include_dir::Dir;
use nalgebra::Point3;
use serde::Deserialize;
use std::collections::HashMap;

pub struct JsonModelLoader {
    models: HashMap<String, Model>,
}

impl JsonModelLoader {
    pub fn new(dir: &Dir) -> Self {
        let mut models = HashMap::new();

        for file in dir.files() {
            if file.path().extension().and_then(|ext| ext.to_str()) == Some("json") {
                let file_contents = file.contents_utf8().expect("Failed to read file contents");
                let model_geometry = load_model(file_contents);
                let file_name = file
                    .path()
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap();

                models.insert(file_name.to_string(), model_geometry);
            }
        }

        JsonModelLoader { models }
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

fn load_model(json_string: &str) -> Model {
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
