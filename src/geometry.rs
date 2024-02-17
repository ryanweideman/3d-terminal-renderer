use nalgebra::{Matrix4, Point2, Point3, Point4, Perspective3, Vector3, Vector4};

use crate::constants::{ASPECT_RATIO, FOV, NEAR_PLANE, FAR_PLANE, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::world_objects::{Entity};

#[derive(Clone)]
pub struct Model {
    pub geometry: Vec<Triangle3>
}

#[derive(Copy, Clone)]
pub struct Color { 
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r, g, b}
    }
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

#[derive(Copy, Clone)]
pub struct BoundingBox2<T> {
    pub x_min: T,
    pub y_min: T,
    pub x_max: T,
    pub y_max: T
}

#[derive(Copy, Clone)]
pub struct ProjectionResult {
    pub camera_frame_triangle: Triangle3,
    pub normal: Vector3<f32>,
    pub clip_space_triangle: Triangle4, 
    pub ndc_triangle: Triangle3,
    pub screen_triangle: Triangle3,
    pub screen_bounding_box: BoundingBox2<usize>
}

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

pub fn transform_entity_model(entity: &Entity) -> Vec<Triangle3> {
    let scale = entity.get_scale();
    let rotation = Matrix4::from(entity.get_rotation().clone());
    let translation = Matrix4::new_translation(&entity.get_origin().coords);

    let transform = translation * rotation * scale;

    let transformed_triangles_vec: Vec<Triangle3> = entity.get_model_geometry().iter().map(|triangle| {
        let transformed_vertices = triangle.vertices.iter().map(|vertex| {
            transform.transform_point(&vertex)
        }).collect::<Vec<Point3<f32>>>();
        
        Triangle3 {
            vertices: [transformed_vertices[0], transformed_vertices[1], transformed_vertices[2]],
            color: entity.get_maybe_color().unwrap_or_else(|| triangle.color.clone())
        }
    }).collect();

    transformed_triangles_vec
}

#[allow(dead_code)]
fn is_vertex_outside_frustum(vertex : &Point4<f32>) -> bool {
    let w = vertex.w;
    let x_out_of_range = vertex.x <= -w || vertex.x >= w;
    let y_out_of_range = vertex.y <= -w || vertex.y >= w;
    let z_out_of_range = vertex.z <= -w || vertex.z >= w;

    x_out_of_range || y_out_of_range || z_out_of_range
}

fn is_triangle_fully_outside_frustum(triangle : &Triangle4) -> bool {
    let (v0, v1, v2) = triangle.vertices();

    let lx = v0.x <= -v0.w && v1.x <= -v1.w && v2.x <= -v2.w;
    let gx = v0.x >=  v0.w && v1.x >=  v1.w && v2.x >=  v2.w;
    let ly = v0.y <= -v0.w && v1.y <= -v1.w && v2.y <= -v2.w;
    let gy = v0.y >=  v0.w && v1.y >=  v1.w && v2.y >=  v2.w;
    let lz = v0.z <= -v0.w && v1.z <= -v1.w && v2.z <= -v2.w;
    let gz = v0.z >=  v0.w && v1.z >=  v1.w && v2.z >=  v2.w;

    lx || gx || ly || gy || lz || gz
}

fn calculate_clip_space_near_plane_intersection(a: &Point4<f32>, b: &Point4<f32>) -> Point4<f32> {
    let alpha = (-b.w  - b.z) / (a.z + a.w - b.w - b.z);
    let a_vec: Vector4<f32> = a.coords;
    let b_vec: Vector4<f32> = b.coords;
    let intersection = alpha * a_vec + (1.0 - alpha) * b_vec;
    Point4::from(intersection)
}

