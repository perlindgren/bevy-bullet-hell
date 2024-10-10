use bevy::prelude::*;

use crate::player::DeltaResource;

pub fn update_system(keys: Res<ButtonInput<KeyCode>>, mut target_resource: ResMut<DeltaResource>) {
    let mut total_x = 0.0;
    let mut total_y = 0.0;
    // Key Mapping ?
    if keys.pressed(KeyCode::KeyW) {
        total_y += 1.0
    }

    if keys.pressed(KeyCode::KeyA) || keys.just_released(KeyCode::KeyD) {
        total_x -= 1.0
    }

    if keys.pressed(KeyCode::KeyS) || keys.just_released(KeyCode::KeyW) {
        total_y -= 1.0
    }

    if keys.pressed(KeyCode::KeyD) || keys.just_released(KeyCode::KeyA) {
        total_x += 1.0
    }

    target_resource.player_delta.x = total_x;
    target_resource.player_delta.y = total_y;
}
