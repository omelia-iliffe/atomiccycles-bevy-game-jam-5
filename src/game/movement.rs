//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use crate::{game::cycles::AddCycle, AppSet};

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.add_systems(
        Update,
        (apply_movement, apply_revolve)
            .chain()
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController {
    add_count: bool,
}

impl MovementController {
    pub fn new() -> Self {
        Self { add_count: false }
    }
}

fn record_movement_controller(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // Collect directional input.
    let add_count = if input.clear_just_pressed(KeyCode::Space) {
        true
    } else {
        false
    };

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.add_count = add_count;
    }
}

fn apply_movement(
    mut movement_query: Query<(
        &MovementController,
        &mut RevolutionCount,
        &Transform,
        &RevolveZone,
    )>,
) {
    for (controller, mut count, transform, revolve_zone) in &mut movement_query {
        log::debug!(
            "inside: {}",
            revolve_zone.inside(transform.rotation.z.abs()),
        );
        if controller.add_count && revolve_zone.inside(transform.rotation.z.abs()) {
            count.add_count();
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Revolve {
    pub speed: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RevolveZone {
    min: f32,
    max: f32,
}

impl RevolveZone {
    pub fn new(min: f32, max: f32) -> Self {
        RevolveZone { min, max }
    }
    pub fn inside(&self, angle: f32) -> bool {
        angle < self.max && angle > self.min
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RevolutionCount {
    pub count: u32,
    pub count_max: u32,
    decreasing: bool,
}

impl RevolutionCount {
    pub fn new(count: u32) -> Self {
        let count_max = 1;
        let count = if count > count_max { count_max } else { count };
        Self {
            count,
            count_max,
            decreasing: false,
        }
    }

    pub fn add_count(&mut self) {
        self.count = std::cmp::min(self.count + 1, self.count_max);
    }
}
fn apply_revolve(
    time: Res<Time>,
    mut movement_query: Query<(&Revolve, &mut RevolutionCount, &mut Transform)>,
    mut commands: Commands,
) {
    for (revolve, mut count, mut transform) in &mut movement_query {
        let curr_rot = transform.rotation.z;
        if count.count > 0 {
            let angle = revolve.speed * time.delta_seconds();
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(angle));
            if transform.rotation.z.abs() < curr_rot.abs() && !count.decreasing {
                count.decreasing = true;
            } else if transform.rotation.z.abs() > curr_rot.abs() && count.decreasing {
                count.decreasing = false;
                count.count -= 1;
                commands.trigger(AddCycle)
            }
        }
        log::debug!(
            "rotation: {}, count {}, decreasing {}",
            transform.rotation.z,
            count.count,
            count.decreasing
        )
    }
}
