pub const ASPECT_RATIO  : f64 = 4.0 / 4.0;
pub const SCREEN_WIDTH  : usize = 60;
pub const SCREEN_HEIGHT : usize = ((SCREEN_WIDTH as f64) / ASPECT_RATIO) as usize;
pub const NEAR_PLANE : f64 = 0.1;
pub const FAR_PLANE  : f64 = 30.0;
pub const FOV : f64 = std::f64::consts::PI / 2.5;
pub const TARGET_FPS : f64 = 30.0;