fn clip_triangle(triangle: &Triangle4) -> Vec<Triangle4> {
    if is_triangle_fully_outside_frustum(triangle) {
        return Vec::new();
    }

    let vertices: Vec<(&Point4<f32>, bool)> = triangle.vertices.iter()
        .map(|v| (v, v.z > -v.w))
        .collect();

    let index = vertices.iter()
        .position(|&(_, inside)| inside)
        .unwrap_or(usize::MAX);

    // None of the vertices are within the near plane
    if index == usize::MAX {
        return Vec::new();
    }

     // Vertice A is gauranteed within the near plane
    let a = vertices[index];
    let b = vertices[(index + 1) % 3];
    let c = vertices[(index + 2) % 3];

    let b_inside: bool = b.1;
    let c_inside: bool = c.1;

    match (b_inside, c_inside) {
        // Triangle is already fully within the near plane
        (true, true) => vec![*triangle],
        // Triangle is clipped into two triangles
        (true, false) => {
            let i1 = calculate_clip_space_near_plane_intersection(a.0, c.0);
            let i2 = calculate_clip_space_near_plane_intersection(b.0, c.0);
            vec![
                Triangle4 { vertices: [*a.0, *b.0, i1], color: triangle.color }, 
                Triangle4 { vertices: [*b.0, i2, i1], color: triangle.color }
            ]
        }
        // Triangle is clipped into two triangles
        (false, true) => {
            let i1 = calculate_clip_space_near_plane_intersection(a.0, b.0);
            let i2 = calculate_clip_space_near_plane_intersection(b.0, c.0);
            vec![
                Triangle4 { vertices: [*a.0, i1, *c.0], color: triangle.color }, 
                Triangle4 { vertices: [*c.0, i1, i2], color: triangle.color }
            ]      
        }
        // Triangle is clipped into one smaller triangle
        (false, false) => {
            let i1 = calculate_clip_space_near_plane_intersection(a.0, b.0);
            let i2 = calculate_clip_space_near_plane_intersection(a.0, c.0);
            vec![Triangle4 { vertices: [*a.0, i1, i2], color: triangle.color }]
        }
    }
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

pub fn transform_world_vertice_to_camera_coords(
        point: &Point3<f32>, 
        camera_transform: &Matrix4<f32>) -> Point3<f32> {
    let t = camera_transform * point.to_homogeneous();
    (t.xyz() / t.w).into()
}

fn transform_triangle_to_camera_coords(triangle: &Triangle3, camera_transform: &Matrix4<f32>) 
        -> Triangle3 {
    let (world_v0, world_v1, world_v2) = triangle.vertices();

    let camera_v0 = transform_world_vertice_to_camera_coords(world_v0, camera_transform);
    let camera_v1 = transform_world_vertice_to_camera_coords(world_v1, camera_transform);
    let camera_v2 = transform_world_vertice_to_camera_coords(world_v2, camera_transform);

    Triangle3 {
        vertices: [camera_v0, camera_v1, camera_v2],
        color: triangle.color
    }
}

fn camera_coordinates_to_clip_space(camera_triangle: &Triangle3, projection_matrix: &Matrix4<f32>) -> Triangle4 {
    let (camera_v0, camera_v1, camera_v2) = camera_triangle.vertices();
    
    let v0 = Point4::from(projection_matrix * camera_v0.to_homogeneous());
    let v1 = Point4::from(projection_matrix * camera_v1.to_homogeneous());
    let v2 = Point4::from(projection_matrix * camera_v2.to_homogeneous());

    Triangle4 {
        vertices : [v0, v1, v2],
        color: camera_triangle.color
    }
}

fn clips_space_to_ndc(clip_space_triangle: &Triangle4) -> Triangle3 {
    let (clip_space_v0, clip_space_v1, clip_space_v2) = clip_space_triangle.vertices();
    let v0 = Point3::from(clip_space_v0.xyz() / clip_space_v0.w);
    let v1 = Point3::from(clip_space_v1.xyz() / clip_space_v1.w);
    let v2 = Point3::from(clip_space_v2.xyz() / clip_space_v2.w);
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
        camera_transform : &Matrix4<f32>) -> Vec<ProjectionResult> {
    // Transform world coordinates to camera coordinates
    let camera_frame_triangle = transform_triangle_to_camera_coords(&input, &camera_transform);

    // Calculate the normal in camera coordinates
    let normal = calculate_triangle_normal(&camera_frame_triangle);

    // Transform the camera coordinates to clip space
    let clip_space_triangle = camera_coordinates_to_clip_space(&camera_frame_triangle, &projection_matrix);

    let clipped_triangles : Vec<Triangle4> = clip_triangle(&clip_space_triangle);

    clipped_triangles.iter()
        .map(|clipped_triangle| {
            // Transform from clip space coordinates to normalized device coordinates
            let ndc_triangle = clips_space_to_ndc(clipped_triangle);

            // Transform from normalized device coordinates to screen coordinates
            let screen_triangle = ndc_to_screen(&ndc_triangle);

            // Get bounding box of the projected triangle
            let bounding_box = calculate_bounding_box(&screen_triangle);

            let result = ProjectionResult {
                camera_frame_triangle: camera_frame_triangle,
                clip_space_triangle: *clipped_triangle,
                ndc_triangle: ndc_triangle,
                screen_triangle: screen_triangle,
                screen_bounding_box: bounding_box,
                normal: normal
            };

            result
        }).collect()
}