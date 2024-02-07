use nalgebra::{Matrix3x4, Matrix4, Point3, Point4, Rotation3, Perspective3};

use crate::constants::{ASPECT_RATIO, FOV, NEAR_PLANE, FAR_PLANE, SCREEN_WIDTH, SCREEN_HEIGHT};

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

/* 
    Perspective3 produces a symmetric frustum identical to that used by OpenGL
    Perspective matrix :

    |  f / aspect  0                              0                                 0  |
    |  0           f                              0                                 0  |
    |  0           0   -(far + near) / (far - near)    -2 * far * near / (far - near)  |
    |  0           0                             -1                                 0  |

    where f = 1 / tan(fov / 2)
*/
pub fn get_projection_matrix() -> Matrix4<f32> {
    Perspective3::new(
        ASPECT_RATIO, 
        FOV, 
        NEAR_PLANE,
        FAR_PLANE)
        .to_homogeneous()
}

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

pub fn transform_triangle_to_camera_coords(triangle: &Triangle3, camera_transform: &Matrix3x4<f32>) 
        -> (Point3<f32>, Point3<f32>, Point3<f32>) {
    let world_v0 = triangle.geometry[0];
    let world_v1 = triangle.geometry[1];
    let world_v2 = triangle.geometry[2];

    let camera_v0: Point3<f32> = (camera_transform * world_v0.to_homogeneous()).xyz().into();
    let camera_v1: Point3<f32> = (camera_transform * world_v1.to_homogeneous()).xyz().into();
    let camera_v2: Point3<f32> = (camera_transform * world_v2.to_homogeneous()).xyz().into();

    (camera_v0, camera_v1, camera_v2)
}

pub fn camera_coordinates_to_clip_space(camera_vertex: &Point3<f32>, projection_matrix: &Matrix4<f32>) -> Point4<f32> {
    (projection_matrix * camera_vertex.to_homogeneous()).into()
}

pub fn clips_space_to_ndc(clip_space_vertex: &Point4<f32>) -> Point3<f32> {
    clip_space_vertex.xyz() / clip_space_vertex.w
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