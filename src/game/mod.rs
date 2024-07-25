//! Game mechanics and content.

use bevy::prelude::*;

pub mod assets;
pub mod audio;
pub mod cycles;
mod movement;
pub mod spawn;
mod ui;
mod upgrades;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
        cycles::plugin,
        ui::plugin,
        upgrades::plugin,
    ));
}
