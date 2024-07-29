use std::time::Duration;

pub fn compute_electron_cost(rings: usize, electrons: usize) -> u32 {
    5 * (electrons as u32 + 1) * (rings as u32 + 1).pow(2)
pub const STARTING_CYCLES: u32 = compute_ring_cost(0) + compute_electron_cost(0, 0);

}

pub fn compute_ring_cost(count: usize) -> u32 {
    25 + (10 * count as u32)
}

pub fn compute_speed_cost(speed: f32) -> u32 {
    (2. * speed) as u32
}

pub fn compute_cycle_cost(duration: Option<Duration>) -> u32 {
    if let Some(duration) = duration {
        let secs = duration.as_secs_f32();
        (3. / secs) as u32
    } else {
        10
    }
}
