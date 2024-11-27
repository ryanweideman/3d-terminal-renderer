use crate::geometry::Model;
use crate::models::json_model_loader;

use include_dir::Dir;
use std::collections::HashMap;

pub struct ModelStore<'a> {
    models: HashMap<String, Model>,
    dir: &'a Dir<'a>,
}

impl<'a> ModelStore<'a> {
    pub fn new(dir: &'a Dir<'a>) -> Self {
        ModelStore { 
            models: HashMap::new(),
            dir
        }
    }

    pub fn init(&mut self) {
        for file in self.dir.files() {
            let path = file.path();

            let file_type = path
                .extension()
                .and_then(|ext| ext.to_str());
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap();
            let file_contents = file.contents_utf8().expect("Failed to read file contents");

            match file_type {
                Some("json") => {
                    let model_geometry = json_model_loader::load_model(file_contents);
                    self.models.insert(file_name.to_string(), model_geometry);
                },
                Some("obj") => {},
                Some(_other) => {},
                None => {},
            }
        }
    }

    pub fn get_model(&self, model_name: &str) -> &Model {
        self.models
            .get(model_name)
            .unwrap_or_else(|| panic!("Could not get model of name {}", model_name))
    }
}
