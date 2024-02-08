pub fn round_up_to_nearest_increment(value: f32, increment: f32) -> f32 {
    let scaled = value / increment;
    let rounded = scaled.ceil();
    rounded * increment
}

pub fn scale_range(x: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    assert!(x >= old_min && x <= old_max, "x must be within [-1, 1]");

    new_min + ((x - old_min) / (old_max - old_min)) * (new_max - new_min)
}