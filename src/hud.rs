use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(200.),
                    border: UiRect::all(Val::Px(2.0)),

                    ..default()
                },
                ..default()
            });
        });
}
