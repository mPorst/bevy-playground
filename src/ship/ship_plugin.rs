use crate::ship::ship_system::*;
use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ship_update);
    }
}
