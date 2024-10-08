use avian2d::prelude::*;
use bevy::{
    color::palettes::css::{GREEN, RED, WHITE},
    prelude::*,
};

use bevy_inspector_egui::prelude::*;
// use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::common::*;

#[derive(Component)]
pub struct PlayerComponent;

#[derive(Component)]
pub struct CrossComponent;

#[derive(Resource, Default)]
pub struct DeltaResource {
    pub player_delta: Vec2,
    pub aim_delta: Vec2,
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PlayerResource {
    pub player_pos: Vec2,
    pub aim_pos: Vec2,
}

pub fn update_system(
    time: Res<Time>,

    dr: Res<DeltaResource>,
    mut pr: ResMut<PlayerResource>,
    mut player_query: Query<&mut Transform, With<PlayerComponent>>,
    mut cross_query: Query<&mut Transform, (With<CrossComponent>, Without<PlayerComponent>)>,

    mut gizmos: Gizmos,
) {
    let mut pt = player_query.single_mut();
    let mut ct = cross_query.single_mut();

    pr.player_pos += dr.player_delta * PLAYER_SPEED * time.delta_seconds();
    pr.aim_pos = pr.player_pos + dr.aim_delta * 500.0;

    pr.player_pos = pr.player_pos.clamp(SCREEN_MIN, SCREEN_MAX);

    pt.translation.x = pr.player_pos.x;
    pt.translation.y = pr.player_pos.y;

    ct.translation.x = pr.aim_pos.x;
    ct.translation.y = pr.aim_pos.y;

    gizmos.line_gradient_2d(pr.player_pos, pr.aim_pos, RED, GREEN);
}

pub fn collider_system(mut query: Query<(&mut Sprite, &CollidingEntities), With<PlayerComponent>>) {
    for (mut sprite, colliding_entities) in &mut query {
        if colliding_entities.0.is_empty() {
            sprite.color = WHITE.into();
        } else {
            debug!("--- collision with player ---");
            sprite.color = RED.into();
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DeltaResource::default());
    commands.insert_resource(PlayerResource::default());

    commands.spawn((
        PlayerComponent,
        RigidBody::Dynamic,
        Collider::rectangle(10.0, 10.0),
        Sensor,
        SpriteBundle {
            texture: asset_server.load("sprites/cross.png"),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        MassPropertiesBundle::new_computed(&Collider::rectangle(10.0, 10.0), 1.0),
    ));

    commands.spawn((
        CrossComponent,
        SpriteBundle {
            texture: asset_server.load("sprites/cross.png"),
            ..default()
        },
    ));
}
