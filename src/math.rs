pub fn round_up_to_nearest_increment(value: f64, increment: f64) -> f64 {
    let scaled = value / increment;
    let rounded = scaled.ceil();
    rounded * increment
}

pub fn scale_range(x: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    assert!(x >= old_min && x <= old_max, "x must be within [-1, 1]");

    new_min + ((x - old_min) / (old_max - old_min)) * (new_max - new_min)
}