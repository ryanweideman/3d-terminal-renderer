use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::geometry;

use nalgebra::{Point2, Point3, Point4};
use geometry::{Triangle3};
use std::io::Write;

pub fn clear_screen() {
    println!("\x1b[H\x1b[J");
    std::io::stdout().flush().unwrap();
}

pub fn reset_cursor() {
    println!("\x1b[H");
    std::io::stdout().flush().unwrap();
}

pub fn hide_cursor() {
    print!("\x1b[?25l");
    std::io::stdout().flush().unwrap();
}

pub fn show_cursor() {
    print!("\x1b[?25h");
    std::io::stdout().flush().unwrap();
}

pub fn move_cursor(x : usize, x_offset : usize, y : usize, y_offset : usize) {
    print!("\x1b[{};{}H", y + y_offset, x + x_offset);
    std::io::stdout().flush().unwrap();
}

fn rgb_channel_to_ansi_index(v: u8) -> u8 {
    // the ansi rgb values are on the scale 0-5
    // 0-95 map to 0, 95-255 map to 1-5
    if v < 95 {
        return 0;
    }
    1 + (v - 95) / 40
}

pub fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u16 {
    let rc = rgb_channel_to_ansi_index(r);
    let gc = rgb_channel_to_ansi_index(g);
    let bc = rgb_channel_to_ansi_index(b);

    // Uses finer grayscale. Ignores 0 case since the deadzone is massive
    if rc != 0 && rc == gc && gc == bc {
        return 232 + ((r as f32) * 0.09375) as u16;
    }

    (16 + 36 * rc + 6 * gc + bc).into()
}


pub fn output_screen_buffer(screen_buffer : &[[u16; SCREEN_WIDTH] ; SCREEN_HEIGHT]) {
    print!("  ");
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            print!("\x1b[48;5;{}m  \x1b[m", screen_buffer[y][x]);
        }
        print!("\n  ");
    }
}

pub fn interpolate_attributes_at_pixel(
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

pub fn is_point_in_triangle(pt: &Point2<f32>, triangle: &Triangle3) -> bool {
    let v1 = Point2::new(triangle.geometry[0].x, triangle.geometry[0].y);
    let v2 = Point2::new(triangle.geometry[1].x, triangle.geometry[1].y);
    let v3 = Point2::new(triangle.geometry[2].x, triangle.geometry[2].y);

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

pub fn calculate_bounding_box(projected_triangle : &Triangle3) -> (usize, usize, usize, usize) {
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