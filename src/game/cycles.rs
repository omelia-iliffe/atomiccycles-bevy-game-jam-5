//! Count the number of cycles for use as currency

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CycleCount(0));
    app.observe(add_cycle);
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
