use crate::common::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Rotate(f32);

pub fn update_system(time: Res<Time>, mut rect_query: Query<(&mut Transform, &Rotate)>) {
    for (mut transform, Rotate(rotate)) in rect_query.iter_mut() {
        transform.rotate_z(*rotate * time.delta_seconds() * ROTATION_SPEED);
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));

    let color = Color::linear_rgba(1.0, 0.0, 0.0, 1.0);
    let color2 = Color::linear_rgba(0.0, 1.0, 0.0, 1.0);

    commands.spawn((
        Rotate(-0.025),
        MaterialMesh2dBundle {
            mesh: shape.clone(),
            material: materials.add(color),
            transform: Transform::from_xyz(200.0, 100.0, 0.0),

            ..default()
        },
    ));

    commands.spawn((
        Rotate(0.05),
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color2),
            transform: Transform::from_xyz(-200.0, 100.0, 0.0),
            ..default()
        },
    ));
}
