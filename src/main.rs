use bevy::prelude::*;

mod camera;
mod components;
mod logic {
    pub mod ant_logic;
    pub mod hive_logic;
}
mod common;
mod inventory;
mod setup_world;
mod ship;

use crate::camera::*;
use crate::components::*;
use crate::logic::ant_logic::*;
use crate::logic::hive_logic::*;
use crate::setup_world::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(YeetCameraPlugin)
        .add_plugins(YeetSetupWorldPlugin)
        .add_plugins(YeetAntLogicPlugin)
        .add_plugins(YeetHiveLogicPlugin);
    app.add_systems(Startup, setup_player);

    app.run();
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Player,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
