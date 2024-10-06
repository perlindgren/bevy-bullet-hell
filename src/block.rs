use crate::common::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use rand::random;

#[derive(Component)]
pub struct Block {
    rotation: f32,
    speed: Vec2,
}

pub fn update_system(time: Res<Time>, mut rect_query: Query<(&mut Transform, &mut Block)>) {
    for (mut transform, mut block) in rect_query.iter_mut() {
        transform.rotate_z(block.rotation * time.delta_seconds() * BLOCK_ROTATION_SPEED);
        let trans: Vec3 =
            transform.translation + block.speed.extend(0.0) * time.delta_seconds() * BLOCKS_SPEED;
        if trans.x < -HALF_WIDTH || trans.x > HALF_WIDTH {
            block.speed.x *= -1.0;
        }
        if trans.y < -HALF_HEIGHT || trans.y > HALF_HEIGHT {
            block.speed.y *= -1.0;
        }
        transform.translation = trans;
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));

    for i in 0..10 {
        let color = Color::linear_rgba(random(), random(), random(), 1.0);
        commands.spawn((
            Block {
                rotation: random(),
                speed: ((random::<f32>() - 0.5), (random::<f32>() - 0.5)).into(),
            },
            MaterialMesh2dBundle {
                mesh: shape.clone(),
                material: materials.add(color),
                transform: Transform::from_xyz(
                    (random::<f32>() - 0.5) * HALF_WIDTH,
                    (random::<f32>() - 0.5) * HALF_HEIGHT,
                    1.0 + i as f32, // different z to avoid flicker
                ),

                ..default()
            },
        ));
    }
}