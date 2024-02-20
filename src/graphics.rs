use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::geometry;

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, SetBackgroundColor},
    terminal::Clear,
    terminal::ClearType,
};

use geometry::ProjectionResult;
use nalgebra::Point2;
use std::time;

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
/*
    // Uses finer grayscale. Ignores 0 case since the deadzone is massive
    if rc != 0 && rc == gc && gc == bc {
        return 232 + ((r as f64) * 0.09375) as u16;
    }
*/
    (16 + 36 * rc + 6 * gc + bc).into()
}

pub fn clear_screen(stdout: &mut std::io::Stdout) {
    queue!(stdout, Clear(ClearType::All)).ok();
}

pub fn output_screen_buffer(
    stdout: &mut std::io::Stdout,
    screen_buffer: &[[u16; SCREEN_WIDTH]; SCREEN_HEIGHT],
) {
    queue!(stdout, MoveTo(1, 1)).unwrap();
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            queue!(
                stdout,
                SetBackgroundColor(Color::AnsiValue(screen_buffer[y][x] as u8)),
                Print("  ")
            )
            .unwrap();
        }
        queue!(stdout, MoveTo(1, (y + 1) as u16)).unwrap();
    }
}

pub fn print_debug_info(
    stdout: &mut std::io::Stdout,
    total_time_elapsed: &time::Duration,
    processed_time_elapsed: &time::Duration,
    projection_results: &Vec<ProjectionResult>,
) {
    queue!(stdout, MoveTo(1, (SCREEN_HEIGHT) as u16)).unwrap();
    queue!(
        stdout,
        SetBackgroundColor(Color::AnsiValue(0)),
        Print(format!(
            "total loop time elapsed ms: {:3.0}",
            total_time_elapsed.as_secs_f64() * 1000.0
        ))
    )
    .unwrap();
    queue!(stdout, MoveTo(1, (SCREEN_HEIGHT + 1) as u16)).unwrap();
    /*
    queue!(
        stdout,
        SetBackgroundColor(Color::AnsiValue(0)),
        Print(format!("processing time elapsed ms: {:3.0}", processed_time_elapsed.as_secs_f64() * 1000.0))
    ).unwrap();
    queue!(stdout, MoveTo(1, (SCREEN_HEIGHT + 2) as u16)).unwrap();
    */
    /*
    for i in 0..projection_results.len() {
        let result = projection_results[i];
        let (c0, c1, c2) = result.camera_frame_triangle.vertices();
        let (cl0, cl1, cl2) = result.clip_space_triangle.vertices();
        let (n0, n1, n2) = result.ndc_triangle.vertices();

        queue!(stdout, MoveTo(1, (SCREEN_HEIGHT + i*4) as u16)).unwrap();
        queue!(
            stdout,
            SetBackgroundColor(Color::AnsiValue(0)),
            Print(format!(
                "camera [{:.2} {:.2} {:.2}] [{:.2} {:.2} {:.2}] [{:.2} {:.2} {:.2}]",
                c0.x, c0.y, c0.z, c1.x, c1.y, c1.z, c2.x, c2.y, c2.z
            ))
        ).unwrap();

        queue!(stdout, MoveTo(1, (SCREEN_HEIGHT + i*4 + 1) as u16)).unwrap();
        queue!(
            stdout,
            SetBackgroundColor(Color::AnsiValue(0)),
            Print(format!(
                "clip   [{:.2} {:.2} {:.2} {:.2}] [{:.2} {:.2} {:.2} {:.2}] [{:.2} {:.2} {:.2} {:.2}]",
                cl0.x, cl0.y, cl0.z, cl0.w, cl1.x, cl1.y, cl1.z, cl1.w, cl2.x, cl2.y, cl2.z, cl2.w
            ))
        ).unwrap();

        queue!(stdout, MoveTo(1, (SCREEN_HEIGHT + i*4 + 2) as u16)).unwrap();
        queue!(
            stdout,
            SetBackgroundColor(Color::AnsiValue(0)),
            Print(format!(
                "ndc    [{:.2} {:.2} {:.2}] [{:.2} {:.2} {:.2}] [{:.2} {:.2} {:.2}]",
                n0.x, n0.y, n0.z, n1.x, n1.y, n1.z, n2.x, n2.y, n2.z
            ))
        ).unwrap();
    }
    */

    queue!(
        stdout,
        SetBackgroundColor(Color::AnsiValue(0)),
        Print(format!(
            "num triangles in frame: {:5}",
            projection_results.len()
        ))
    )
    .unwrap();
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

    let z = 1.0 / (iz0 * lambda0 + iz1 * lambda1 + iz2 * lambda2);
    z
}
