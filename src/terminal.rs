use std::time;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::{Color, Print, SetBackgroundColor},
    terminal,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use std::io;
use std::io::Write;

use crate::buffer::Buffer;
use crate::geometry;
use crate::keyboard::{Keyboard, Keys};

use geometry::ProjectionResult;

pub struct Terminal {
    width: usize,
    height: usize,
    default_color: [u8; 3],
    use_true_color: bool,
    aspect_ratio: f64,
    screen_buffer: Option<Buffer<[u8; 3]>>,
    keyboard: Keyboard,
}

impl Terminal {
    pub fn new(default_color: [u8; 3], aspect_ratio: f64, use_true_color: bool) -> Self {
        Self {
            width: 0,
            height: 0,
            default_color,
            use_true_color,
            aspect_ratio,
            screen_buffer: None,
            keyboard: Keyboard::new(),
        }
    }

    pub fn init(&self, stdout: &mut std::io::Stdout) -> io::Result<()> {
        enable_raw_mode()?;

        stdout.execute(EnterAlternateScreen)?;
        queue!(stdout, Hide)?;
        Ok(())
    }

    pub fn update(&mut self) -> io::Result<()> {
        self.keyboard.update()?;
        Ok(())
    }

    pub fn get_key_presses(&self) -> Vec<Keys> {
        Vec::from_iter(self.keyboard.pressed_keys.clone())
    }

    pub fn is_ctrl_c_pressed(&self) -> bool {
        self.keyboard.pressed_keys.contains(&Keys::CtrlC)
    }

    pub fn get_mutable_screen_buffer_reference(
        &mut self,
        stdout: &mut std::io::Stdout,
    ) -> &mut Buffer<[u8; 3]> {
        let (new_width, new_height) = get_aspect_corrected_dimensions(self.aspect_ratio);
        if new_width != self.width || new_height != self.height || self.screen_buffer.is_none() {
            clear_screen(stdout).ok();
            self.width = new_width;
            self.height = new_height;
            self.screen_buffer = Some(Buffer::<[u8; 3]>::new(
                self.default_color,
                self.width,
                self.height,
            ));
        }

        self.screen_buffer.as_mut().unwrap()
    }

    pub fn output_screen_buffer(&self, stdout: &mut std::io::Stdout) -> io::Result<()> {
        queue!(stdout, MoveTo(1, 1))?;
        let screen_buffer = self.screen_buffer.as_ref().unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                let r = screen_buffer[y][x][0];
                let g = screen_buffer[y][x][1];
                let b = screen_buffer[y][x][2];

                let color = if self.use_true_color {
                    Color::Rgb { r, g, b }
                } else {
                    Color::AnsiValue(rgb_to_ansi256(r, g, b))
                };

                queue!(stdout, SetBackgroundColor(color), Print("  "))?;
            }
            queue!(stdout, MoveTo(1, (y + 1) as u16))?;
        }
        flush(stdout)?;
        Ok(())
    }

    pub fn destroy(&self, stdout: &mut std::io::Stdout) -> io::Result<()> {
        queue!(stdout, Show)?;
        disable_raw_mode()?;
        stdout.execute(LeaveAlternateScreen)?;
        clear_screen(stdout)?;
        flush(stdout)?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}

fn rgb_channel_to_ansi_index(v: u8) -> u8 {
    // the ansi rgb values are on the scale 0-5
    // 0-95 map to 0, 95-255 map to 1-5
    if v < 95 {
        return 0;
    }
    1 + (v - 95) / 40
}

pub fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let rc = rgb_channel_to_ansi_index(r);
    let gc = rgb_channel_to_ansi_index(g);
    let bc = rgb_channel_to_ansi_index(b);

    // Uses finer grayscale. Ignores 0 case since the deadzone is massive
    //if rc != 0 && rc == gc && gc == bc {
    //    return 232 + ((r as f64) * 0.09375) as u16;
    //}

    16 + 36 * rc + 6 * gc + bc
}

fn flush(stdout: &mut std::io::Stdout) -> io::Result<()> {
    stdout.flush()?;
    Ok(())
}

fn clear_screen(stdout: &mut std::io::Stdout) -> io::Result<()> {
    queue!(stdout, SetBackgroundColor(Color::Black))?;
    queue!(stdout, Clear(ClearType::All))?;
    stdout.flush()?;
    Ok(())
}

fn get_aspect_corrected_dimensions(target_aspect_rato: f64) -> (usize, usize) {
    let (columns, rows) = terminal::size().expect("Failed to get terminal size");
    let width = (columns / 2 - 2) as usize;
    let height = (rows - 2) as usize;

    let aspect: f64 = (width as f64) / (height as f64);
    if aspect > target_aspect_rato {
        return (((height as f64) * target_aspect_rato) as usize, height);
    }
    (width, ((width as f64) / target_aspect_rato) as usize)
}

#[allow(dead_code)]
pub fn print_debug_info(
    stdout: &mut std::io::Stdout,
    total_time_elapsed: &time::Duration,
    _processed_time_elapsed: &time::Duration,
    projection_results: &[ProjectionResult],
    screen_height: usize,
) {
    queue!(stdout, MoveTo(1, (screen_height) as u16)).unwrap();
    queue!(
        stdout,
        SetBackgroundColor(Color::AnsiValue(0)),
        Print(format!(
            "total loop time elapsed ms: {:3.0}",
            total_time_elapsed.as_secs_f64() * 1000.0
        ))
    )
    .unwrap();
    queue!(stdout, MoveTo(1, (screen_height + 1) as u16)).unwrap();
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
