//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;
use std::f32::consts::PI;

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
pub struct BaseTransform(pub Transform);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController {
    pub add_count: bool,
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
    let add_count = input.clear_just_pressed(KeyCode::Space);

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        if add_count {
            controller.add_count = true;
        }
    }
}

fn apply_movement(mut movement_query: Query<(&mut MovementController, &mut RevolutionController)>) {
    for (mut controller, mut count) in &mut movement_query {
        if controller.add_count && count.refire_allowed() {
            count.add_count();
            controller.add_count = false
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Revolve {
    pub speed: f32,
    pub multiplier: f32,
}

impl Revolve {
    pub fn new(speed: f32) -> Self {
        Revolve {
            speed,
            multiplier: 1.0,
        }
    }

    pub fn speed(&self) -> f32 {
        self.speed * self.multiplier
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct RevolutionController {
    pub count: u32,
    pub count_max: u32,
    pub refire_angle: f32,
    angle: f32,
}

impl RevolutionController {
    pub fn new(count_max: u32, refire_angle: f32) -> Self {
        Self {
            count: 0,
            count_max,
            angle: 0.,
            refire_angle,
        }
    }

    pub fn add_count(&mut self) {
        self.count = std::cmp::min(self.count + 1, self.count_max);
    }

    pub fn refire_allowed(&self) -> bool {
        self.angle == 0.0 || self.angle > self.refire_angle
    }
}
fn apply_revolve(
    time: Res<Time>,
    mut movement_query: Query<(
        &Revolve,
        &mut RevolutionController,
        &mut Transform,
        &BaseTransform,
    )>,
    mut commands: Commands,
) {
    for (revolve, mut count, mut transform, base) in &mut movement_query {
        if count.count > 0 {
            count.angle += revolve.speed() * time.delta_seconds();
            if count.angle > 2.0 * PI {
                count.angle = 0.0;
                count.count -= 1;
                commands.trigger(AddCycle)
            }
            transform.rotation = base.0.rotation;
            transform.translation = base.0.translation;
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(count.angle));
        }
        log::debug!("rotation: {}, count {}", transform.rotation.z, count.count,)
    }
}
