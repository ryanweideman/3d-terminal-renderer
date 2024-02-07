use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use nalgebra::{Matrix4, Point3, Rotation3};


#[derive(Copy, Clone)]
pub struct Color { 
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Copy, Clone)]
pub struct Triangle3 {
    pub geometry: [Point3<f32> ; 3],
    pub color: Color
}

pub struct Cube {
    pub origin: Point3<f32>,
    pub rotation: Rotation3<f32>
}

pub struct PointLight {
    pub origin: Point3<f32>
}

pub const CUBE_TRIANGLES: [Triangle3; 12] = [
    // Front face
    Triangle3 { geometry: [Point3::new(-0.5, -0.5,  0.5), Point3::new( 0.5, -0.5,  0.5), Point3::new( 0.5,  0.5,  0.5)], color: Color {r: 255, g: 0, b: 0} },
    Triangle3 { geometry: [Point3::new(-0.5, -0.5,  0.5), Point3::new( 0.5,  0.5,  0.5), Point3::new(-0.5,  0.5,  0.5)], color: Color {r: 255, g: 0, b: 0} },
    
    // Back face
    Triangle3 { geometry: [Point3::new(-0.5, -0.5, -0.5), Point3::new( 0.5,  0.5, -0.5), Point3::new( 0.5, -0.5, -0.5)], color: Color {r: 0, g: 255, b: 0} },
    Triangle3 { geometry: [Point3::new(-0.5, -0.5, -0.5), Point3::new(-0.5,  0.5, -0.5), Point3::new( 0.5,  0.5, -0.5)], color: Color {r: 0, g: 255, b: 0} },
    
    // Right face
    Triangle3 { geometry: [Point3::new( 0.5, -0.5, -0.5), Point3::new( 0.5,  0.5,  0.5), Point3::new( 0.5, -0.5,  0.5)], color: Color {r: 0, g: 0, b: 255} },
    Triangle3 { geometry: [Point3::new( 0.5, -0.5, -0.5), Point3::new( 0.5,  0.5, -0.5), Point3::new( 0.5,  0.5,  0.5)], color: Color {r: 0, g: 0, b: 255} },
    
    // Left face
    Triangle3 { geometry: [Point3::new(-0.5, -0.5, -0.5), Point3::new(-0.5, -0.5,  0.5), Point3::new(-0.5,  0.5,  0.5)], color: Color {r: 255, g: 255, b: 0} },
    Triangle3 { geometry: [Point3::new(-0.5, -0.5, -0.5), Point3::new(-0.5,  0.5,  0.5), Point3::new(-0.5,  0.5, -0.5)], color: Color {r: 255, g: 255, b: 0} },
    
    // Top face
    Triangle3 { geometry: [Point3::new(-0.5,  0.5, -0.5), Point3::new(-0.5,  0.5,  0.5), Point3::new( 0.5,  0.5,  0.5)], color: Color {r: 255, g: 0, b: 255} },
    Triangle3 { geometry: [Point3::new(-0.5,  0.5, -0.5), Point3::new( 0.5,  0.5,  0.5), Point3::new( 0.5,  0.5, -0.5)], color: Color {r: 255, g: 0, b: 255} },
    
    // Bottom face
    Triangle3 { geometry: [Point3::new(-0.5, -0.5, -0.5), Point3::new( 0.5, -0.5, -0.5), Point3::new( 0.5, -0.5,  0.5)], color: Color {r: 0, g: 255, b: 255} },
    Triangle3 { geometry: [Point3::new(-0.5, -0.5, -0.5), Point3::new( 0.5, -0.5,  0.5), Point3::new(-0.5, -0.5,  0.5)], color: Color {r: 0, g: 255, b: 255} },
];

pub fn get_cube_geometry(cube: &Cube) -> [Triangle3; 12] {
    let rotation = Matrix4::from(cube.rotation);
    let translation = Matrix4::new_translation(&cube.origin.coords);

    // First rotate the geometry around it's origin (model space), then translate it to the desired position (world space)
    let transform = translation * rotation;

    let transformed_triangles_vec: Vec<Triangle3> = CUBE_TRIANGLES.iter().map(|triangle| {
        let transformed_geometry = triangle.geometry.iter().map(|vertex| {
            transform.transform_point(&vertex)
        }).collect::<Vec<Point3<f32>>>();
        
        Triangle3 {
            geometry: [transformed_geometry[0], transformed_geometry[1], transformed_geometry[2]],
            color: triangle.color.clone()
        }
    }).collect();

    let transformed_triangles: [Triangle3; 12] = match transformed_triangles_vec.try_into() {
        Ok(arr) => arr,
        Err(_) => panic!("Expected a Vec of length 12"),
    };

    transformed_triangles
}

#[allow(dead_code)]
pub fn is_vertex_outside_frustum(ndc : &Point3<f32>) -> bool {
    let x_out_of_range = ndc.x < -1.0 || ndc.x > 1.0;
    let y_out_of_range = ndc.y < -1.0 || ndc.y > 1.0;
    let z_out_of_range = ndc.z < -1.0 || ndc.z > 1.0;

    x_out_of_range || y_out_of_range || z_out_of_range
}

pub fn ndc_to_screen(ndc : &Point3<f32>) -> Point3<f32> {
    let px = (ndc.x + 1.0) / 2.0 * (SCREEN_WIDTH as f32);
    let py = (ndc.y + 1.0) / 2.0 * (SCREEN_HEIGHT as f32);
    Point3::new(px, py, ndc.z)
}

pub fn screen_to_ndc(screen: &Point3<f32>) -> Point3<f32> {
    let x_ndc = (screen.x / (SCREEN_WIDTH as f32)) * 2.0 - 1.0;
    let y_ndc = (screen.y / (SCREEN_HEIGHT as f32)) * 2.0 - 1.0;
    let z_ndc = screen.z; 

    Point3::new(x_ndc, y_ndc, z_ndc)
}