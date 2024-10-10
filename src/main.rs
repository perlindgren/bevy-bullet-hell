use avian2d::{math::*, prelude::*};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_bullet_hell::{
    block, camera,
    common::*,
    config::{self, *},
    gamepad, hud, keyboard, mouse, overlay, player, selector, shooting, tile, ui, weapon,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

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
            bevy_egui::EguiPlugin,
            DefaultInspectorConfigPlugin,
            FrameTimeDiagnosticsPlugin,
            TilemapPlugin,
        ))
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
                ui::setup,
                config::setup,
                weapon::setup,
                selector::setup,
                hud::setup,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                // there is 100% a better way of doing this, probably split configresource
                // into more specific resources and bundle it?
                keyboard::update_system.run_if(resource_equals::<ConfigResource>(ConfigResource {
                    gamepad: false,
                })),
                mouse::update_system.run_if(resource_equals::<ConfigResource>(ConfigResource {
                    gamepad: false,
                })),
                mouse::reset_vector.run_if(resource_changed::<ConfigResource>),
                gamepad::update_system.run_if(resource_equals::<ConfigResource>(ConfigResource {
                    gamepad: true,
                })),
                player::update_system,
                player::collider_system,
                block::update_system,
                shooting::new_shot_system,
                shooting::update_system,
                shooting::collider_system,
                overlay::fps_update_system,
                camera::update_system,
                ui::update_system,
                selector::update_system,
            )
                .chain(),
        )
        .run();
}
