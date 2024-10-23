// mouse events
use bevy::prelude::*;
use bevy_bullet_hell::{input_cfg, ui_egui_cfg};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use input_linux_tools::{device::*, mouse::*};

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

#[derive(Component)]
pub struct Player(usize);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("sprites/cross.png");

    for i in 0..1 {
        commands.spawn((
            Player(i),
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
    mut player_q: Query<(&mut Transform, &Player)>,
) {
    let speed = 50.0;
    let time_speed_delta = time.delta_seconds() * speed;
    for (index, input) in players_input_r.player_input.iter().enumerate() {
        // if let Some(input) = &input.pos_input {
        //     match input {
        //         DeviceType::Mouse(mouse, _) => {
        //             while let Some(event) = mouse.read() {
        //                 match event {
        //                     MouseEvent::MotionEvent(motion) => {
        //                         if let Some((mut t, _)) =
        //                             player_q.iter_mut().find(|(_t, Player(nr))| (*nr == index))
        //                         {
        //                             t.translation.x += motion.delta.x * time_speed_delta;
        //                             t.translation.y -= motion.delta.y * time_speed_delta;
        //                             // evdev delta in other direction
        //                         }
        //                     }
        //                     _ => {}
        //                 }
        //             }
        //         }
        //         _ => {}
        //     }
        // }
    }
}
