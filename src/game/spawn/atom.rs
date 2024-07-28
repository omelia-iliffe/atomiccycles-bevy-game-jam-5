//! Spawn the atom scene

use crate::game::assets::{HandleMap, ImageKey};
use crate::game::movement::BaseTransform;
use crate::game::upgrades::Upgrades;
use crate::{
    game::movement::{MovementController, RevolutionController, Revolve},
    screen::Screen,
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_atom_scene);
}

#[derive(Event, Debug)]
pub struct SpawnAtomScene;

#[derive(Component)]
pub struct Atom;

#[derive(Component)]
pub struct Ring {
    pub index: usize,
    pub max_electrons: usize,
}

impl Ring {
    pub fn new(index: usize) -> Self {
        let max_electrons = if index == 0 { 2 } else { 8 };
        Self {
            index,
            max_electrons,
        }
    }
    pub fn radius(&self) -> f32 {
        100. + (self.index as f32 * 50.)
    }
}

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
    revolve: Revolve,
    revolution_controller: RevolutionController,
    upgrades: Upgrades,
    state_scoped: StateScoped<Screen>,
    pickable_bundle: PickableBundle,
    on_click: On<Pointer<Click>>,
}
impl ElectronBundle {
    fn new(
        ring_index: usize,
        index: usize,
        radius: f32,
        image_handles: &HandleMap<ImageKey>,
    ) -> Self {
        let base_transform = BaseTransform(Transform::from_xyz(radius, 0.0, 10.));
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
            revolve: Revolve::new(3.0),
            revolution_controller: RevolutionController::new(1, 350_f32.to_radians()),
            state_scoped: StateScoped(Screen::Playing),
            pickable_bundle: PickableBundle::default(), // <- Makes the mesh pickable.
            upgrades: Upgrades::electron(),
            on_click: On::<Pointer<Click>>::target_component_mut::<MovementController>(
                |_click, controller| {
                    controller.add_count = true;
                },
            ),
        }
    }
}

fn spawn_atom_scene(
    _trigger: Trigger<SpawnAtomScene>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Atom,
            TransformBundle::default(),
            InheritedVisibility::default(),
        ))
        .with_children(|parent| {
            // Spawn Proton
            parent.spawn((
                Proton,
                SpriteBundle {
                    texture: image_handles[&ImageKey::Proton].clone_weak(),
                    ..Default::default()
                },
                StateScoped(Screen::Playing),
            ));
            // Spawn First Ring
            let ring = Ring::new(0);
            let ring_radius = ring.radius();
            let ring_index = ring.index;
            parent
                .spawn((
                    ring,
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Circle::new(ring_radius))),
                        material: materials.add(Color::srgba_u8(0x28, 0x66, 0x6e, 0x66)),
                        transform: Transform::from_xyz(0., 0., -100.),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(ElectronBundle::new(
                        ring_index,
                        0,
                        ring_radius,
                        image_handles.as_ref(),
                    ));
                });
        });
}

pub fn add_ring(
    commands: &mut Commands,
    query_atom: &Query<(Entity, &Children), With<Atom>>,
    query_rings: &Query<&Ring>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,

) -> bool {
    let (atom, children) = query_atom.get_single().unwrap();
    let ring_count = children.iter().filter(|child| query_rings.get(**child).is_ok()).count();

    let ring = Ring::new(ring_count);
    let ring_radius = ring.radius();
    commands.entity(atom).with_children(|parent| {
        parent
            .spawn((
                ring,
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle::new(ring_radius))),
                    material: materials.add(Color::srgba_u8(0x28, 0x66, 0x6e, 0x66)),
                    transform: Transform::from_xyz(0., 0., -100.),
                    ..default()
                },
            ));
    });

    true

}

pub fn add_electron(
    commands: &mut Commands,
    image_handles: &HandleMap<ImageKey>,
    query: &Query<(Entity, Option<&Children>, &Ring)>,
    query_electrons: &Query<(&Parent, &Electron)>,
) -> bool {
    let rings = query
        .iter()
        .sort_by_key::<&Ring, _>(|ring| ring.index)
        .collect::<Vec<_>>();

    let Some((parent, ring, index)) = rings.into_iter().find_map(|(parent, maybe_children, ring)| {
        let mut electron_count = 0;
        if let Some(children) = maybe_children {
            for child in children {
                if query_electrons.get(*child).is_ok() {
                    electron_count += 1;
                }
            }
        }
        if electron_count < ring.max_electrons {
            Some((parent, ring, electron_count))
        } else {
            log::info!("Ring {} is full", ring.index);
            None
        }
    }) else {
        return false;
    };

    commands.entity(parent).with_children(|parent| {
        parent.spawn(ElectronBundle::new(
            ring.index,
            index,
            ring.radius(),
            image_handles,
        ));
    });

    true
}
