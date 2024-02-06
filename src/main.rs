extern crate nalgebra as na;
use na::{Vector3, Matrix4, Matrix3x4, Perspective3, Point2, Point3, Point4, Rotation3, Unit};

use std::{time};

const ASPECT_RATIO : f32 = 4.0 / 3.0;
const SCREEN_WIDTH : usize = 54;
const SCREEN_HEIGHT : usize = ((SCREEN_WIDTH as f32) / ASPECT_RATIO) as usize;

#[derive(Copy, Clone)]
struct Color { 
    r: u8,
    g: u8,
    b: u8
}

#[derive(Copy, Clone)]
struct Triangle3 {
    geometry: [Point3<f32> ; 3],
    color: Color
}

struct Cube {
    origin: Point3<f32>,
    rotation: Rotation3<f32>
}

const CUBE_TRIANGLES: [Triangle3; 12] = [
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

struct PointLight {
    origin: Point3<f32>
}

fn get_cube_geometry(cube: &Cube) -> [Triangle3; 12] {
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

fn rgb_channel_to_ansi_index(v: u8) -> u8 {
    // the ansi rgb values are on the scale 0-5
    // 0-95 map to 0, 95-255 map to 1-5
    if v < 95 {
        return 0;
    }
    1 + (v - 95) / 40
}

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u16 {
    let rc = rgb_channel_to_ansi_index(r);
    let gc = rgb_channel_to_ansi_index(g);
    let bc = rgb_channel_to_ansi_index(b);

    // Uses finer grayscale. Ignores 0 case since the deadzone is massive
    /*
    if rc != 0 && rc == gc && gc == bc {
        return 232 + ((r as f32) * 0.09375) as u16;
    }
    */

    (16 + 36 * rc + 6 * gc + bc).into()
}

fn sign(p1: &Point2<f32>, p2: Point2<f32>, p3: Point2<f32>) -> f32 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}

fn is_point_in_triangle(pt: &Point2<f32>, triangle: &Triangle3) -> bool {
    let v1 = Point2::new(triangle.geometry[0].x, triangle.geometry[0].y);
    let v2 = Point2::new(triangle.geometry[1].x, triangle.geometry[1].y);
    let v3 = Point2::new(triangle.geometry[2].x, triangle.geometry[2].y);

    let d1 = sign(pt, v1, v2);
    let d2 = sign(pt, v2, v3);
    let d3 = sign(pt, v3, v1);

    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;

    !(has_neg && has_pos)
}

fn clear_screen() {
    println!("\x1b[H\x1b[J");
}

/*
            // Cull triangles fully outside of frustum
            if is_vertex_outside_frustum(&ndc_v0) && is_vertex_outside_frustum(&ndc_v1) && is_vertex_outside_frustum(&ndc_v2) {
                continue;
            }

fn is_vertex_outside_frustum(ndc : &Point3<f32>) -> bool {
    let x_out_of_range = ndc.x < -1.0 || ndc.x > 1.0;
    let y_out_of_range = ndc.y < -1.0 || ndc.y > 1.0;
    let z_out_of_range = ndc.z < -1.0 || ndc.z > 1.0;

    x_out_of_range || y_out_of_range || z_out_of_range
}
*/

fn ndc_to_screen(ndc : &Point3<f32>) -> Point3<f32> {
    let px = (ndc.x + 1.0) / 2.0 * (SCREEN_WIDTH as f32);
    let py = (ndc.y + 1.0) / 2.0 * (SCREEN_HEIGHT as f32);
    Point3::new(px, py, ndc.z)
}

fn screen_to_ndc(screen: &Point3<f32>) -> Point3<f32> {
    let x_ndc = (screen.x / (SCREEN_WIDTH as f32)) * 2.0 - 1.0;
    let y_ndc = (screen.y / (SCREEN_HEIGHT as f32)) * 2.0 - 1.0;
    let z_ndc = screen.z; 

    Point3::new(x_ndc, y_ndc, z_ndc)
}

fn calculate_bounding_box(projected_triangle : &Triangle3) -> (usize, usize, usize, usize) {
    let minx = projected_triangle.geometry[0].x
        .min(projected_triangle.geometry[1].x)
        .min(projected_triangle.geometry[2].x)
        .max(0.0)
        .floor() as usize;
    let miny = projected_triangle.geometry[0].y
        .min(projected_triangle.geometry[1].y)
        .min(projected_triangle.geometry[2].y)
        .max(0.0)
        .floor() as usize;
    let maxx = projected_triangle.geometry[0].x
        .max(projected_triangle.geometry[1].x)
        .max(projected_triangle.geometry[2].x)
        .min(SCREEN_WIDTH as f32)
        .ceil() as usize;
    let maxy = projected_triangle.geometry[0].y
        .max(projected_triangle.geometry[1].y)
        .max(projected_triangle.geometry[2].y)
        .min(SCREEN_HEIGHT as f32)
        .ceil() as usize;

    (minx, miny, maxx, maxy)
}

fn interpolate_attributes_at_pixel(
        p  : &Point2<f32>,
        v0 : &Point3<f32>, 
        v1 : &Point3<f32>, 
        v2 : &Point3<f32>,
        clip_v0 : &Point4<f32>,
        clip_v1 : &Point4<f32>,
        clip_v2 : &Point4<f32>,
        ndc0 : &Point3<f32>, 
        ndc1 : &Point3<f32>, 
        ndc2 : &Point3<f32>) 
        -> (f32, f32) {

    let total_area : f32 = v0.x * (v1.y - v2.y) + v1.x * (v2.y - v0.y) + v2.x * (v0.y - v1.y);
    let lambda0 : f32 = ((v1.y - v2.y) * (p.x - v2.x) + (v2.x - v1.x) * (p.y - v2.y)) / total_area;
    let lambda1 : f32 = ((v2.y - v0.y) * (p.x - v2.x) + (v0.x - v2.x) * (p.y - v2.y)) / total_area;
    let lambda2 : f32 = 1.0 - lambda0 - lambda1;
 
    assert!(lambda0 + lambda1 + lambda2 < 1.00001 
        && lambda0 + lambda1 + lambda2 > 0.99999);

    let wp0 = 1.0 / clip_v0.w;
    let wp1 = 1.0 / clip_v1.w;
    let wp2 = 1.0 / clip_v2.w;

    let den = wp0 * lambda0 + wp1 * lambda1 + wp2 * lambda2;
    let lambdap0 = lambda0 * wp0 / den;
    let lambdap1 = lambda1 * wp1 / den;
    let lambdap2 = lambda2 * wp2 / den;

    let z = ndc0.z * lambdap0 + ndc1.z * lambdap1 + ndc2.z * lambdap2;
    let w = 1.0 / den;
    (z, w)
}

fn output_screen_buffer(screen_buffer : &[[u16; SCREEN_WIDTH] ; SCREEN_HEIGHT]) {
    print!("  ");
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            print!("\x1b[48;5;{}m  \x1b[m", screen_buffer[y][x]);
        }
        print!("\n  ");
    }
}

