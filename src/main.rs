use bevy::{color::palettes::tailwind, prelude::*};

use crate::bevy_camera::*;
use crate::components::*;

mod bevy_camera;
mod components;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(YeetCameraPlugin);
    app.add_systems(Startup, (setup_world, setup_player, setup_lights));

    app.run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = meshes.add(Sphere::new(2.0));
    let sphere_material = materials.add(Color::from(tailwind::BLUE_700));
    commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(sphere_material.clone()),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Player,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn setup_lights(mut commands: Commands) {
    commands.spawn(AmbientLight {
        color: Color::from(tailwind::SKY_950),
        ..default()
    });

    commands.spawn((
        PointLight {
            color: Color::from(tailwind::YELLOW_100),
            intensity: 40_000_000.0, // this value seems to create a good brightness of the pointlight
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0),
    ));
}
