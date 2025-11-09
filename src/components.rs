use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Asteroid {
    pub amount_ore: f32,
}

#[derive(Component)]
pub struct Ant {
    pub health: f32,
    pub max_health: f32,
    pub stored_ore: f32,
    pub max_stored_ore: f32,
}

#[derive(Component)]
pub struct AntHive {
    pub stored_ore: f32,
}

#[derive(Component)]
pub struct Collidable;
