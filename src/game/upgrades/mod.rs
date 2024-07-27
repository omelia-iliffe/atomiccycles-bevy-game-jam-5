mod upgrade_types;

use super::{cycles::CycleCount, movement::Revolve};
use crate::game::spawn::atom::AddElectron;
use crate::game::ui::upgrades::{GlobalUpgradeIndex, UpgradeEntity};
use crate::game::upgrades::upgrade_types::SingleUpgrade;
use bevy::prelude::*;
use upgrade_types::LevelUpgrade;

pub use upgrade_types::Upgrade;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, add_global_upgrades);
    app.add_systems(Update, (apply_upgrade, apply_global_upgrade));
}

fn add_global_upgrades(mut commands: Commands) {
    commands.insert_resource(Upgrades::global());
}

#[derive(Debug, Clone)]
pub enum UpgradeAction {
    SpeedAdd(f32),
    SpeedMult(f32),
    Electron,
}

impl std::fmt::Display for UpgradeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpgradeAction::SpeedAdd(v) => write!(f, "Speed +{}", v),
            UpgradeAction::SpeedMult(v) => write!(f, "Speed x{}", v),
            UpgradeAction::Electron => write!(f, ""),
        }
    }
}

#[derive(Component, Resource)]
pub struct Upgrades(pub Vec<Box<dyn Upgrade + Send + Sync>>);

impl Upgrades {
    pub fn global() -> Self {
        Self(vec![Box::new(SingleUpgrade::new(
            "New Electron",
            None,
            50,
            UpgradeAction::Electron,
        ))])
    }
    pub fn electron() -> Self {
        Self(vec![Box::new(LevelUpgrade::new(
            "Speed Single",
            None,
            vec![
                (1, UpgradeAction::SpeedAdd(1.0)),
                (2, UpgradeAction::SpeedAdd(1.0)),
                (2, UpgradeAction::SpeedAdd(1.0)),
            ],
        ))])
    }
}

fn process_upgrade(
    upgrade: &mut dyn Upgrade,
    cycle_count: &mut CycleCount,
    process: impl FnOnce(&UpgradeAction),
) {
    log::info!("Pressed upgrade: {}", upgrade.name());
    if upgrade.purchased() {
        log::info!("Upgrade already purchased");
        return;
    }
    let Some(cost) = upgrade.next_cost() else {
        return;
    };
    let can_purchase = cycle_count.0 >= cost;

    if !can_purchase {
        log::info!("Cannot purchase upgrade: not enough cycles");
        return;
    }

    cycle_count.0 -= cost;

    log::info!("Purchased upgrade: {}", upgrade.name());

    let upgrade_type = upgrade.purchase();
    match upgrade_type {
        Some(upgrade_type) => process(upgrade_type),
        None => log::info!("No more upgrades"),
    }
}

fn apply_global_upgrade(
    mut commands: Commands,
    q_interaction: Query<(&Interaction, &GlobalUpgradeIndex), Changed<Interaction>>,
    mut upgrades: ResMut<Upgrades>,
    mut query: Query<&mut Revolve>,
    mut cycle_count: ResMut<CycleCount>,
) {
    for (interaction, index) in q_interaction.iter() {
        if interaction != &Interaction::Pressed {
            continue;
        }

        let Some(upgrade) = upgrades.0.get_mut(index.0) else {
            continue;
        };
        process_upgrade(
            upgrade.as_mut(),
            cycle_count.as_mut(),
            |upgrade_type| match upgrade_type {
                UpgradeAction::SpeedAdd(speed) => {
                    for mut r in &mut query {
                        r.speed += speed;
                    }
                }
                UpgradeAction::SpeedMult(mult) => {
                    for mut r in &mut query {
                        r.multiplier = *mult;
                    }
                }
                UpgradeAction::Electron => {
                    commands.trigger(AddElectron);
                }
            },
        );
    }
}
fn apply_upgrade(
    q_interaction: Query<(&Interaction, &UpgradeEntity), Changed<Interaction>>,
    mut q_electron: Query<(&mut Upgrades, &mut Revolve)>,

    mut cycle_count: ResMut<CycleCount>,
) {
    for (interaction, index) in q_interaction.iter() {
        if interaction != &Interaction::Pressed {
            continue;
        }

        let Ok((mut upgrades, mut revolve)) = q_electron.get_mut(index.entity) else {
            continue;
        };
        let Some(upgrade) = upgrades.0.get_mut(index.index) else {
            continue;
        };
        process_upgrade(
            upgrade.as_mut(),
            cycle_count.as_mut(),
            |upgrade_type| match upgrade_type {
                UpgradeAction::SpeedAdd(speed) => {
                    revolve.speed += speed;
                }
                UpgradeAction::SpeedMult(mult) => {
                    revolve.multiplier = *mult;
                }
                UpgradeAction::Electron => (),
            },
        );
    }
}
