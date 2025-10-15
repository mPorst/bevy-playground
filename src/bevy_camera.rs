use crate::components::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

const CAMERA_SENSITIVITY: f32 = 0.005;

pub struct YeetCameraPlugin;

impl Plugin for YeetCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_update);
    }
}

fn camera_update(
    mut player: Single<&mut Transform, With<Player>>,
    mouse_delta: Res<AccumulatedMouseMotion>,
) {
    let deltax = mouse_delta.delta.x * CAMERA_SENSITIVITY;
    let deltay = mouse_delta.delta.y * CAMERA_SENSITIVITY;
    if deltax.abs() > 0.0 || deltay.abs() > 0.0 {
        let original_rotation = player.rotation;

        let yaw = Quat::from_euler(EulerRot::XYZ, 0.0, -deltax, 0.0);
        let pitch = Quat::from_euler(EulerRot::XYZ, -deltay, 0.0, 0.0);
        // multiplication from right
        player.rotation = (original_rotation * yaw * pitch).normalize();
    }
}
