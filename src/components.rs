use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct AntHive;

#[derive(Component)]
pub struct Storage {
    pub stored_ore: f32,
    pub max_stored_ore: f32,
}

#[derive(Component)]
pub struct Collidable;

#[derive(Component, Default, Clone, Copy)]
pub struct Target {
    pub target: Option<Entity>,
}

#[derive(Component, Default, Clone, Copy)]
pub struct HomeBase {
    pub home_base: Option<Entity>,
}
