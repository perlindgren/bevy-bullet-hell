use bevy::prelude::*;

use crate::player::DeltaResource;

pub fn update_system(keys: Res<ButtonInput<KeyCode>>, mut delta_r: ResMut<DeltaResource>) {
    let mut total_x = 0.0;
    let mut total_y = 0.0;
    // Key Mapping ?
    if keys.pressed(KeyCode::KeyW) {
        debug!("KeyW");
        total_y += 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        debug!("KeyA");
        total_x -= 1.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        debug!("KeyS");
        total_y -= 1.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        debug!("KeyD");
        total_x += 1.0;
    }

    delta_r.player_delta.x = total_x;
    delta_r.player_delta.y = total_y;
}
