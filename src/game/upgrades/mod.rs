mod upgrade_types;

use super::{cycles::CycleCount, movement::Revolve};
use crate::game::spawn::atom::{add_electron, Electron, Ring};
use crate::game::ui::upgrades::{GlobalUpgradeIndex, UpgradeEntity};
use crate::game::upgrades::upgrade_types::SingleUpgrade;
use bevy::prelude::*;
use upgrade_types::LevelUpgrade;

use crate::game::assets::{HandleMap, ImageKey};
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
        Self(vec![
            Box::new(SingleUpgrade::new(
                "New Electron",
                None,
                15,
                UpgradeAction::Electron,
            )),
            Box::new(SingleUpgrade::new(
                "New Electron",
                None,
                15,
                UpgradeAction::Electron,
            )),
        ])
    }
    pub fn electron() -> Self {
        Self(vec![
            Box::new(LevelUpgrade::new(
                "Add Speed",
                None,
                vec![
                    (1, UpgradeAction::SpeedAdd(1.0)),
                    (2, UpgradeAction::SpeedAdd(1.0)),
                    (4, UpgradeAction::SpeedAdd(1.0)),
                    (8, UpgradeAction::SpeedAdd(1.0)),
                    (16, UpgradeAction::SpeedAdd(1.0)),
                    (32, UpgradeAction::SpeedAdd(1.0)),
                ],
            )),
            Box::new(LevelUpgrade::new(
                "Multiply Speed",
                None,
                vec![
                    (1, UpgradeAction::SpeedMult(1.1)),
                    (2, UpgradeAction::SpeedAdd(1.1)),
                    (4, UpgradeAction::SpeedAdd(1.1)),
                    (8, UpgradeAction::SpeedAdd(1.1)),
                    (16, UpgradeAction::SpeedAdd(1.1)),
                    (32, UpgradeAction::SpeedAdd(1.1)),
                ],
            )),
        ])
    }
}

// }
//     None => log::info!("No more upgrades"),
//     Some(upgrade_type) => process(upgrade_type),
fn process_upgrade(
    upgrade: &mut dyn Upgrade,
    cycle_count: &mut CycleCount,
    process: impl FnOnce(&UpgradeAction) -> bool,
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

    let upgrade_type = upgrade.upgrade_action().unwrap();

    let success = process(upgrade_type);
    if !success {
        log::error!("Failed to apply upgrade");
        return;
    }

    if upgrade.purchase().is_err() {
        log::error!("Failed to purchase upgrade");
        return;
    }
    cycle_count.0 -= cost;

    log::info!("Purchased upgrade: {}", upgrade.name());
}

fn apply_global_upgrade(
    mut commands: Commands,
    q_interaction: Query<(&Interaction, &GlobalUpgradeIndex), Changed<Interaction>>,
    mut upgrades: ResMut<Upgrades>,
    mut query_revolve: Query<&mut Revolve>,
    mut cycle_count: ResMut<CycleCount>,
    image_handles: Res<HandleMap<ImageKey>>,
    query_ring: Query<(Entity, &Children, &Ring)>,
    query_electrons: Query<(&Parent, &Electron)>,
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
                    for mut r in &mut query_revolve {
                        r.speed += speed;
                    }
                    true
                }
                UpgradeAction::SpeedMult(mult) => {
                    for mut r in &mut query_revolve {
                        r.multiplier = *mult;
                    }
                    true
                }
                UpgradeAction::Electron => {
                    add_electron(
                        &mut commands,
                        image_handles.as_ref(),
                        &query_ring,
                        &query_electrons,
                    )

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
                    true
                }
                UpgradeAction::SpeedMult(mult) => {
                    revolve.multiplier = *mult;
                    true
                }
                UpgradeAction::Electron => false,
            },
        );
    }
}
