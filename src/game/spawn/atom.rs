//! Spawn the atom scene

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        movement::{MovementController, RevolutionCount, Revolve, RevolveZone},
    },
    screen::Screen,
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_atom_scene);
    app.register_type::<Electron>();
    app.register_type::<Proton>();
}

#[derive(Event, Debug)]
pub struct SpawnAtomScene;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Proton;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Electron;

fn spawn_atom_scene(
    _trigger: Trigger<SpawnAtomScene>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Proton,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0, 15.0))),
            material: materials.add(Color::srgb(0.1, 1.0, 0.1)),
            ..Default::default()
        },
        // WrapWithinWindow,
        // player_animation,
        StateScoped(Screen::Playing),
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0, 30.0))),
            material: materials.add(Color::srgb(1.0, 0.1, 0.1)),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
    commands.spawn((
        Electron,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0, 15.0))),
            material: materials.add(Color::srgb(0.1, 1.0, 0.1)),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..Default::default()
        },
        MovementController::new(),
        Revolve { speed: 3.0 },
        RevolutionCount::new(2),
        RevolveZone::new(0.0, 0.1),
        StateScoped(Screen::Playing),
    ));
}
