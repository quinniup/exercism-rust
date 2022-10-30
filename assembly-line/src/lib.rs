// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut count_for_hours = 221.0 * f64::from(speed);
    match speed  {
        1..=4 => count_for_hours = count_for_hours * 1.0,
        5..=8 => count_for_hours = count_for_hours * 0.9,
        9|10 => count_for_hours = count_for_hours * 0.77,
        _ => {}
    };
    return count_for_hours
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let count_for_minute = 221.0 / 60.0 * f64::from(speed);
    let mut total_for_minute = 0.0;
    match speed  {
        1..=4 => total_for_minute = count_for_minute * 1.0,
        5..=8 => total_for_minute = count_for_minute * 0.9,
        9|10 => total_for_minute = count_for_minute * 0.77,
        _ => {}
    };
    return total_for_minute as u32
}
