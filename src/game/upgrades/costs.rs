use std::time::Duration;

pub const STARTING_CYCLES: u32 = compute_ring_cost(0) + compute_electron_cost(0, 0);

pub const fn compute_electron_cost(rings: usize, electrons: usize) -> u32 {
    match (rings, electrons) {
        (0,0 ) => 0,
        (0,1 ) => 5,
        (_, _) =>
            5 * (electrons as u32 + 1) * (rings as u32 + 1).pow(2)
    }
}

pub const fn compute_ring_cost(rings: usize) -> u32 {
    match rings {
        0 => 0,
        _ => 25  * (rings as u32).pow(2)
    }
}

pub fn compute_speed_cost(rings:usize, level: u32) -> u32 {
    (4 * ( level + 1) )* (rings as u32 + 1).pow(2)
}

pub fn compute_cycle_cost(rings: usize, duration: Option<Duration>) -> u32 {
    (if let Some(duration) = duration {
        let secs = duration.as_secs_f32();
        (3. / secs) as u32 + 5
    } else {
        3
    })  * (rings as u32 + 1).pow(2)
}
