use crate::components::*;
use bevy::{color::palettes::tailwind, log::*, prelude::*};
use rand::prelude::*;

pub struct YeetHiveLogicPlugin;

impl Plugin for YeetHiveLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hive_agent_logic);
    }
}

fn hive_agent_logic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, &mut Storage), With<AntHive>>,
) {
    const ANT_SPAWN_COST: f32 = 25.0;

    for (mut transform, mut storage) in query.iter_mut() {
        if storage.stored_ore >= ANT_SPAWN_COST {
            spawn_new_ant(&mut commands, &mut meshes, &mut materials, &mut transform);
            storage.stored_ore -= ANT_SPAWN_COST;
            println!("spawned new ant");
        }
    }
}

fn spawn_new_ant(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    hive_pos: &mut Transform,
) {
    let ellipsoid_mesh = meshes.add(Capsule3d::new(1.0, 4.0));
    let sphere_material = materials.add(Color::from(tailwind::LIME_500));
    let mut ant_pos = hive_pos.clone();
    ant_pos.translation = ant_pos.translation + ant_pos.forward().normalize() * 3.0;

    commands.spawn((
        Mesh3d(ellipsoid_mesh),
        MeshMaterial3d(sphere_material.clone()),
        ant_pos,
        Asteroid,
        Storage {
            stored_ore: 0.0,
            max_stored_ore: 10.0,
        },
        Collidable,
    ));
}
