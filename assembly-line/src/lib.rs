// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    const PRODUCTION_AT_BASE_SPEED: f64 = 221.0;
    match speed {
        0 => 0.0,
        1..=4 => speed as f64 * 1.0 * PRODUCTION_AT_BASE_SPEED,
        5..=8 => speed as f64 * 0.9 * PRODUCTION_AT_BASE_SPEED,
        9 | 10 => speed as f64 * 0.77 * PRODUCTION_AT_BASE_SPEED,
        _ => panic!("Speed not in range 0 - 10"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
