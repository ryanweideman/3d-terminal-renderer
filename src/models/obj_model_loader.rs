use crate::geometry::{Color, Model, Triangle3};
use crate::models::mtl_loader::Material;
use crate::models::model_store::MaterialStore;

use nalgebra::Point3;
use rand::Rng;

pub fn load_model(file_contents: &str, material_store: &MaterialStore) -> Model {
    let mut material_file_names : Vec<&str>  = Vec::new();
    let mut vertices  : Vec<(f32, f32, f32)> = Vec::new();
    let mut normals   : Vec<(f32, f32, f32)> = Vec::new();
    let mut current_material : Option<Material> = None;

    let mut triangles : Vec<Triangle3> = Vec::new();

    for line in file_contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        //println!("{:#?}", current_material);

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("mtllib") => 
                material_file_names.push(parts.next()
                    .expect("Unable to process file name")),
            Some("usemtl") => {
                let material_name = parts.next()
                    .expect("Unable to process material name");
                let material : Material = material_file_names.iter()
                    .filter_map(|file_name| material_store.get(file_name, material_name))
                    .next()
                    .expect(&format!("Unknown material with name {}", material_name))
                    .clone();
                    
                current_material.replace(material);
            }
            Some("v") => 
                vertices.push(parse_vertex(parts)
                    .expect("Unable to process vertex")),
            Some("vn") => 
                normals.push(parse_normal(parts)
                    .expect("Unable to process normal")),
            Some("f") => {
                triangles.push(parse_face(
                    parts, 
                    current_material.clone(),
                    &vertices,
                    &normals).expect("Unable to process face"));
            }
            _ => {}
        }
    }

    Model {
        geometry: triangles,
    }
}

fn parse_vertex<'a>(parts: impl Iterator<Item = &'a str>) -> Option<(f32, f32, f32)> {
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
        normals: &Vec<(f32, f32, f32)>) -> Option<Triangle3> {

    let vertex_data : Vec<&str>= parts.collect();
    let data_0 : Vec<Option<usize>> = vertex_data[0]
        .split('/')
        .map(|s| s.parse::<usize>().ok())
        .collect();
    let data_1 : Vec<Option<usize>> = vertex_data[1]
        .split('/')
        .map(|s| s.parse::<usize>().ok())
        .collect();
    let data_2 : Vec<Option<usize>> = vertex_data[2]
        .split('/')
        .map(|s| s.parse::<usize>().ok())
        .collect();

    assert!(data_0[2].unwrap() == data_1[2].unwrap() && data_1[2].unwrap() == data_2[2].unwrap());
    let normal_index = data_0[2].unwrap() - 1;
    let _normal : (f32, f32, f32) = normals[normal_index];

    // Obj uses 1 based indexing, so subtracting 1 is necessary for converting to 0 based
    let v0 = vertices[data_0[0].unwrap() - 1];
    let v1 = vertices[data_1[0].unwrap() - 1];
    let v2 = vertices[data_2[0].unwrap() - 1];

    let p0 = Point3::new(v0.0 as f64, v0.1 as f64, v0.2 as f64);
    let p1 = Point3::new(v1.0 as f64, v1.1 as f64, v1.2 as f64);
    let p2 = Point3::new(v2.0 as f64, v2.1 as f64, v2.2 as f64);

    let mut rng = rand::thread_rng();
    let rcolor_1 = Color::new(rng.gen(), rng.gen(), rng.gen());

    let color = current_material.map(|material| {
        let kd = material.kd.unwrap();
        Color::new(
            (kd.0 * 255.0).round() as u8, 
            (kd.1 * 255.0).round() as u8, 
            (kd.2 * 255.0).round() as u8)
    }).unwrap_or(rcolor_1);

    Some(Triangle3 {
        vertices : [p0, p1, p2],
        color : color
    })
}
