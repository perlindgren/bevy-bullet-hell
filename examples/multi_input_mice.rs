// mouse events
use bevy::prelude::*;
use bevy_bullet_hell::{common::NR_PLAYERS, input_cfg, ui_egui_cfg};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use input_linux_tools::{
    device::*,
    input_linux::Key,
    keyboard::{KeyStatus, KeyboardEvent},
    mouse::*,
};
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                    cursor: bevy::window::Cursor {
                        grab_mode: bevy::window::CursorGrabMode::Confined,
                        visible: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
            bevy_framepace::FramepacePlugin,
            bevy_egui::EguiPlugin,
            DefaultInspectorConfigPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup, input_cfg::setup, ui_egui_cfg::setup))
        .add_systems(Update, (update_system, ui_egui_cfg::update_system))
        .run();
}

#[derive(Component, Default)]
pub struct Player {
    id: usize,
    keys_pressed: HashMap<Key, KeyStatus>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("sprites/cross.png");

    for i in 0..NR_PLAYERS {
        commands.spawn((
            Player { id: i, ..default() },
            SpriteBundle {
                texture: texture.clone(),
                ..default()
            },
        ));
    }
}

pub fn update_system(
    time: Res<Time>,
    players_input_r: Res<input_cfg::PlayersInput>,
    mut player_q: Query<(&mut Transform, &mut Player)>,
) {
    for (index, input) in players_input_r.player_input.iter().enumerate() {
        match &input.pos_input.evdev {
            Some(EvDev::Mouse(mouse)) => {
                let speed = 50.0;
                let time_speed_delta = time.delta_seconds() * speed;

                // check if the player is connected to mouse
                if let Some((mut t, _)) =
                    player_q.iter_mut().find(|(_, player)| (player.id == index))
                {
                    // yes connected
                    while let Some(event) = mouse.read() {
                        match event {
                            MouseEvent::MotionEvent(motion) => {
                                t.translation.x += motion.delta.x * time_speed_delta;
                                t.translation.y -= motion.delta.y * time_speed_delta;
                                // evdev delta in other direction
                            }
                            _ => {}
                        }
                    }
                }
            }

            Some(EvDev::Keyboard(keyboard)) => {
                if let Some((mut t, mut player)) =
                    player_q.iter_mut().find(|(_, player)| (player.id == index))
                {
                    // read events, and store their status
                    while let Some(KeyboardEvent { key, status }) = keyboard.read() {
                        player.keys_pressed.insert(key, status);
                    }

                    // match directions
                    let left = player
                        .keys_pressed
                        .get(&Key::A)
                        .map_or(false, |status| status == &KeyStatus::Pressed);
                    let right = player
                        .keys_pressed
                        .get(&Key::D)
                        .map_or(false, |status| status == &KeyStatus::Pressed);

                    let up = player
                        .keys_pressed
                        .get(&Key::W)
                        .map_or(false, |status| status == &KeyStatus::Pressed);
                    let down = player
                        .keys_pressed
                        .get(&Key::S)
                        .map_or(false, |status| status == &KeyStatus::Pressed);

                    let speed = 400.0;
                    let time_speed_delta = time.delta_seconds() * speed;
                    //   update player position
                    t.translation.y += time_speed_delta
                        * (if up { 1.0 } else { 0.0 } - if down { 1.0 } else { 0.0 });
                    t.translation.x += time_speed_delta
                        * (if right { 1.0 } else { 0.0 } - if left { 1.0 } else { 0.0 });
                }
            }
            _ => {}
        }
    }
}
