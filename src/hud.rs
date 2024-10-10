use bevy::{color::palettes::css::*, prelude::*};

#[derive(Component)]
struct LeftWeapon;

#[derive(Component)]
struct RightWeapon;

use crate::{common::*, selector::SelectorResource, weapon::WeaponsResource};
pub fn setup(mut commands: Commands, weapons: Res<WeaponsResource>) {
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
                        // Ammo,
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
            parent.spawn((
                LeftWeapon,
                ImageBundle {
                    transform: Transform::from_translation((0.0, 0.0, 102.0).into())
                        .with_scale((2.0, 2.0, 1.0).into()),
                    image: UiImage {
                        texture: weapons.weapons[0].image.clone(),
                        ..default()
                    },

                    ..default()
                },
            ));
            parent.spawn((
                LeftWeapon,
                ImageBundle {
                    transform: Transform::from_translation((0.0, 0.0, 102.0).into())
                        .with_scale((2.0, 2.0, 1.0).into()),
                    image: UiImage {
                        texture: weapons.weapons[1].image.clone(),
                        ..default()
                    },

                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section("again", TextStyle { ..default() }));
        });
}

fn update_system(state: Res<SelectorResource>, mut query: Query<&mut Text>) {
    //     let mut text = query.single_mut();
    //     text.sections[0].value = format!("{}", state.ammo);
}
