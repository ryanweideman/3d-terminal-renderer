pub const ASPECT_RATIO  : f32 = 4.0 / 3.0;
pub const SCREEN_WIDTH  : usize = 54;
pub const SCREEN_HEIGHT : usize = ((SCREEN_WIDTH as f32) / ASPECT_RATIO) as usize;
pub const NEAR_PLANE : f32 = 0.1;
pub const FAR_PLANE  : f32 = 1000.0;
pub const FOV : f32 = std::f32::consts::PI / 2.5;
pub const TARGET_FPS : f32 = 20.0;