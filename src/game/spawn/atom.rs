//! Spawn the atom scene

use crate::game::assets::{HandleMap, ImageKey};
use crate::game::movement::BaseTransform;
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
    app.register_type::<Electron>();
    app.register_type::<Proton>();
}

#[derive(Event, Debug)]
pub struct SpawnAtomScene;

#[derive(Component)]
pub struct Atom;

#[derive(Component)]
pub struct Ring;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Proton;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Electron;

#[derive(Bundle)]
pub struct ElectronBundle {
    marker: Electron,
    sprite: SpriteBundle,
    base_transform: BaseTransform,
    movement_controller: MovementController,
    revolve: Revolve,
    revolution_controller: RevolutionController,
    state_scoped: StateScoped<Screen>,
    pickable_bundle: PickableBundle,
    on_click: On<Pointer<Click>>,
}
impl ElectronBundle {
    fn new(radius: f32, image_handles: Res<HandleMap<ImageKey>>) -> Self {
        let base_transform = BaseTransform(Transform::from_xyz(radius, 0.0, 10.));
        Self {
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
            let ring_1_radius = 100.;
            parent
                .spawn((
                    Ring,
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Circle::new(ring_1_radius))),
                        material: materials.add(Color::srgba_u8(0x28, 0x66, 0x6e, 0x66)),
                        transform: Transform::from_xyz(0., 0., -100.),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(ElectronBundle::new(ring_1_radius, image_handles));
                });
        });
}
