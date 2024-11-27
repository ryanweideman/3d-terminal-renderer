use crate::geometry::Model;
use crate::model_loaders::ModelLoader;

use include_dir::Dir;
use std::collections::HashMap;

pub struct ObjModelLoader {
    models: HashMap<String, Model>,
}

impl ModelLoader for ObjModelLoader {
    fn get_model(&self, model_name: &str) -> &Model {
        self.models
            .get(model_name)
            .unwrap_or_else(|| panic!("Could not get model of name {}", model_name))
    }
}

impl ObjModelLoader {
    pub fn new(_dir: &Dir) -> Self {
        let models = HashMap::new();

        ObjModelLoader { models }
    }
}
