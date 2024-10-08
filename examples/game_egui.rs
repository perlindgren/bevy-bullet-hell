use avian2d::{math::*, prelude::*};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_bullet_hell::player::PlayerResource;
use bevy_bullet_hell::{block, camera, common::*, gamepad, overlay, player, shooting, tile};
use bevy_ecs_tilemap::prelude::*;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

// `InspectorOptions` are completely optional
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    name: String,
    #[inspector(min = 0.0, max = 1.0)]
    option: f32,
}

fn setup(mut commands: Commands) {
    // we might want to setup a custom camera, for now just default
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(RES_X, RES_Y),
                        resizable: false,
                        title: "Bevy-Bullet-Hell".to_string(),
                        desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(1.0),
        ))
        .init_resource::<Configuration>() // `ResourceInspectorPlugin` won't initialize the resource
        .register_type::<Configuration>() // you need to register your type to display it
        .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        // also works with built-in resources, as long as they are `Reflect`
        .add_plugins(ResourceInspectorPlugin::<Time>::default())
        .add_plugins(ResourceInspectorPlugin::<PlayerResource>::default())
        .add_plugins(TilemapPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Gravity(Vector::ZERO))
        .add_event::<shooting::ShotEvent>()
        .add_systems(
            Startup,
            (
                setup,
                overlay::setup,
                player::setup,
                block::setup,
                shooting::setup,
                tile::setup,
            )
                .chain(),
        )
        .add_systems(
            Update,
            // (
            (
                gamepad::update_system,
                player::update_system,
                player::collider_system,
                block::update_system,
                shooting::new_shot_system,
                shooting::update_system,
                shooting::collider_system,
                overlay::fps_update_system,
                camera::update_system,
            )
                .chain(), // .before(shooting::new_shot_system),
        )
        // )
        .run();
}
