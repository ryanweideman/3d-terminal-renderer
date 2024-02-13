use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::geometry;

use crossterm::{queue, style::{Color, Print, SetBackgroundColor}, cursor::MoveTo};

use nalgebra::{Point2};
use geometry::{ProjectionResult};

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


pub fn output_screen_buffer(stdout : &mut std::io::Stdout, screen_buffer : &[[u16; SCREEN_WIDTH] ; SCREEN_HEIGHT]) {
    queue!(stdout, MoveTo(1, 1)).unwrap();
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            queue!(
                stdout,
                SetBackgroundColor(Color::AnsiValue(screen_buffer[y][x] as u8)),
                Print("  ")
            ).unwrap();
        }
        queue!(
            stdout,
            MoveTo(1, (y+1) as u16)
        ).unwrap();
    }
}

pub fn interpolate_attributes_at_pixel(
    p: &Point2<f32>,
    projection_result: &ProjectionResult) 
    -> (f32, f32) {

    let (p0, p1, p2) = projection_result.screen_triangle.vertices();
    let (clip_v0, clip_v1, clip_v2) = projection_result.clip_space_triangle.vertices();
    let (ndc_v0, ndc_v1, ndc_v2) = projection_result.ndc_triangle.vertices();

    let total_area : f32 = p0.x * (p1.y - p2.y) + p1.x * (p2.y - p0.y) + p2.x * (p0.y - p1.y);
    let lambda0 : f32 = ((p1.y - p2.y) * (p.x - p2.x) + (p2.x - p1.x) * (p.y - p2.y)) / total_area;
    let lambda1 : f32 = ((p2.y - p0.y) * (p.x - p2.x) + (p0.x - p2.x) * (p.y - p2.y)) / total_area;
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

    let z = ndc_v0.z * lambdap0 + ndc_v1.z * lambdap1 + ndc_v2.z * lambdap2;
    let w = 1.0 / den;
    (z, w)
}