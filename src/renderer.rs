use nalgebra::{Matrix4, Point2, Point3, Vector3};

use crate::buffer::Buffer;
use crate::camera::Camera;
use crate::geometry;
use crate::world_objects::Light;

pub fn render_geometry(
    screen_buffer: &mut Buffer<[u8; 3]>,
    geometry: &Vec<geometry::Triangle3>,
    world_lights: &[Light],
    camera: &Camera,
    background_color: [u8; 3],
) -> Vec<geometry::ProjectionResult> {
    let view_projection_matrix: Matrix4<f64> = camera.get_view_projection_matrix();
    let inverse_view_projection_matrix = view_projection_matrix.try_inverse().unwrap();

    let screen_width = screen_buffer.width;
    let screen_height = screen_buffer.height;
    let mut z_buffer = Buffer::<f64>::new(f64::MAX, screen_width, screen_height);
    let mut projection_buffer = Buffer::<usize>::new(usize::MAX, screen_width, screen_height);

    let mut cached_projection_results = Vec::with_capacity(geometry.len());

    for triangle in geometry {
        // world cords -> camera coords -> ndc -> screen coords
        let projection_results = geometry::project_triangle(
            triangle,
            &view_projection_matrix,
            screen_width,
            screen_height,
        );

        for projection_result in &projection_results {
            cached_projection_results.push(*projection_result);
            let projection_result_index = cached_projection_results.len() - 1;

            let (x_min, y_min, x_max, y_max) = projection_result
                .screen_bounding_box
                .get_screen_constrained_bounds(screen_width, screen_height);

            // Rasterize
            for y in y_min..y_max {
                for x in x_min..x_max {
                    let px = (x as f64) + 0.5;
                    let py = (y as f64) + 0.5;
                    let pixel = Point2::new(px, py);

                    if !geometry::is_point_in_triangle(&pixel, &projection_result.screen_triangle) {
                        continue;
                    }

                    let z = geometry::interpolate_attributes_at_pixel(&pixel, projection_result);

                    // pixel in this triangle is the closest to the camera
                    if z < z_buffer[y][x] {
                        z_buffer[y][x] = z;
                        projection_buffer[y][x] = projection_result_index;
                    }
                }
            }
        }
    }

    for y in 0..screen_height {
        for x in 0..screen_width {
            let projection_result_index = projection_buffer[y][x];
            if projection_result_index == usize::MAX {
                screen_buffer[y][x] = background_color;
                continue;
            }
            let projection_result = &cached_projection_results[projection_result_index];

            let pixel = Point3::new((x as f64) + 0.5, (y as f64) + 0.5, z_buffer[y][x]);

            let light_intensity = calculate_pixel_lighting(
                &pixel,
                &projection_result.normal,
                world_lights,
                &inverse_view_projection_matrix,
                screen_width,
                screen_height,
            );

            let color = projection_result.screen_triangle.color;
            let r = ((color.r as f64) * light_intensity) as u8;
            let g = ((color.g as f64) * light_intensity) as u8;
            let b = ((color.b as f64) * light_intensity) as u8;

            screen_buffer[y][x] = [r, g, b];
        }
    }

    cached_projection_results.clone()
}

fn calculate_pixel_lighting(
    pixel: &Point3<f64>,
    normal: &Vector3<f64>,
    world_lights: &[Light],
    inverse_view_projection_matrix: &Matrix4<f64>,
    screen_width: usize,
    screen_height: usize,
) -> f64 {
    let p_ndc = geometry::screen_to_ndc(pixel, screen_width, screen_height).to_homogeneous();
    let point_world_space_homogeneous = inverse_view_projection_matrix * p_ndc;
    let point_world_space = point_world_space_homogeneous.xyz() / point_world_space_homogeneous.w;

    let light_intensity = world_lights
        .iter()
        .map(|light| match light {
            Light::PointLight(point_light) => {
                let origin = point_light.get_origin();
                let light_norm = (origin - point_world_space).coords.normalize();
                let diffuse_intensity = light_norm.dot(normal).max(0.0);

                let distance = (origin - point_world_space).coords.magnitude();
                let attenuation = 1.0
                    / (1.0
                        + point_light.get_linear_attenuation() * distance
                        + point_light.get_quadratic_attenuation() * distance * distance);

                diffuse_intensity * attenuation * light.get_intensity()
            }
            Light::AmbientLight(_) => light.get_intensity(),
        })
        .sum::<f64>()
        .min(1.0);

    light_intensity
}

