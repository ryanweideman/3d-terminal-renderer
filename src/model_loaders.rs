use crate::geometry::Model;

pub trait ModelLoader {
    fn get_model(&self, model_name: &str) -> &Model;
}

mod json_model_loader;
pub use json_model_loader::JsonModelLoader;
