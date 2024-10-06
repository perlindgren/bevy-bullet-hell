use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_bullet_hell::{block, common::*, gamepad, player, shooting};

fn setup(mut commands: Commands) {
    // we might want to setup a custom camera, for now just default
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(RES_X, RES_Y),
                resizable: false,
                title: "Bevy-Bullet-Hell".to_string(),
                desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_event::<shooting::ShotEvent>()
        .add_systems(
            Startup,
            (setup, player::setup, block::setup, shooting::setup).chain(),
        )
        .add_systems(
            Update,
            (
                (
                    gamepad::update_system,
                    player::update_system,
                    block::update_system,
                    shooting::update_system,
                )
                    .before(shooting::new_shot_system),
                (shooting::new_shot_system),
            ),
        )
        .run();
}
