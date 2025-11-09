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
    query: Query<(&mut Transform, &mut Storage), With<AntHive>>,
) {
    const ANT_SPAWN_COST: f32 = 25.0;

    for (transform, storage) in query.iter() {
        if storage.stored_ore > ANT_SPAWN_COST {
            println!("hi")
        }
    }
}
