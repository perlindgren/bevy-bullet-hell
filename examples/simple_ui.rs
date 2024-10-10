use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

// use crate::common::*;

pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(200.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(100.),
                    border: UiRect::all(Val::Px(20.0)),

                    ..default()
                },
                background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                ..default()
            });
            parent.spawn(TextBundle::from_section(
                "hello",
                TextStyle {
                    color: GOLD.into(),
                    ..default()
                },
            ));
            parent.spawn(TextBundle::from_section("again", TextStyle { ..default() }));
        });
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        // .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup);

    // {
    //     app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
    //         .add_systems(Update, toggle_overlay);
    // }

    app.run();
}
