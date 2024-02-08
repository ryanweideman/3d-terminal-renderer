use nalgebra::{Matrix3x4, Matrix4, Point2, Point3, Point4, Rotation3, Perspective3, Vector3};

use crate::constants::{ASPECT_RATIO, FOV, NEAR_PLANE, FAR_PLANE, SCREEN_WIDTH, SCREEN_HEIGHT};

#[derive(Copy, Clone)]
pub struct Color { 
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Copy, Clone)]
pub struct Triangle3 {
    pub vertices: [Point3<f32> ; 3],
    pub color: Color
}

impl Triangle3 {
    pub fn vertices(&self) -> (&Point3<f32>, &Point3<f32>, &Point3<f32>) {
        (&self.vertices[0], &self.vertices[1], &self.vertices[2])
    }
}

#[derive(Copy, Clone)]
pub struct Triangle4 {
    pub vertices: [Point4<f32> ; 3],
    pub color: Color
}

impl Triangle4 {
    pub fn vertices(&self) -> (&Point4<f32>, &Point4<f32>, &Point4<f32>) {
        (&self.vertices[0], &self.vertices[1], &self.vertices[2])
    }
}

pub struct BoundingBox2<T> {
    pub x_min: T,
    pub y_min: T,
    pub x_max: T,
    pub y_max: T
}

pub struct Cube {
    pub origin: Point3<f32>,
    pub rotation: Rotation3<f32>
}

pub struct PointLight {
    pub origin: Point3<f32>
}

pub struct ProjectionResult {
    pub camera_frame_triangle: Triangle3,
    pub normal: Vector3<f32>,
    pub clip_space_triangle: Triangle4, 
    pub ndc_triangle: Triangle3,
    pub screen_triangle: Triangle3,
    pub screen_bounding_box: BoundingBox2<usize>
}

