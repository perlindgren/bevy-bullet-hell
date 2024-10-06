use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};

use crate::common::*;

#[derive(Event)]
pub struct FireLazerEvent;

#[derive(Component)]
pub struct PlayerComponent;

#[derive(Component)]
pub struct CrossComponent;

#[derive(Resource, Default)]
pub struct DeltaResource {
    pub player_delta: Vec2,
    pub aim_delta: Vec2,
}

#[derive(Resource, Default)]
pub struct PositionResource {
    pub player_pos: Vec2,
    pub aim_pos: Vec2,
}

pub fn update_system(
    time: Res<Time>,

    dr: Res<DeltaResource>,
    mut pr: ResMut<PositionResource>,
    mut player_query: Query<&mut Transform, With<PlayerComponent>>,
    mut cross_query: Query<&mut Transform, (With<CrossComponent>, Without<PlayerComponent>)>,

    mut gizmos: Gizmos,
) {
    let mut pt = player_query.single_mut();
    let mut ct = cross_query.single_mut();

    pr.player_pos += dr.player_delta * PLAYER_SPEED * time.delta_seconds();
    pr.aim_pos = pr.player_pos + dr.aim_delta * 500.0;

    pt.translation.x = pr.player_pos.x;
    pt.translation.y = pr.player_pos.y;

    ct.translation.x = pr.aim_pos.x;
    ct.translation.y = pr.aim_pos.y;

    // gizmos.line_2d(pr.player_pos, pr.aim_pos, RED);
    gizmos.line_gradient_2d(pr.player_pos, pr.aim_pos, RED, GREEN);
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DeltaResource::default());
    commands.insert_resource(PositionResource::default());
    commands.spawn((
        PlayerComponent,
        SpriteBundle {
            texture: asset_server.load("sprites/cross.png"),
            ..default()
        },
    ));

    commands.spawn((
        CrossComponent,
        SpriteBundle {
            texture: asset_server.load("sprites/cross.png"),
            ..default()
        },
    ));
}
