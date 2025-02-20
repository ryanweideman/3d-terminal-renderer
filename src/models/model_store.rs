use crate::geometry::Model;
use crate::models::json_model_loader;
use crate::models::mtl_loader;
use crate::models::mtl_loader::Material;
use crate::models::obj_model_loader;

use include_dir::Dir;
use include_dir::File;
use std::collections::HashMap;

pub struct ModelStore<'a> {
    models: HashMap<String, Model>,
    dir: &'a Dir<'a>,
}

#[derive(Debug)]
pub struct MaterialStore {
    materials: HashMap<String, HashMap<String, Material>>,
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
        let mut material_store: MaterialStore = MaterialStore::new();

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

        // Process mtl files second
        self.dir
            .files()
            .flat_map(|file| get_file_info(file))
            .filter(|info| info.file_type == "mtl")
            .for_each(|info| {
                material_store.put(
                    &info.file_name,
                    &mtl_loader::parse_materials(info.file_contents),
                )
            });

        // Process obj files third
        self.dir
            .files()
            .flat_map(|file| get_file_info(file))
            .filter(|info| info.file_type == "obj")
            .for_each(|info| {
                let model_geometry =
                    obj_model_loader::load_model(info.file_contents, &material_store);
                //println!("{:#?}", model_geometry);
                self.models
                    .insert(info.file_name.to_string(), model_geometry);
            });

        //println!("{:#?}", material_store);
        //println!("{:#?}", self.models);
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

impl MaterialStore {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
        }
    }

    pub fn put(&mut self, file_name: &str, materials: &HashMap<String, Material>) {
        let file_materials = self
            .materials
            .entry(file_name.to_string())
            .or_insert_with(HashMap::new);

        for (material_name, material) in materials {
            file_materials.insert(material_name.clone(), material.clone());
        }
    }

    pub fn get(&self, file_name: &str, material_name: &str) -> Option<&Material> {
        self.materials
            .get(file_name)
            .and_then(|file_materials| file_materials.get(material_name))
    }
}
