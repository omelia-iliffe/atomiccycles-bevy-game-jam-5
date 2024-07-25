//! The ui for the level

use bevy::prelude::*;
mod cycle_ui;
mod upgrades;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level_ui);
    app.add_plugins((cycle_ui::plugin, upgrades::plugin));
}

#[derive(Event, Debug)]
pub struct SpawnLevelUi;

fn spawn_level_ui(_trigger: Trigger<SpawnLevelUi>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    // commands.trigger(SpawnPlayer);

    commands.trigger(cycle_ui::SpawnCycleUi);
    commands.trigger(upgrades::SpawnUpgradesUi);
}
