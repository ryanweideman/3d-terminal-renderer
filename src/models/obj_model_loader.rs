use crate::geometry::Model;

pub fn load_model(_file_contents: &str) -> Model {
    Model {
        geometry: Vec::new(),
    }
}
