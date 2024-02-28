use nalgebra::{Matrix4, Point2, Point3, Point4, Vector3};

use crate::world_objects::Entity;

//use rand::Rng;

pub struct Model {
    pub geometry: Vec<Triangle3>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Triangle3 {
    pub vertices: [Point3<f64>; 3],
    pub color: Color,
}

impl Triangle3 {
    pub fn vertices(&self) -> (&Point3<f64>, &Point3<f64>, &Point3<f64>) {
        (&self.vertices[0], &self.vertices[1], &self.vertices[2])
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Triangle4 {
    pub vertices: [Point4<f64>; 3],
    pub color: Color,
}

impl Triangle4 {
    pub fn vertices(&self) -> (&Point4<f64>, &Point4<f64>, &Point4<f64>) {
        (&self.vertices[0], &self.vertices[1], &self.vertices[2])
    }
}

#[derive(Copy, Clone)]
pub struct BoundingBox2 {
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
}

impl BoundingBox2 {
    pub fn get_screen_constrained_bounds(
        &self,
        screen_width: usize,
        screen_height: usize,
    ) -> (usize, usize, usize, usize) {
        (
            (self.x_min.floor() as usize).max(0),
            (self.y_min.floor() as usize).max(0),
            (self.x_max.ceil() as usize).min(screen_width),
            (self.y_max.ceil() as usize).min(screen_height),
        )
    }
}

#[derive(Copy, Clone)]
pub struct ProjectionResult {
    pub normal: Vector3<f64>,
    pub clip_space_triangle: Triangle4,
    pub ndc_triangle: Triangle3,
    pub screen_triangle: Triangle3,
    pub screen_bounding_box: BoundingBox2,
}

pub fn transform_entity_model(entity: &Entity) -> Vec<Triangle3> {
    let scale = entity.get_scale();
    let rotation = Matrix4::from(entity.get_rotation());
    let translation = Matrix4::new_translation(&entity.get_origin().coords);

    let transform = translation * rotation * scale;

    let transformed_triangles_vec: Vec<Triangle3> = entity
        .get_model_geometry()
        .iter()
        .map(|triangle| {
            let transformed_vertices = triangle
                .vertices
                .iter()
                .map(|vertex| transform.transform_point(vertex))
                .collect::<Vec<Point3<f64>>>();

            Triangle3 {
                vertices: [
                    transformed_vertices[0],
                    transformed_vertices[1],
                    transformed_vertices[2],
                ],
                color: entity.get_maybe_color().unwrap_or(triangle.color),
            }
        })
        .collect();

    transformed_triangles_vec
}

#[allow(dead_code)]
fn is_vertex_outside_frustum(vertex: &Point4<f64>) -> bool {
    let w = vertex.w;
    let x_out_of_range = vertex.x <= -w || vertex.x >= w;
    let y_out_of_range = vertex.y <= -w || vertex.y >= w;
    let z_out_of_range = vertex.z <= -w || vertex.z >= w;

    x_out_of_range || y_out_of_range || z_out_of_range
}

#[allow(dead_code)]
fn is_triangle_fully_outside_frustum(triangle: &Triangle4) -> bool {
    let (v0, v1, v2) = triangle.vertices();

    let lx = v0.x <= -v0.w && v1.x <= -v1.w && v2.x <= -v2.w;
    let gx = v0.x >= v0.w && v1.x >= v1.w && v2.x >= v2.w;
    let ly = v0.y <= -v0.w && v1.y <= -v1.w && v2.y <= -v2.w;
    let gy = v0.y >= v0.w && v1.y >= v1.w && v2.y >= v2.w;
    let lz = v0.z <= -v0.w && v1.z <= -v1.w && v2.z <= -v2.w;
    let gz = v0.z >= v0.w && v1.z >= v1.w && v2.z >= v2.w;

    lx || gx || ly || gy || lz || gz
}

#[derive(Copy, Clone)]
enum FrustumPlane {
    Near,
    Far,
    Left,
    Right,
    Top,
    Bottom,
}

fn calculate_clip_space_plane_intersection(
    plane: FrustumPlane,
    a: &Point4<f64>,
    b: &Point4<f64>,
) -> Point4<f64> {
    let alpha = match plane {
        FrustumPlane::Near => (-b.w - b.z) / (a.z + a.w - b.w - b.z),
        FrustumPlane::Far => (b.w - b.z) / (a.z - a.w + b.w - b.z),
        FrustumPlane::Left => (-b.w - b.x) / (a.x + a.w - b.w - b.x),
        FrustumPlane::Right => (b.w - b.x) / (a.x - a.w + b.w - b.x),
        FrustumPlane::Bottom => (-b.w - b.y) / (a.y + a.w - b.w - b.y),
        FrustumPlane::Top => (b.w - b.y) / (a.y - a.w + b.w - b.y),
    };
    let intersection = alpha * a.coords + (1.0 - alpha) * b.coords;
    Point4::from(intersection)
}

fn clip_triangle_against_plane(plane: FrustumPlane, triangles: &[Triangle4]) -> Vec<Triangle4> {
    triangles
        .iter()
        .flat_map(|triangle| {
            let vertices: Vec<(&Point4<f64>, bool)> = triangle
                .vertices
                .iter()
                .map(|v| match plane {
                    FrustumPlane::Near => (v, v.z >= -v.w),
                    FrustumPlane::Far => (v, v.z <= v.w),
                    FrustumPlane::Left => (v, v.x >= -v.w),
                    FrustumPlane::Right => (v, v.x <= v.w),
                    FrustumPlane::Bottom => (v, v.y >= -v.w),
                    FrustumPlane::Top => (v, v.y <= v.w),
                })
                .collect();

            let index = vertices
                .iter()
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

            let rcolor_1 = triangle.color;
            let rcolor_2 = triangle.color;
            /*
                let mut rng = rand::thread_rng();
                let rcolor_1 = Color::new(rng.gen(), rng.gen(), rng.gen());
                let rcolor_2 = Color::new(rng.gen(), rng.gen(), rng.gen());
            */

            match (b_inside, c_inside) {
                // Triangle is already fully within the near plane
                (true, true) => vec![*triangle],
                // Triangle is clipped into two triangles
                (true, false) => {
                    let i1 = calculate_clip_space_plane_intersection(plane, a.0, c.0);
                    let i2 = calculate_clip_space_plane_intersection(plane, b.0, c.0);
                    vec![
                        Triangle4 {
                            vertices: [*a.0, *b.0, i2],
                            color: rcolor_1,
                        },
                        Triangle4 {
                            vertices: [*a.0, i2, i1],
                            color: rcolor_2,
                        },
                    ]
                }
                // Triangle is clipped into two triangles
                (false, true) => {
                    let i1 = calculate_clip_space_plane_intersection(plane, a.0, b.0);
                    let i2 = calculate_clip_space_plane_intersection(plane, c.0, b.0);
                    vec![
                        Triangle4 {
                            vertices: [*a.0, i1, *c.0],
                            color: rcolor_1,
                        },
                        Triangle4 {
                            vertices: [*c.0, i1, i2],
                            color: rcolor_2,
                        },
                    ]
                }
                // Triangle is clipped into one smaller triangle
                (false, false) => {
                    let i1 = calculate_clip_space_plane_intersection(plane, a.0, b.0);
                    let i2 = calculate_clip_space_plane_intersection(plane, a.0, c.0);
                    vec![Triangle4 {
                        vertices: [*a.0, i1, i2],
                        color: rcolor_2,
                    }]
                }
            }
        })
        .collect()
}

fn clip_triangle_to_frustum(triangle: &Triangle4) -> Vec<Triangle4> {
    let near_clipped_triangles = clip_triangle_against_plane(FrustumPlane::Near, &[*triangle]);
    let far_clipped_triangles =
        clip_triangle_against_plane(FrustumPlane::Far, &near_clipped_triangles);
    let left_clipped_triangles =
        clip_triangle_against_plane(FrustumPlane::Left, &far_clipped_triangles);
    let right_clipped_triangles =
        clip_triangle_against_plane(FrustumPlane::Right, &left_clipped_triangles);
    let bottom_clipped_triangles =
        clip_triangle_against_plane(FrustumPlane::Bottom, &right_clipped_triangles);
    clip_triangle_against_plane(FrustumPlane::Top, &bottom_clipped_triangles)
}

pub fn is_point_in_triangle(pt: &Point2<f64>, triangle: &Triangle3) -> bool {
    let v1 = Point2::new(triangle.vertices[0].x, triangle.vertices[0].y);
    let v2 = Point2::new(triangle.vertices[1].x, triangle.vertices[1].y);
    let v3 = Point2::new(triangle.vertices[2].x, triangle.vertices[2].y);

    fn sign(p1: &Point2<f64>, p2: Point2<f64>, p3: Point2<f64>) -> f64 {
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
    point: &Point3<f64>,
    camera_transform: &Matrix4<f64>,
) -> Point3<f64> {
    let t = camera_transform * point.to_homogeneous();
    (t.xyz() / t.w).into()
}

#[allow(dead_code)]
fn transform_triangle_to_camera_coords(
    triangle: &Triangle3,
    camera_transform: &Matrix4<f64>,
) -> Triangle3 {
    let (world_v0, world_v1, world_v2) = triangle.vertices();

    let camera_v0 = transform_world_vertice_to_camera_coords(world_v0, camera_transform);
    let camera_v1 = transform_world_vertice_to_camera_coords(world_v1, camera_transform);
    let camera_v2 = transform_world_vertice_to_camera_coords(world_v2, camera_transform);

    Triangle3 {
        vertices: [camera_v0, camera_v1, camera_v2],
        color: triangle.color,
    }
}

#[allow(dead_code)]
fn camera_coordinates_to_clip_space(
    camera_triangle: &Triangle3,
    projection_matrix: &Matrix4<f64>,
) -> Triangle4 {
    let (camera_v0, camera_v1, camera_v2) = camera_triangle.vertices();

    let v0 = Point4::from(projection_matrix * camera_v0.to_homogeneous());
    let v1 = Point4::from(projection_matrix * camera_v1.to_homogeneous());
    let v2 = Point4::from(projection_matrix * camera_v2.to_homogeneous());

    Triangle4 {
        vertices: [v0, v1, v2],
        color: camera_triangle.color,
    }
}

fn transform_world_space_to_clip_space(
    world_triangle: &Triangle3,
    view_projection_matrix: &Matrix4<f64>,
) -> Triangle4 {
    let (world_v0, world_v1, world_v2) = world_triangle.vertices();

    let v0 = Point4::from(view_projection_matrix * world_v0.to_homogeneous());
    let v1 = Point4::from(view_projection_matrix * world_v1.to_homogeneous());
    let v2 = Point4::from(view_projection_matrix * world_v2.to_homogeneous());

    Triangle4 {
        vertices: [v0, v1, v2],
        color: world_triangle.color,
    }
}

fn clips_space_to_ndc(clip_space_triangle: &Triangle4) -> Triangle3 {
    let (clip_space_v0, clip_space_v1, clip_space_v2) = clip_space_triangle.vertices();
    let v0 = Point3::from(clip_space_v0.xyz() / clip_space_v0.w);
    let v1 = Point3::from(clip_space_v1.xyz() / clip_space_v1.w);
    let v2 = Point3::from(clip_space_v2.xyz() / clip_space_v2.w);
    Triangle3 {
        vertices: [v0, v1, v2],
        color: clip_space_triangle.color,
    }
}

fn ndc_to_screen(ndc_triangle: &Triangle3, screen_width: usize, screen_height: usize) -> Triangle3 {
    let transform = |ndc: &Point3<f64>| -> Point3<f64> {
        let px = (ndc.x + 1.0) / 2.0 * (screen_width as f64);
        let py = (1.0 - (ndc.y + 1.0) / 2.0) * (screen_height as f64);
        Point3::new(px, py, ndc.z)
    };
    let (ndc_v0, ndc_v1, ndc_v2) = ndc_triangle.vertices();
    let v0 = transform(ndc_v0);
    let v1 = transform(ndc_v1);
    let v2 = transform(ndc_v2);

    Triangle3 {
        vertices: [v0, v1, v2],
        color: ndc_triangle.color,
    }
}

pub fn screen_to_ndc(
    screen: &Point3<f64>,
    screen_width: usize,
    screen_height: usize,
) -> Point3<f64> {
    let x_ndc = (screen.x / (screen_width as f64)) * 2.0 - 1.0;
    let y_ndc = 1.0 - (screen.y / (screen_height as f64)) * 2.0;
    let z_ndc = screen.z;

    Point3::new(x_ndc, y_ndc, z_ndc)
}

fn calculate_triangle_normal(triangle: &Triangle3) -> Vector3<f64> {
    let (v0, v1, v2) = triangle.vertices();
    (v1 - v0).cross(&(v2 - v0)).normalize()
}

fn calculate_bounding_box(projected_triangle: &Triangle3) -> BoundingBox2 {
    let x_min = projected_triangle.vertices[0]
        .x
        .min(projected_triangle.vertices[1].x)
        .min(projected_triangle.vertices[2].x);
    let y_min = projected_triangle.vertices[0]
        .y
        .min(projected_triangle.vertices[1].y)
        .min(projected_triangle.vertices[2].y);
    let x_max = projected_triangle.vertices[0]
        .x
        .max(projected_triangle.vertices[1].x)
        .max(projected_triangle.vertices[2].x);
    let y_max = projected_triangle.vertices[0]
        .y
        .max(projected_triangle.vertices[1].y)
        .max(projected_triangle.vertices[2].y);

    BoundingBox2 {
        x_min,
        y_min,
        x_max,
        y_max,
    }
}

pub fn interpolate_attributes_at_pixel(
    p: &Point2<f64>,
    projection_result: &ProjectionResult,
) -> f64 {
    let (p0, p1, p2) = projection_result.screen_triangle.vertices();
    let (ndc_v0, ndc_v1, ndc_v2) = projection_result.ndc_triangle.vertices();

    let total_area: f64 = p0.x * (p1.y - p2.y) + p1.x * (p2.y - p0.y) + p2.x * (p0.y - p1.y);
    let lambda0: f64 = ((p1.y - p2.y) * (p.x - p2.x) + (p2.x - p1.x) * (p.y - p2.y)) / total_area;
    let lambda1: f64 = ((p2.y - p0.y) * (p.x - p2.x) + (p0.x - p2.x) * (p.y - p2.y)) / total_area;
    let lambda2: f64 = 1.0 - lambda0 - lambda1;

    assert!(lambda0 + lambda1 + lambda2 < 1.00001 && lambda0 + lambda1 + lambda2 > 0.99999);

    let iz0 = 1.0 / ndc_v0.z;
    let iz1 = 1.0 / ndc_v1.z;
    let iz2 = 1.0 / ndc_v2.z;

    // Interpolate z depth
    1.0 / (iz0 * lambda0 + iz1 * lambda1 + iz2 * lambda2)
}

pub fn project_triangle(
    input: &Triangle3,
    view_projection_matrix: &Matrix4<f64>,
    screen_width: usize,
    screen_height: usize,
) -> Vec<ProjectionResult> {
    // Calculate the normal in world coordinates
    // TODO: calculate these when applying the model matrix
    let normal = calculate_triangle_normal(input);

    // Transform the world triangle coordinates to clip space
    let clip_space_triangle = transform_world_space_to_clip_space(input, view_projection_matrix);

    // Clip the transformed triangles against the 6 frustum planes
    // Produces new triangle geometry if necessary
    let clipped_triangles: Vec<Triangle4> = clip_triangle_to_frustum(&clip_space_triangle);

    clipped_triangles
        .iter()
        .map(|clipped_triangle| {
            // Transform from clip space coordinates to normalized device coordinates
            let ndc_triangle = clips_space_to_ndc(clipped_triangle);

            // Transform from normalized device coordinates to screen coordinates
            let screen_triangle = ndc_to_screen(&ndc_triangle, screen_width, screen_height);

            // Get bounding box of the projected triangle
            let bounding_box = calculate_bounding_box(&screen_triangle);

            ProjectionResult {
                clip_space_triangle: *clipped_triangle,
                ndc_triangle,
                screen_triangle,
                screen_bounding_box: bounding_box,
                normal,
            }
        })
        .collect()
}
