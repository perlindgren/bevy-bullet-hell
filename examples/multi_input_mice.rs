// mouse events
use bevy::prelude::*;
use input_linux_tools::mouse::*;
use std::{thread, time};

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
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, update_system)
        .run();
}

#[derive(Component)]
pub struct Player(usize);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("sprites/cross.png");
    commands.spawn((
        Player(0),
        SpriteBundle {
            texture: texture.clone(),
            ..default()
        },
    ));

    commands.spawn((
        Player(1),
        SpriteBundle {
            texture,
            ..default()
        },
    ));

    let mut mice = vec![];
    mice.push(Mouse::new_first_match("Laser", false).unwrap());
    mice.push(Mouse::new_first_match("Dell", false).unwrap());
    commands.insert_resource(Mice { mice });
}

#[derive(Resource)]
pub struct Mice {
    pub mice: Vec<Mouse>,
}

pub fn update_system(
    time: Res<Time>,
    mice_input: Res<Mice>,
    mut player_q: Query<(&mut Transform, &Player)>,
) {
    let speed = 50.0;
    let time_speed_delta = time.delta_seconds() * speed;
    for (index, mouse) in mice_input.mice.iter().enumerate() {
        let mut nr_events_in_frame = 0;
        while let Some(event) = mouse.read() {
            nr_events_in_frame += 1;
            // println!("event {:?}", event);
            match event {
                MouseEvent::MotionEvent(motion) => {
                    if let Some((mut t, _)) =
                        player_q.iter_mut().find(|(_t, Player(nr))| (*nr == index))
                    {
                        t.translation.x += motion.delta.x * time_speed_delta;
                        t.translation.y -= motion.delta.y * time_speed_delta; // evdev delta in other direction
                    }
                }
                _ => {}
            }
        }
        if nr_events_in_frame != 0 {
            println!("nr events in frame {}", nr_events_in_frame);
        }
    }
}
