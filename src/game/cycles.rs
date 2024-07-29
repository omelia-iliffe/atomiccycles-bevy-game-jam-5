//! Count the number of cycles for use as currency

use crate::game::upgrades::costs::STARTING_CYCLES;
use crate::AppSet;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CycleCount(STARTING_CYCLES))
        .observe(add_cycle)
        .add_systems(
            Update,
            record_movement_controller.in_set(AppSet::RecordInput),
        );
}

#[derive(Event, Debug)]
pub struct AddCycle;

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
pub struct CycleCount(pub u32);

fn add_cycle(_trigger: Trigger<AddCycle>, mut count: ResMut<CycleCount>) {
    count.0 += 1;
    log::info!("Added to cycle count: {}", count.0)
}

fn record_movement_controller(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut cycle_count: ResMut<CycleCount>,
) {
    if input.clear_just_pressed(KeyCode::KeyM) {
        cycle_count.0 += 1;
    }
    if input.clear_just_pressed(KeyCode::KeyN) {
        cycle_count.0 += 10;
    }
}
