// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PRODUCED_PER_HOUR: u32 = 221;

fn success_rate(speed: u8) -> f64 {
    match speed {
        0 ..= 4 => 1.00,
        5 ..= 8 => 0.90,
        9 ..= 10 => 0.77,
        _ => panic!("Speed must be a value from 0 to 10")
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    CARS_PRODUCED_PER_HOUR as f64 * speed as f64 * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let production_per_minute = production_rate_per_hour(speed) / 60.0;
    production_per_minute.floor() as u32
}
