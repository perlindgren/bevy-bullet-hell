use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    common::*,
    player::{PlayerComponent, PlayerResource},
};

#[derive(Event)]
pub struct ShotEvent;

#[derive(Component)]
pub struct ShotComponent(Vec2);

#[derive(Component)]
pub struct BombComponent;

#[derive(Resource)]
pub struct ShootingResource {
    shot_image: Handle<Image>,
    _bomb_image: Handle<Image>,
}

pub fn new_shot_system(
    mut commands: Commands,
    shooting_resource: Res<ShootingResource>,
    player_resource: Res<PlayerResource>,
    mut player_er: EventReader<ShotEvent>,
) {
    let speed = player_resource.aim_pos - player_resource.player_pos;
    for _ in player_er.read() {
        debug!("shot_event received");
        debug!("speed {:?} {}", speed, speed.normalize());
        commands.spawn((
            ShotComponent(speed.normalize()),
            SpriteBundle {
                texture: shooting_resource.shot_image.clone(),
                transform: Transform::from_xyz(
                    player_resource.player_pos.x,
                    player_resource.player_pos.y,
                    0.0,
                ),
                ..default()
            },
            Collider::rectangle(10.0, 10.0),
            Sensor,
            RigidBody::Dynamic,
        ));
    }
}

pub fn update_system(
    mut commands: Commands,
    time: Res<Time>,

    mut shot_query: Query<(Entity, &mut Transform, &ShotComponent)>,
) {
    for (entity, mut transform, ShotComponent(speed)) in shot_query.iter_mut() {
        transform.translation += speed.extend(0.0) * SHOT_SPEED * time.delta_seconds();
        // if out of bounds then despawn
        if !SCREEN_RECT.contains((transform.translation.x, transform.translation.y).into()) {
            debug!("despawn");
            commands.entity(entity).despawn();
        }
    }
}

pub fn collider_system(
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerComponent>>,
    mut collision_query: Query<(Entity, &CollidingEntities), With<ShotComponent>>,
) {
    let player_entity = player_query.single();

    for (shot_entity, colliding_entities) in &mut collision_query {
        for entity in colliding_entities.iter() {
            if *entity != player_entity {
                trace!("hit target");
                commands.entity(shot_entity).despawn();
                commands.entity(*entity).despawn();
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ShootingResource {
        shot_image: asset_server.load("sprites/cross.png"),
        _bomb_image: asset_server.load("sprites/cross.png"),
    });
}
