use crate::{
    common::*,
    selector::{Hand, SelectorResource},
    weapon::WeaponsResource,
};
use bevy::{color::palettes::css::*, prelude::*};

#[derive(Component)]
pub struct InHand(Hand);

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
                InHand(Hand::Left),
                ImageBundle {
                    transform: Transform::from_translation((2.0, 2.0, 102.0).into())
                        .with_scale((2.0, 2.0, 1.0).into()),
                    image: UiImage {
                        texture: weapons.texture.clone(),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: weapons.texture_atlas_layout.clone(),
                    index: 0,
                },
            ));
            parent.spawn((
                InHand(Hand::Right),
                ImageBundle {
                    transform: Transform::from_translation((0.0, 0.0, 102.0).into())
                        .with_scale((2.0, 2.0, 1.0).into()),
                    image: UiImage {
                        texture: weapons.texture.clone(),
                        ..default()
                    },

                    ..default()
                },
                TextureAtlas {
                    layout: weapons.texture_atlas_layout.clone(),
                    index: 1,
                },
            ));

            parent.spawn(TextBundle::from_section("again", TextStyle { ..default() }));
        });
}

pub fn update_system(
    state: Res<SelectorResource>,
    mut _query: Query<&mut Text>,
    mut hand_q: Query<(&mut TextureAtlas, &InHand)>,
) {
    for (mut texture_atlas, InHand(hand)) in hand_q.iter_mut() {
        match hand {
            Hand::Left => {
                if let Some(i) = state.current_left {
                    texture_atlas.index = state.texture_index[i as usize] as usize;
                }
            }

            Hand::Right => {
                if let Some(i) = state.current_right {
                    texture_atlas.index = state.texture_index[i as usize] as usize;
                }
            }
        }
    }
    //     text.sections[0].value = format!("{}", state.ammo);
}
