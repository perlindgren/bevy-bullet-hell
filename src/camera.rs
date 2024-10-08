use crate::player::PlayerResource;
use bevy::prelude::*;

pub fn update_system(
    player_resource: Res<PlayerResource>,
    mut player_query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = player_query.single_mut();

    transform.translation = player_resource.player_pos.extend(0.0) * 0.25;
}
