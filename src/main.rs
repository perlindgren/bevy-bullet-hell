use avian2d::{math::*, prelude::*};
use bevy::{
    // core_pipeline::,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::{camera::Viewport, view::visibility::RenderLayers},
    window::WindowResolution,
};

use bevy_bullet_hell::{
    block, camera,
    common::*,
    config::{self, ConfigResource},
    gamepad, hud, keyboard, mouse, player, post_process, post_process2, selector, shooting, tile,
    ui_egui, weapon,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

fn setup(mut commands: Commands) {
    // our main camera, which also holds our UI
    let camera = Camera2dBundle::default();
    println!("camera scale {}", camera.projection.scale);
    commands.spawn((
        camera,
        IsDefaultUiCamera,
        post_process::PostProcessSettings {
            intensity: 0.02,
            ..default()
        },
        post_process2::PostProcessSettings2 {
            intensity: 0.02,
            ..default()
        },
    ));
    // custom camera, used for popup sector
    let custom_camera = Camera2dBundle {
        camera: Camera {
            order: 1,

            viewport: Some(Viewport {
                physical_position: UVec2::new(
                    (1.0 * (HALF_WIDTH - SELECTOR_RADIUS)) as u32,
                    (1.0 * RES_Y
                        - SELECTOR_SIZE as f32
                        - HID_HEIGHT as f32
                        - SELECTOR_BOTTOM as f32) as u32,
                ),
                physical_size: UVec2::new(SELECTOR_SIZE, SELECTOR_SIZE),
                ..default()
            }),
            ..default()
        },
        ..default()
    };
    // custom_camera.projection.scale;
    commands.spawn((CustomCamera, custom_camera, RenderLayers::from_layers(&[1])));
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
            UiMaterialPlugin::<hud::excite::CustomUIMaterial>::default(),
            post_process::PostProcessPlugin,
            post_process2::PostProcessPlugin2,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Gravity(Vector::ZERO))
        .add_event::<shooting::ShotEvent>()
        .add_systems(
            Startup,
            (
                setup,
                config::setup,
                player::setup,
                block::setup,
                shooting::setup,
                tile::setup,
                ui_egui::setup,
                weapon::setup,
                selector::setup,
                hud::fps::setup,
                hud::excite::setup,
                hud::hud_ui::setup,
                post_process::setup,
                post_process2::setup,
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
                block::update_system,
                shooting::new_shot_system,
                shooting::update_system,
                shooting::collider_system.run_if(on_event::<CollisionStarted>()),
                player::collider_system
                    .run_if(on_event::<CollisionStarted>().or_else(on_event::<CollisionEnded>())),
                ui_egui::update_system,
                selector::update_system,
                hud::fps::update_system,
                hud::excite::keyboard_input, // for debugging
                hud::excite::update_system,
                hud::hud_ui::update_system,
                camera::update_system,
                post_process::update_system,
                post_process2::update_system,
                post_process::update_shockwaves,
            )
                .chain(),
        )
        .run();
}