const CUBE_TRIANGLES: [Triangle3; 12] = [
    // Front face
    Triangle3 { vertices: [Point3::new(-0.5, -0.5,  0.5), Point3::new( 0.5, -0.5,  0.5), Point3::new( 0.5,  0.5,  0.5)], color: Color {r: 255, g: 0, b: 0} },
    Triangle3 { vertices: [Point3::new(-0.5, -0.5,  0.5), Point3::new( 0.5,  0.5,  0.5), Point3::new(-0.5,  0.5,  0.5)], color: Color {r: 255, g: 0, b: 0} },
    
    // Back face
    Triangle3 { vertices: [Point3::new(-0.5, -0.5, -0.5), Point3::new( 0.5,  0.5, -0.5), Point3::new( 0.5, -0.5, -0.5)], color: Color {r: 0, g: 255, b: 0} },
    Triangle3 { vertices: [Point3::new(-0.5, -0.5, -0.5), Point3::new(-0.5,  0.5, -0.5), Point3::new( 0.5,  0.5, -0.5)], color: Color {r: 0, g: 255, b: 0} },
    
    // Right face
    Triangle3 { vertices: [Point3::new( 0.5, -0.5, -0.5), Point3::new( 0.5,  0.5,  0.5), Point3::new( 0.5, -0.5,  0.5)], color: Color {r: 0, g: 0, b: 255} },
    Triangle3 { vertices: [Point3::new( 0.5, -0.5, -0.5), Point3::new( 0.5,  0.5, -0.5), Point3::new( 0.5,  0.5,  0.5)], color: Color {r: 0, g: 0, b: 255} },
    
    // Left face
    Triangle3 { vertices: [Point3::new(-0.5, -0.5, -0.5), Point3::new(-0.5, -0.5,  0.5), Point3::new(-0.5,  0.5,  0.5)], color: Color {r: 255, g: 255, b: 0} },
    Triangle3 { vertices: [Point3::new(-0.5, -0.5, -0.5), Point3::new(-0.5,  0.5,  0.5), Point3::new(-0.5,  0.5, -0.5)], color: Color {r: 255, g: 255, b: 0} },
    
    // Top face
    Triangle3 { vertices: [Point3::new(-0.5,  0.5, -0.5), Point3::new(-0.5,  0.5,  0.5), Point3::new( 0.5,  0.5,  0.5)], color: Color {r: 255, g: 0, b: 255} },
    Triangle3 { vertices: [Point3::new(-0.5,  0.5, -0.5), Point3::new( 0.5,  0.5,  0.5), Point3::new( 0.5,  0.5, -0.5)], color: Color {r: 255, g: 0, b: 255} },
    
    // Bottom face
    Triangle3 { vertices: [Point3::new(-0.5, -0.5, -0.5), Point3::new( 0.5, -0.5, -0.5), Point3::new( 0.5, -0.5,  0.5)], color: Color {r: 0, g: 255, b: 255} },
    Triangle3 { vertices: [Point3::new(-0.5, -0.5, -0.5), Point3::new( 0.5, -0.5,  0.5), Point3::new(-0.5, -0.5,  0.5)], color: Color {r: 0, g: 255, b: 255} },
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

    // First rotate the vertices around it's origin (model space), then translate it to the desired position (world space)
    let transform = translation * rotation;

    let transformed_triangles_vec: Vec<Triangle3> = CUBE_TRIANGLES.iter().map(|triangle| {
        let transformed_vertices = triangle.vertices.iter().map(|vertex| {
            transform.transform_point(&vertex)
        }).collect::<Vec<Point3<f32>>>();
        
        Triangle3 {
            vertices: [transformed_vertices[0], transformed_vertices[1], transformed_vertices[2]],
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
fn is_vertex_outside_frustum(ndc : &Point3<f32>) -> bool {
    let x_out_of_range = ndc.x < -1.0 || ndc.x > 1.0;
    let y_out_of_range = ndc.y < -1.0 || ndc.y > 1.0;
    let z_out_of_range = ndc.z < -1.0 || ndc.z > 1.0;

    x_out_of_range || y_out_of_range || z_out_of_range
}

pub fn is_point_in_triangle(pt: &Point2<f32>, triangle: &Triangle3) -> bool {
    let v1 = Point2::new(triangle.vertices[0].x, triangle.vertices[0].y);
    let v2 = Point2::new(triangle.vertices[1].x, triangle.vertices[1].y);
    let v3 = Point2::new(triangle.vertices[2].x, triangle.vertices[2].y);

    fn sign(p1: &Point2<f32>, p2: Point2<f32>, p3: Point2<f32>) -> f32 {
        (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    }

    let d1 = sign(pt, v1, v2);
    let d2 = sign(pt, v2, v3);
    let d3 = sign(pt, v3, v1);

    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;

    !(has_neg && has_pos)
}

fn transform_triangle_to_camera_coords(triangle: &Triangle3, camera_transform: &Matrix3x4<f32>) 
        -> Triangle3 {
    let (world_v0, world_v1, world_v2) = triangle.vertices();

    let camera_v0: Point3<f32> = (camera_transform * world_v0.to_homogeneous()).xyz().into();
    let camera_v1: Point3<f32> = (camera_transform * world_v1.to_homogeneous()).xyz().into();
    let camera_v2: Point3<f32> = (camera_transform * world_v2.to_homogeneous()).xyz().into();

    Triangle3 {
        vertices : [camera_v0, camera_v1, camera_v2],
        color: triangle.color
    }
}

fn camera_coordinates_to_clip_space(camera_triangle: &Triangle3, projection_matrix: &Matrix4<f32>) -> Triangle4 {
    let (camera_v0, camera_v1, camera_v2) = camera_triangle.vertices();
    
    let v0 : Point4<f32> = (projection_matrix * camera_v0.to_homogeneous()).into();
    let v1 : Point4<f32> = (projection_matrix * camera_v1.to_homogeneous()).into();
    let v2 : Point4<f32> = (projection_matrix * camera_v2.to_homogeneous()).into();

    Triangle4 {
        vertices : [v0, v1, v2],
        color: camera_triangle.color
    }
}

fn clips_space_to_ndc(clip_space_triangle: &Triangle4) -> Triangle3 {
    let (clip_space_v0, clip_space_v1, clip_space_v2) = clip_space_triangle.vertices();
    let v0 : Point3<f32> = (clip_space_v0.xyz() / clip_space_v0.w).into();
    let v1 : Point3<f32> = (clip_space_v1.xyz() / clip_space_v1.w).into();
    let v2 : Point3<f32> = (clip_space_v2.xyz() / clip_space_v2.w).into();
    Triangle3 {
        vertices: [v0, v1, v2],
        color: clip_space_triangle.color
    }
}

fn ndc_to_screen(ndc_triangle : &Triangle3) -> Triangle3 {
    let transform = |ndc: &Point3<f32>| -> Point3<f32> {
        let px = (ndc.x + 1.0) / 2.0 * (SCREEN_WIDTH as f32);
        let py = (ndc.y + 1.0) / 2.0 * (SCREEN_HEIGHT as f32);
        Point3::new(px, py, ndc.z)
    };
    let (ndc_v0, ndc_v1, ndc_v2) = ndc_triangle.vertices();
    let v0 = transform(ndc_v0);
    let v1 = transform(ndc_v1);
    let v2 = transform(ndc_v2);

    Triangle3 {
        vertices: [v0, v1, v2],
        color: ndc_triangle.color
    }
}

pub fn screen_to_ndc(screen: &Point3<f32>) -> Point3<f32> {
    let x_ndc = (screen.x / (SCREEN_WIDTH as f32)) * 2.0 - 1.0;
    let y_ndc = (screen.y / (SCREEN_HEIGHT as f32)) * 2.0 - 1.0;
    let z_ndc = screen.z; 

    Point3::new(x_ndc, y_ndc, z_ndc)
}

fn calculate_triangle_normal(triangle: &Triangle3) -> Vector3<f32> {
    let (v0, v1, v2) = triangle.vertices();
    (v1 - v0).cross(&(v2 - v0)).normalize() 
}

fn calculate_bounding_box(projected_triangle : &Triangle3) -> BoundingBox2<usize> {
    let x_min = projected_triangle.vertices[0].x
        .min(projected_triangle.vertices[1].x)
        .min(projected_triangle.vertices[2].x)
        .floor() as usize;
    let y_min = projected_triangle.vertices[0].y
        .min(projected_triangle.vertices[1].y)
        .min(projected_triangle.vertices[2].y)
        .floor() as usize;
    let x_max = projected_triangle.vertices[0].x
        .max(projected_triangle.vertices[1].x)
        .max(projected_triangle.vertices[2].x)
        .ceil() as usize;
    let y_max = projected_triangle.vertices[0].y
        .max(projected_triangle.vertices[1].y)
        .max(projected_triangle.vertices[2].y)
        .ceil() as usize;

    BoundingBox2 {
        x_min: x_min,
        y_min: y_min,
        x_max: x_max,
        y_max: y_max
    }
}

pub fn project_triangle(
        input : &Triangle3, 
        projection_matrix : &Matrix4<f32>,  
        camera_transform : &Matrix3x4<f32>) -> Option<ProjectionResult> {
    // Transform world coordinates to camera coordinates
    let camera_frame_triangle = transform_triangle_to_camera_coords(&input, &camera_transform);

    // Calculate the normal in camera coordinates
    let normal = calculate_triangle_normal(&camera_frame_triangle);

    // Transform the camera coordinates to clip space
    let clip_space_triangle = camera_coordinates_to_clip_space(&camera_frame_triangle, &projection_matrix);

    // Transform from clip space coordinates to normalized device coordinates
    let ndc_triangle = clips_space_to_ndc(&clip_space_triangle);

    // Transform from normalized device coordinates to screen coordinates
    let screen_triangle = ndc_to_screen(&ndc_triangle);

    // Get bounding box of the projected triangle
    let bounding_box = calculate_bounding_box(&screen_triangle);

    let result = ProjectionResult {
        camera_frame_triangle: camera_frame_triangle,
        clip_space_triangle: clip_space_triangle,
        ndc_triangle: ndc_triangle,
        screen_triangle: screen_triangle,
        screen_bounding_box: bounding_box,
        normal: normal
    };

    Some(result)
}