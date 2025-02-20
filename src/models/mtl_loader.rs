use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub kd: Option<(f32, f32, f32)>,
}

pub fn parse_materials(file_contents: &str) -> HashMap<String, Material> {
    let mut materials: HashMap<String, Material> = HashMap::new();

    let mut current_material: Option<Material> = None;

    for line in file_contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("newmtl") => {
                // Save the previous material
                if let Some(material) = current_material.take() {
                    materials.insert(material.name.clone(), material);
                }

                // Start a new material
                if let Some(name) = parts.next() {
                    current_material = Some(Material {
                        name: name.to_string(),
                        kd: None,
                    });
                }
            }
            Some("Kd") => {
                if let Some(material) = current_material.as_mut() {
                    let kd_values: Vec<f32> = parts.filter_map(|p| p.parse::<f32>().ok()).collect();

                    material.kd = Some((kd_values[0], kd_values[1], kd_values[2]));
                }
            }
            _ => {}
        }
    }

    // Save the last material
    if let Some(material) = current_material {
        materials.insert(material.name.clone(), material);
    }

    materials
}
