pub mod yeet_camera;

use crate::components::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

pub struct YeetCameraPlugin;

impl Plugin for YeetCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                yeet_camera::camera_view_update,
                yeet_camera::camera_move_update,
            ),
        );
    }
}
