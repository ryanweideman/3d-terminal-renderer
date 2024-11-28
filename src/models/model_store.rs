use crate::geometry::Model;
use crate::models::json_model_loader;
use crate::models::obj_model_loader;

use include_dir::Dir;
use include_dir::File;
use std::collections::HashMap;

pub struct ModelStore<'a> {
    models: HashMap<String, Model>,
    dir: &'a Dir<'a>,
}

struct FileInfo<'b> {
    pub file_name: &'b str,
    pub file_type: &'b str,
    pub file_contents: &'b str,
}

impl<'a> ModelStore<'a> {
    pub fn new(dir: &'a Dir<'a>) -> Self {
        ModelStore {
            models: HashMap::new(),
            dir,
        }
    }

    pub fn init(&mut self) {
        // Process json files first
        self.dir
            .files()
            .flat_map(|file| get_file_info(file))
            .filter(|info| info.file_type == "json")
            .for_each(|info| {
                let model_geometry = json_model_loader::load_model(info.file_contents);
                self.models
                    .insert(info.file_name.to_string(), model_geometry);
            });

        // Process obj files second
        self.dir
            .files()
            .flat_map(|file| get_file_info(file))
            .filter(|info| info.file_type == "obj")
            .for_each(|info| {
                let model_geometry = obj_model_loader::load_model(info.file_contents);
                self.models
                    .insert(info.file_name.to_string(), model_geometry);
            });
    }

    pub fn get_model(&self, model_name: &str) -> &Model {
        self.models
            .get(model_name)
            .unwrap_or_else(|| panic!("Could not get model of name {}", model_name))
    }
}

fn get_file_info<'b>(file: &'b File<'b>) -> Option<FileInfo<'b>> {
    let file_type = file.path().extension()?.to_str()?;
    let file_name = file.path().file_name()?.to_str()?;
    let file_contents = file.contents_utf8()?;

    Some(FileInfo {
        file_name,
        file_type,
        file_contents,
    })
}
