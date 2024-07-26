//! Spawn the atom scene

use crate::{
    game::movement::{MovementController, RevolutionController, Revolve},
    screen::Screen,
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::prelude::*;
use crate::game::movement::BaseTransform;

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
        StateScoped(Screen::Playing),
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0, 30.0))),
            material: materials.add(Color::srgb(1.0, 0.1, 0.1)),
            transform: Transform::from_xyz(100.0, 0.0, -1.0),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
    let base_transform =
        BaseTransform(Transform::from_xyz(100.0, 0.0, 0.0));
    commands.spawn((
        Electron,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(15.0, 15.0))),
            material: materials.add(Color::srgb(0.1, 1.0, 0.1)),
            transform: base_transform.0,
            ..Default::default()
        },
        base_transform,
        MovementController::new(),
        Revolve::new(3.0),
        RevolutionController::new(1, 350_f32.to_radians()),
        StateScoped(Screen::Playing),
        PickableBundle::default(), // <- Makes the mesh pickable.
        On::<Pointer<Click>>::target_component_mut::<MovementController>(|_click, controller| {
            controller.add_count = true;
        }),
    ));
}
