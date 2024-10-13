// custom camera experiment
//

use bevy::prelude::*;
use bevy::render::{camera::Viewport, view::visibility::RenderLayers};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct CustomCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // default camera
    commands.spawn(Camera2dBundle::default());
    // spawn to be rendered by default camera
    commands.spawn((SpriteBundle {
        texture: asset_server.load("sprites/jump.png"),
        ..default()
    },));
    // custom camera
    commands.spawn((
        CustomCamera,
        Camera2dBundle {
            camera: Camera {
                order: 2,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(256, 256),
                    ..default()
                }),
                ..default()
            },
            ..default()
        },
        RenderLayers::from_layers(&[1, 2]),
    ));
    // spawn in both custom camera and default
    commands.spawn((
        RenderLayers::from_layers(&[0, 1]),
        SpriteBundle {
            texture: asset_server.load("sprites/gunt.png"),
            ..default()
        },
    ));
}
