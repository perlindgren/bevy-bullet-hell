use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    common::CustomCamera,
    player::{DeltaResource, PlayerResource},
    shooting::ShotEvent,
};

pub fn update_system(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    player_resource: Res<PlayerResource>,
    button_inputs: Res<ButtonInput<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<CustomCamera>>,
    mut delta_r: ResMut<DeltaResource>,
    mut fire_lazer_ew: EventWriter<ShotEvent>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_windows.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let delta = world_position - player_resource.player_pos;
        let delta = delta / 500.0;
        let delta = delta.clamp_length(0.0, 1.0);
        delta_r.aim_delta = delta;
        if button_inputs.just_pressed(MouseButton::Left) {
            fire_lazer_ew.send(ShotEvent);
        }

        if button_inputs.just_pressed(MouseButton::Right) {
            fire_lazer_ew.send(ShotEvent);
        }
    }
}

pub fn reset_vector(mut delta_r: ResMut<DeltaResource>) {
    delta_r.aim_delta = (0.0, 0.0).into();
}
