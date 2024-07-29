//! Spawn the atom scene

use crate::game::assets::{HandleMap, ImageKey};
use crate::game::movement::BaseTransform;
use crate::{
    game::movement::{MovementController, RevolutionController},
    screen::Screen,
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use rand::random;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_atom_scene)
        .observe(add_proton)
        .observe(add_proton_neutron)
        .add_systems(Update, cycle_rings);
}

#[derive(Event, Debug)]
pub struct SpawnAtomScene;

#[derive(Component)]
pub struct Atom;

#[derive(Component)]
pub struct Ring {
    pub index: usize,
    pub max_electrons: usize,
    pub cycle_timer: Option<Timer>,
}

impl Ring {
    pub fn new(index: usize) -> Self {
        let max_electrons = if index == 0 { 2 } else { 8 };
        Self {
            index,
            max_electrons,
            cycle_timer: None,
        }
    }
    pub fn radius(&self) -> f32 {
        100. + (self.index as f32 * 50.)
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct InNucleus;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Neutron;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Proton;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Electron;

#[derive(Bundle)]
pub struct ElectronBundle {
    name: Name,
    marker: Electron,
    sprite: SpriteBundle,
    base_transform: BaseTransform,
    movement_controller: MovementController,
    revolution_controller: RevolutionController,
    pickable_bundle: PickableBundle,
    on_click: On<Pointer<Click>>,
}
impl ElectronBundle {
    pub(crate) fn new(
        ring_index: usize,
        index: usize,
        radius: f32,
        image_handles: &HandleMap<ImageKey>,
    ) -> Self {
        let rotation = if ring_index == 0 {
            360. / 2. * index as f32
        } else {
            360. / 8. * index as f32
        }
        .to_radians();
        let mut transform = Transform::from_xyz(radius, 0.0, 10.);
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(rotation));
        let base_transform = BaseTransform(transform);
        Self {
            name: Name::new(format!("Electron {}.{}", ring_index + 1, index + 1)),
            marker: Electron,
            sprite: SpriteBundle {
                texture: image_handles[&ImageKey::Electron].clone_weak(),
                transform: base_transform.0,
                ..Default::default()
            },
            base_transform,
            movement_controller: MovementController::new(),
            revolution_controller: RevolutionController::new(1, 350_f32.to_radians()),
            pickable_bundle: PickableBundle::default(), // <- Makes the mesh pickable.
            on_click: On::<Pointer<Click>>::target_component_mut::<MovementController>(
                |_click, controller| {
                    controller.add_count = true;
                },
            ),
        }
    }
}

fn spawn_atom_scene(_trigger: Trigger<SpawnAtomScene>, mut commands: Commands) {
    commands.spawn((
        Atom,
        TransformBundle::default(),
        InheritedVisibility::default(),
        StateScoped(Screen::Playing),
    ));
}

#[derive(Event)]
pub struct AddProton;

fn add_proton(
    _trigger: Trigger<AddProton>,
    mut commands: Commands,
    query_atom: Query<Entity, With<Atom>>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let Ok(atom) = query_atom.get_single() else {
        return;
    };
    commands.entity(atom).with_children(|parent| {
        parent.spawn((
            Proton,
            InNucleus,
            SpriteBundle {
                texture: image_handles[&ImageKey::Proton].clone_weak(),
                transform: Transform::from_xyz(
                    (random::<f32>() * 2.) - 1.,
                    (random::<f32>() * 2.) - 1.,
                    0.,
                ),
                ..Default::default()
            },
        ));
    });
}
#[derive(Event)]
pub struct AddProtonNeutron;

fn add_proton_neutron(
    _trigger: Trigger<AddProtonNeutron>,
    mut commands: Commands,
    query_atom: Query<Entity, With<Atom>>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let Ok(atom) = query_atom.get_single() else {
        return;
    };
    commands.entity(atom).with_children(|parent| {
        parent.spawn((
            Proton,
            InNucleus,
            SpriteBundle {
                texture: image_handles[&ImageKey::Proton].clone_weak(),
                transform: Transform::from_xyz(
                    (random::<f32>() * 2.) - 1.,
                    (random::<f32>() * 2.) - 1.,
                    0.,
                ),
                ..Default::default()
            },
        ));
        parent.spawn((
            Neutron,
            InNucleus,
            SpriteBundle {
                texture: image_handles[&ImageKey::Neutron].clone_weak(),
                transform: Transform::from_xyz(
                    (random::<f32>() * 2.) - 1.,
                    (random::<f32>() * 2.) - 1.,
                    0.,
                ),
                ..Default::default()
            },
        ));
    });
}

fn cycle_rings(
    mut query_ring: Query<(&mut Ring, &Children)>,
    mut query_electrons: Query<&mut RevolutionController, With<Electron>>,
    time: Res<Time>,
) {
    for (mut ring, children) in &mut query_ring {
        if let Some(timer) = ring.cycle_timer.as_mut() {
            timer.tick(time.delta());
            if timer.finished() {
                for child in children.iter() {
                    if let Ok(mut r) = query_electrons.get_mut(*child) {
                        r.add_count();
                    }
                }
            }
        }
    }
}
