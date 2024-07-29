pub mod costs;

use super::{cycles::CycleCount, movement::Revolve};
use crate::game::assets::{HandleMap, ImageKey};
use crate::game::spawn::atom::{AddProton, AddProtonNeutron, Atom, Electron, ElectronBundle, Ring};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            apply_electron_upgrade,
            apply_cycle_upgrade,
            apply_speed_upgrade,
            apply_buy_ring,
        ),
    );
}

#[derive(Component)]
pub struct BuyNextRing;
#[derive(Component)]
pub struct BuyElectron(pub Entity);
#[derive(Component)]
pub struct SpeedUpgrade(pub Entity);
#[derive(Component)]
pub struct CycleUpgrade(pub Entity);

const MAX_RINGS: usize = 5;

pub const INITIAL_REVOLVE_SPEED: f32 = 3.0;
fn apply_buy_ring(
    q_interaction: Query<(Entity, &Interaction), (With<BuyNextRing>, Changed<Interaction>)>,
    mut cycle_count: ResMut<CycleCount>,
    mut commands: Commands,
    query_atom: Query<(Entity, Option<&Children>), With<Atom>>,
    query_rings: Query<&Ring>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, interaction) in &q_interaction {
        if interaction != &Interaction::Pressed {
            continue;
        }
        let (atom, maybe_children) = query_atom.get_single().unwrap();
        let ring_count = maybe_children
            .map(|children| {
                children
                    .iter()
                    .filter(|child| query_rings.get(**child).is_ok())
                    .count()
            })
            .unwrap_or_default();

        if ring_count + 1 == MAX_RINGS {
            log::info!("Last Ring purchased");
            commands.entity(entity).despawn_recursive();
        }

        let cost = costs::compute_ring_cost(ring_count);
        if cost > cycle_count.0 {
            log::info!("Cannot afford ring: not enough cycles");
            continue;
        }

        cycle_count.0 -= cost;

        let ring = Ring::new(ring_count);
        let ring_radius = ring.radius();
        commands.entity(atom).with_children(|parent| {
            parent.spawn((
                ring,
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle::new(ring_radius))),
                    material: materials.add(Color::srgba_u8(0x28, 0x66, 0x6e, 0x66)),
                    transform: Transform::from_xyz(0., 0., -100.),
                    ..default()
                },
                Revolve::new(INITIAL_REVOLVE_SPEED),
            ));
        });
    }
}

fn apply_speed_upgrade(
    q_interaction: Query<(&Interaction, &SpeedUpgrade), Changed<Interaction>>,
    mut cycle_count: ResMut<CycleCount>,

    mut query_ring: Query<(&Ring, &mut Revolve)>,
) {
    for (interaction, entity) in &q_interaction {
        if interaction != &Interaction::Pressed {
            continue;
        }

        let Ok((ring, mut revolve)) = query_ring.get_mut(entity.0) else {
            continue;
        };

        let cost = costs::compute_speed_cost(ring.index, revolve.level);
        if cost > cycle_count.0 {
            log::info!("Cannot afford speed upgrade: not enough cycles");
            continue;
        }

        revolve.speed += 1.0;
        revolve.level += 1;
        cycle_count.0 -= cost;
    }
}

const INITIAL_CYCLE_TIME: Duration = Duration::from_secs(3);
fn apply_cycle_upgrade(
    q_interaction: Query<(&Interaction, &CycleUpgrade), Changed<Interaction>>,
    mut cycle_count: ResMut<CycleCount>,

    mut query_ring: Query<&mut Ring>,
) {
    for (interaction, entity) in &q_interaction {
        if interaction != &Interaction::Pressed {
            continue;
        }

        let Ok(mut ring) = query_ring.get_mut(entity.0) else {
            continue;
        };

        let cost = costs::compute_cycle_cost(
            ring.index,
            ring.cycle_timer.as_ref().map(|timer| timer.duration()),
        );
        if cost > cycle_count.0 {
            log::info!("Cannot afford cycle speed upgrade: not enough cycles");
            continue;
        }

        if let Some(timer) = ring.cycle_timer.as_mut() {
            timer.set_duration(timer.duration() * 4 / 5);
        } else {
            ring.cycle_timer = Some(Timer::new(INITIAL_CYCLE_TIME, TimerMode::Repeating));
        }

        cycle_count.0 -= cost;
    }
}
fn apply_electron_upgrade(
    mut commands: Commands,
    q_interaction: Query<(&Interaction, &BuyElectron), Changed<Interaction>>,
    mut cycle_count: ResMut<CycleCount>,

    image_handles: Res<HandleMap<ImageKey>>,
    query_ring: Query<(Entity, Option<&Children>, &Ring)>,
    query_electrons: Query<(&Parent, &Electron)>,
) {
    for (interaction, entity) in &q_interaction {
        if interaction != &Interaction::Pressed {
            continue;
        }

        let Ok((parent, maybe_children, ring)) = query_ring.get(entity.0) else {
            continue;
        };

        let mut electron_count = 0;
        if let Some(children) = maybe_children {
            for child in children {
                if query_electrons.get(*child).is_ok() {
                    electron_count += 1;
                }
            }
        }

        if electron_count >= ring.max_electrons {
            log::info!("Ring {} is full", ring.index);
            continue;
        }

        let cost = costs::compute_electron_cost(ring.index, electron_count);
        if cost > cycle_count.0 {
            log::info!("Cannot afford electron: not enough cycles");
            continue;
        }

        commands.entity(parent).with_children(|parent| {
            parent.spawn(ElectronBundle::new(
                ring.index,
                electron_count,
                ring.radius(),
                image_handles.as_ref(),
            ));
        });

        if ring.index == 0 && electron_count == 0 {
            commands.trigger(AddProton);
        } else {
            commands.trigger(AddProtonNeutron);
        }

        cycle_count.0 -= cost;

        //success sub money
    }
}
