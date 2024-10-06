use crate::common::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::random;
use std::time::Duration;

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
    // mut fire_lazer_event: EventReader<FireLazerEvent>,
    dr: Res<DeltaResource>,
    mut pr: ResMut<PositionResource>,
    mut player_query: Query<&mut Transform, With<PlayerComponent>>,
    mut cross_query: Query<&mut Transform, (With<CrossComponent>, Without<PlayerComponent>)>,
    mut rect_query: Query<
        (&mut Transform, &Rotate),
        (Without<PlayerComponent>, Without<CrossComponent>),
    >,
) {
    let mut pt = player_query.single_mut();
    let mut ct = cross_query.single_mut();

    pr.player_pos += dr.player_delta * 10.0;
    pr.aim_pos = pr.player_pos + dr.aim_delta * 500.0;

    pt.translation.x = pr.player_pos.x;
    pt.translation.y = pr.player_pos.y;

    ct.translation.x = pr.aim_pos.x;
    ct.translation.y = pr.aim_pos.y;

    for (mut transform, Rotate(rotate)) in rect_query.iter_mut() {
        transform.rotate_z(*rotate);
    }
    // *transform = transform.looking_at(pt.translation, Dir3::Z);
}

#[derive(Component)]
pub struct Rotate(f32);

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));

    let color = Color::linear_rgba(1.0, 0.0, 0.0, 1.0);
    let color2 = Color::linear_rgba(0.0, 1.0, 0.0, 1.0);

    commands.spawn((
        Rotate(-0.025),
        MaterialMesh2dBundle {
            mesh: shape.clone(),
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                200.0, 100.0, 0.0,
            ),
            // .with_rotation( Quat::from_axis_angle(axis, angle))  ,
            ..default()
        },
    ));

    commands.spawn((
        Rotate(0.05),
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color2),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -200.0, 100.0, 0.0,
            ),
            // .with_rotation( Quat::from_axis_angle(axis, angle))  ,
            ..default()
        },
    ));

    commands.spawn(
        TextBundle::from_section("Press space to toggle wireframes", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}
