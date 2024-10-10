use bevy::{
    color::palettes::css::*,
    // diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

// use crate::common::*;
use bevy_bullet_hell::{
    common::*,
    weapon::{self, WeaponsResource},
};

#[derive(Component)]
struct Ammo;

pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}

pub fn setup_hid(mut commands: Commands, weapons: Res<WeaponsResource>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(200.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(150.),
                        border: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: DIM_GRAY.into(),
                    border_color: DARK_GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Ammo,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                color: LIGHT_CYAN.into(),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            left: Val::Px(10.0),
                            top: Val::Px(10.0),
                            ..default()
                        }),
                    ));
                });
            // parent
            // .spawn(NodeBundle {
            //     style: Style {
            //         width: Val::Px(200.),
            //         height: Val::Px(150.),
            //         border: UiRect::all(Val::Px(20.0)),
            //         justify_content: JustifyContent::Center,
            //         // position_type: PositionType::Absolute,
            //         ..default()
            //     },
            //     background_color: DIM_GRAY.into(),
            //     border_color: DARK_GRAY.into(),

            //     ..default()
            // })
            // .with_children(|parent| {
            parent.spawn(ImageBundle {
                transform: Transform::from_translation((0.0, 0.0, 102.0).into())
                    .with_scale((2.0, 2.0, 1.0).into()),
                image: UiImage {
                    texture: weapons.weapons[0].image.clone(),
                    ..default()
                },

                ..default()
            });
            // })

            parent.spawn(TextBundle::from_section("again", TextStyle { ..default() }));
        });
}

fn hid_system(state: Res<State>, mut query: Query<&mut Text, With<Ammo>>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("{}", state.ammo);
}

#[derive(Resource)]
struct State {
    ammo: u32,
}

fn state_setup(mut commands: Commands) {
    commands.insert_resource(State { ammo: 0 });
}

fn keyboard_input(mut state: ResMut<State>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("space");
        state.ammo += 1;
        // Space was pressed
    }
    if keys.just_released(KeyCode::ControlLeft) {
        // Left Ctrl was released
    }
    if keys.pressed(KeyCode::KeyW) {
        // W is being held down
    }
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        // Either the left or right shift are being held down
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Backspace]) {
        // Either delete or backspace was just pressed
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(
            Startup,
            (setup, weapon::setup, setup_hid, state_setup).chain(),
        )
        .add_systems(Update, (keyboard_input, hid_system));

    // {
    //     app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
    //         .add_systems(Update, toggle_overlay);
    // }

    app.run();
}