pub fn apply_ansi_256_dithering(screen_buffer: &mut Buffer<[u8; 3]>) {
    let width = screen_buffer.width;
    let height = screen_buffer.height;
    let mut dithering_errors = Buffer::<(i16, i16, i16)>::new((0, 0, 0), width, height);
    for y in 0..height {
        for x in 0..width {
            let r = screen_buffer[y][x][0];
            let g = screen_buffer[y][x][1];
            let b = screen_buffer[y][x][2];

            // Calculates dithering for ansi color banding
            let is_grey_scale = r == g && g == b;
            let (dr, dg, db) =
                calculate_dithered_pixel_value(x, y, r, g, b, &mut dithering_errors, is_grey_scale);
            screen_buffer[y][x] = [dr, dg, db];
        }
    }
}

fn calculate_dithered_pixel_value(
    x: usize,
    y: usize,
    r: u8,
    g: u8,
    b: u8,
    dithering_errors: &mut Buffer<(i16, i16, i16)>,
    is_grey_scale: bool,
) -> (u8, u8, u8) {
    let r: i16 = (r as i16) + dithering_errors[y][x].0;
    let g: i16 = (g as i16) + dithering_errors[y][x].1;
    let b: i16 = (b as i16) + dithering_errors[y][x].2;

    // Calculates error of casting to the nearest ansi color
    let calculate_error = |v: i16| -> i16 {
        let a = if v < 95 {
            return 0;
        } else {
            (1 + (v - 95) / 40).min(5)
        };
        let c = (a - 1) * 40 + 95;
        v - c
    };

    /* Burkess Dithering
                X   8   4
        2   4   8   4   2
    */

    let re = calculate_error(r);
    let ge = calculate_error(g);
    let be = calculate_error(b);

    let ra = r.min(255).max(0) as u8;
    let ga = g.min(255).max(0) as u8;
    let ba = b.min(255).max(0) as u8;

    let mut diffuse_error =
        |x: i16, y: i16, r_error: i16, g_error: i16, b_error: i16, factor: i16| {
            if x >= 0
                && y >= 0
                && x < ((dithering_errors.width) as i16)
                && y < ((dithering_errors.height) as i16)
            {
                dithering_errors[y as usize][x as usize].0 += r_error / factor;
                dithering_errors[y as usize][x as usize].1 += g_error / factor;
                dithering_errors[y as usize][x as usize].2 += b_error / factor;
            }
        };

    // Diffuse errors to neighboring pixels according to the dithering pattern
    let cx = x as i16;
    let cy = y as i16;
    diffuse_error(cx + 1, cy, re, ge, be, 4);
    diffuse_error(cx + 2, cy, re, ge, be, 8);
    diffuse_error(cx - 2, cy + 1, re, ge, be, 16);
    diffuse_error(cx - 1, cy + 1, re, ge, be, 8);
    diffuse_error(cx, cy + 1, re, ge, be, 4);
    diffuse_error(cx + 1, cy + 1, re, ge, be, 8);
    diffuse_error(cx + 2, cy + 1, re, ge, be, 16);

    if is_grey_scale {
        let u = ((ra as u16 + ga as u16 + ba as u16) / 3) as u8;
        return (u, u, u);
    }

    (ra, ga, ba)
}