fn round_up_to_nearest_increment(value: f32, increment: f32) -> f32 {
    let scaled = value / increment;
    let rounded = scaled.ceil();
    rounded * increment
}

fn scale_range(x: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    assert!(x >= old_min && x <= old_max, "x must be within [-1, 1]");

    new_min + ((x - old_min) / (old_max - old_min)) * (new_max - new_min)
}

fn main() {
    let mut start_time = time::Instant::now();
    let delay_duration = time::Duration::from_millis(100);
    let ansi_background_color = rgb_to_ansi256(100, 100, 100);

    // Assume camera is fixed at origin, for now
    let camera_transform : Matrix3x4<f32> = Matrix3x4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0
    );

    /* 
        Perspective3 produces a symmetric frustum identical to that used by OpenGL
        Perspective matrix :

        |  f / aspect  0                              0                                 0  |
        |  0           f                              0                                 0  |
        |  0           0   -(far + near) / (far - near)    -2 * far * near / (far - near)  |
        |  0           0                             -1                                 0  |

        where f = 1 / tan(fov / 2)
    */
    let near_plane = 0.1;
    let far_plane = 1000.0;
    let fovy = std::f32::consts::PI / 3.0;
    let projection_matrix : Matrix4<f32> = Perspective3::new(ASPECT_RATIO, fovy, near_plane, far_plane).to_homogeneous();
    let projection_matrix_inverse = projection_matrix.try_inverse().unwrap();

    let mut theta : f32 = 0.0;

    let point_light = PointLight {
        origin: Point3::new(2.0, -2.0, 3.0)
    };

    loop {
        if start_time.elapsed() < delay_duration {
            continue;
        }

        start_time = time::Instant::now();
        clear_screen();

        let mut screen_buffer : [[u16; SCREEN_WIDTH] ; SCREEN_HEIGHT] 
            = [[ansi_background_color ; SCREEN_WIDTH] ; SCREEN_HEIGHT]; 
        let mut z_buffer : [[f32; SCREEN_WIDTH] ; SCREEN_HEIGHT] 
            = [[f32::MAX ; SCREEN_WIDTH] ; SCREEN_HEIGHT]; 

        theta -= 0.15;
        //theta = std::f32::consts::PI / 4.0 + 0.1;
        //theta = 0.5;

        // Define the rotation using Rotation3
        let rotation_axis = Unit::new_normalize(Vector3::new(1.7, 3.0, 0.0)); // Rotate around the Y axis
        //let rotation_axis = Vector3::y_axis(); // Rotate around the Y axis

        let rotation3 = Rotation3::from_axis_angle(&rotation_axis, theta);
        let cube = Cube {
            //origin : Point3::new(0.0, 0.0, -3.0 + f32::sin(phi)),
            origin : Point3::new(0.0, 0.0, -3.0),
            rotation : rotation3
        };

        let geometry : [Triangle3; 12] = get_cube_geometry(&cube);

        for triangle in &geometry {
            // world cords -> camera coords -> ndc -> screen coords

            // Get world cordaintes 
            let world_v0 = triangle.geometry[0];
            let world_v1 = triangle.geometry[1];
            let world_v2 = triangle.geometry[2];
        
            // Transform world coordinates to camera coordinates
            let camera_v0 : Point3<f32> = (camera_transform * world_v0.to_homogeneous()).into();
            let camera_v1 : Point3<f32> = (camera_transform * world_v1.to_homogeneous()).into();
            let camera_v2 : Point3<f32> = (camera_transform * world_v2.to_homogeneous()).into();
        
            // Transform camera coordinates to clip space coordinates
            let clip_space_v0 : Point4<f32> = (projection_matrix * camera_v0.to_homogeneous()).into();
            let clip_space_v1 : Point4<f32> = (projection_matrix * camera_v1.to_homogeneous()).into();
            let clip_space_v2 : Point4<f32> = (projection_matrix * camera_v2.to_homogeneous()).into();

            // Transform from clip space coordinates to normlized device coordinates
            let ndc_v0 : Point3<f32> = clip_space_v0.xyz() / clip_space_v0.w;
            let ndc_v1 : Point3<f32> = clip_space_v1.xyz() / clip_space_v1.w;
            let ndc_v2 : Point3<f32> = clip_space_v2.xyz() / clip_space_v2.w;

            // Transform from normalized device coordinates to screen coordinates
            let screen_v0 = ndc_to_screen(&ndc_v0);
            let screen_v1 = ndc_to_screen(&ndc_v1);
            let screen_v2 = ndc_to_screen(&ndc_v2);

            let projected_triangle = Triangle3 {
                geometry: [
                    screen_v0,
                    screen_v1,
                    screen_v2
                ],
                color: triangle.color
            };

            // Get bounding box of the projected triangle
            let (minx, miny, maxx, maxy) = calculate_bounding_box(&projected_triangle);

            // Rasterize
            for y in miny..maxy {
                for x in minx..maxx {
                    let px = (x as f32) + 0.5;
                    let py = (y as f32) + 0.5;
                    let pixel = Point2::new(px, py);

                    if !is_point_in_triangle(&pixel, &projected_triangle) {
                        continue;
                    }

                    let (z, _w) = interpolate_attributes_at_pixel(
                        &pixel, &screen_v0, &screen_v1, &screen_v2, 
                        &clip_space_v0, &clip_space_v1, &clip_space_v2, 
                        &ndc_v0, &ndc_v1, &ndc_v2);

                    // pixel in this triangle is behind another triangle
                    if z >= z_buffer[y][x] {
                        continue;
                    }

                    let p_ndc = screen_to_ndc(&Point3::new(px, py, z)).to_homogeneous();
                    let point_camera_space_homogeneous = projection_matrix_inverse * p_ndc;
                    let point_camera_space : Point3<f32> = (point_camera_space_homogeneous.xyz() / point_camera_space_homogeneous.w).into();
                    
                    // calculate normal
                    let triangle_norm = (camera_v1 - camera_v0).cross(&(camera_v2 - camera_v0)).normalize();
                    // Transform the point light into camera coordinates
                    let camera_point_light : Point3<f32> = (camera_transform * point_light.origin.to_homogeneous()).into();
                    let light_norm = (camera_point_light - point_camera_space).normalize();

                    
                    let mut dot = light_norm.dot(&triangle_norm).max(0.0);

                   // dot = scale_range(dot, 0.0, 1.0, 0.5, 1.0);
                    dot = round_up_to_nearest_increment(dot, 0.2);

                    let mut r = triangle.color.r;
                    if r != 0 {
                        //r = r.max(95); 
                        r = scale_range((r as f32) * dot, 0.0, 255.0, 95.0, 255.0) as u8;
                        //r = (95.0 + ((r as f32) - 95.0) * dot) as u8
                    }
                    let mut g = triangle.color.g;
                    if g != 0 {
                        //g = g.max(95); 
                        g = scale_range((g as f32) * dot, 0.0, 255.0, 95.0, 255.0) as u8;

                        //g = (95.0 + ((g as f32) - 95.0) * dot) as u8
                    }
                    let mut b = triangle.color.b;
                    if b != 0 {
                        //b = b.max(95);
                        b = scale_range((b as f32) * dot, 0.0, 255.0, 95.0, 255.0) as u8;

                        //b = (95.0 + ((b as f32) - 95.0) * dot) as u8
                    }

                    //println!("{} {} {} {}", dot, r, g, b);

                    screen_buffer[y][x] = rgb_to_ansi256(r, g, b);
                    z_buffer[y][x] = z;
                }
            }
        }

        output_screen_buffer(&screen_buffer);

        let n = (time::Instant::now() - start_time).as_nanos() as f32;
        println!("time elapsed ms: {:.2}", n / 1000000.0);
    }
} 
