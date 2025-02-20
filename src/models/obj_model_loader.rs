use crate::geometry::{Color, Model, Triangle3};
use crate::models::model_store::MaterialStore;
use crate::models::mtl_loader::Material;

use nalgebra::{Point3, Vector3};
use rand::Rng;

pub fn load_model(file_contents: &str, material_store: &MaterialStore) -> Model {
    let mut material_file_names: Vec<&str> = Vec::new();
    let mut vertices: Vec<(f32, f32, f32)> = Vec::new();
    let mut normals: Vec<(f32, f32, f32)> = Vec::new();
    let mut current_material: Option<Material> = None;

    let mut triangles: Vec<Triangle3> = Vec::new();

    for line in file_contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        //println!("{:#?}", current_material);

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("mtllib") => {
                material_file_names.push(parts.next().expect("Unable to process file name"))
            }
            Some("usemtl") => {
                let material_name = parts.next().expect("Unable to process material name");
                let material: Material = material_file_names
                    .iter()
                    .filter_map(|file_name| material_store.get(file_name, material_name))
                    .next()
                    .expect(&format!("Unknown material with name {}", material_name))
                    .clone();

                current_material.replace(material);
            }
            Some("v") => vertices.push(parse_vertex(parts).expect("Unable to process vertex")),
            Some("vn") => normals.push(parse_normal(parts).expect("Unable to process normal")),
            Some("f") => {
                if let Some(face_triangles) =
                    parse_face(parts, current_material.clone(), &vertices, &normals)
                {
                    triangles.extend(face_triangles);
                } else {
                    panic!("Unable to process face");
                }
            }
            _ => {}
        }
    }

    Model {
        geometry: triangles,
    }
}

fn parse_vertex<'a>(
    parts: impl Iterator<Item = &'a str> + std::fmt::Debug,
) -> Option<(f32, f32, f32)> {
    let mut coords = parts.filter_map(|p| p.parse::<f32>().ok());

    Some((coords.next()?, coords.next()?, coords.next()?))
}

fn parse_normal<'a>(parts: impl Iterator<Item = &'a str>) -> Option<(f32, f32, f32)> {
    let mut coords = parts.filter_map(|p| p.parse::<f32>().ok());
    Some((coords.next()?, coords.next()?, coords.next()?))
}

fn parse_face<'a>(
    parts: impl Iterator<Item = &'a str>,
    current_material: Option<Material>,
    vertices: &Vec<(f32, f32, f32)>,
    normals: &Vec<(f32, f32, f32)>,
) -> Option<Vec<Triangle3>> {
    let vertex_data: Vec<&str> = parts.collect();
    if vertex_data.len() < 3 {
        return None;
    }

    let mut parsed_tokens: Vec<(usize, usize)> = Vec::new();
    for token in &vertex_data {
        let parts: Vec<&str> = token.split('/').collect();
        let v_index = parts.get(0)?.parse::<usize>().ok()?;
        // Texture index is not supported
        let n_index = parts.get(2)?.parse::<usize>().ok()?;
        parsed_tokens.push((v_index, n_index));
    }

    // Assume all indices share the share the same normal
    let normal_index = parsed_tokens[0].1 - 1; // Convert 1-based index to 0-based.
    let normal_tuple: (f32, f32, f32) = normals.get(normal_index)?.clone();
    let computed_normal = Vector3::new(
        normal_tuple.0 as f64,
        normal_tuple.1 as f64,
        normal_tuple.2 as f64,
    );

    // Use color of material if it exists, otherwise use random color
    let mut rng = rand::thread_rng();
    let random_color = Color::new(rng.gen(), rng.gen(), rng.gen());
    let color = current_material
        .map(|material| {
            let kd = material.kd.unwrap();
            Color::new(
                (kd.0 * 255.0).round() as u8,
                (kd.1 * 255.0).round() as u8,
                (kd.2 * 255.0).round() as u8,
            )
        })
        .unwrap_or(random_color);

    // Fan triangulation:
    // For a face with vertices [v0, v1, v2, v3, ... vN],
    // produce triangles: [v0, v1, v2], [v0, v2, v3], ..., [v0, v_{N-1}, vN].
    let mut triangles: Vec<Triangle3> = Vec::new();
    for i in 1..(parsed_tokens.len() - 1) {
        let (v0_index, _) = parsed_tokens[0];
        let (v1_index, _) = parsed_tokens[i];
        let (v2_index, _) = parsed_tokens[i + 1];

        let v0 = vertices.get(v0_index - 1)?.clone();
        let v1 = vertices.get(v1_index - 1)?.clone();
        let v2 = vertices.get(v2_index - 1)?.clone();

        let p0 = Point3::new(v0.0 as f64, v0.1 as f64, v0.2 as f64);
        let p1 = Point3::new(v1.0 as f64, v1.1 as f64, v1.2 as f64);
        let p2 = Point3::new(v2.0 as f64, v2.1 as f64, v2.2 as f64);

        triangles.push(Triangle3 {
            vertices: [p0, p1, p2],
            color: color.clone(),
            normal: computed_normal, // Same normal for the entire face.
        });
    }

    Some(triangles)
}
