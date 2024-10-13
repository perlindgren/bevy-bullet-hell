use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use std::f32::consts::{PI, TAU};

use crate::{common::*, player::PlayerResource, weapon::WeaponsResource};

#[derive(Component)]
pub struct Selector;

#[derive(Component)]
pub struct SelectorIcon;

#[derive(Component)]
pub struct SelectorSegment;

#[derive(Component)]
pub struct SelectorText(Hand);

#[derive(Resource, Default)]
pub struct SelectorResource {
    pub texture_index: Vec<u8>, // index to the weapon
    pub current_left: Option<u8>,
    pub current_right: Option<u8>,
}

// setup system
// for now hard coded to 4 weapons on the selection wheel
// wheel starts empty
pub fn setup(mut commands: Commands) {
    let texture_index = vec![0u8, 1, 2, 3, 4];
    commands.insert_resource({
        SelectorResource {
            texture_index,
            current_left: None,
            current_right: None,
        }
    });
}

#[derive(Copy, Clone, Debug)]
pub enum Hand {
    Left,
    Right,
}

fn selector_spawn(
    pos: Vec2,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    weapons_r: &WeaponsResource,
    selector_r: &SelectorResource,
    hand: Hand,
) {
    let nr_select = selector_r.texture_index.len() as f32;

    for (i, index) in selector_r.texture_index.iter().enumerate() {
        let shape = Mesh2dHandle(meshes.add(CircularSector::from_radians(
            SELECTOR_RADIUS,
            0.95 * TAU / nr_select,
        )));

        let weapon_held = match hand {
            Hand::Left => selector_r.current_left,
            Hand::Right => selector_r.current_right,
        };

        let color: Color = {
            if let Some(w) = weapon_held {
                if w == i as u8 {
                    SELECTOR_SELECT_COLOR
                } else {
                    SELECTOR_WHEEL_COLOR
                }
            } else {
                SELECTOR_WHEEL_COLOR
            }
        }
        .into();

        let angle = -(i as f32) * TAU / nr_select;

        // TODO, here we might want to use a component with children instead
        commands.spawn((
            Selector,
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_xyz(pos.x, pos.y, 100.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, angle)),
                ..default()
            },
        ));

        commands.spawn((
            SelectorIcon,
            SpriteBundle {
                texture: weapons_r.texture.clone(),
                transform: Transform::from_translation(
                    (
                        SELECTOR_RADIUS_ICON * angle.sin() + pos.x,
                        SELECTOR_RADIUS_ICON * angle.cos() + pos.y,
                        103.0,
                    )
                        .into(),
                )
                .with_scale((2.0, 2.0, 1.0).into()),

                ..default()
            },
            TextureAtlas {
                layout: weapons_r.texture_atlas_layout.clone(),
                index: *index as usize,
            },
        ));
    }

    let shape = Mesh2dHandle(meshes.add(CircularSector::from_radians(
        0.95 * SELECTOR_RADIUS,
        0.95 * TAU / nr_select,
    )));

    let color: Color = SELECTOR_SELECTOR_COLOR.into();
    commands.spawn((
        SelectorSegment,
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(
                color, //.with_alpha(0.1)
            ),
            transform: Transform::from_xyz(pos.x, pos.y, 101.0),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));

    let shape = Mesh2dHandle(meshes.add(Circle::new(40.0)));

    let color: Color = SELECTOR_WHEEL_COLOR.into();
    commands.spawn((
        Selector,
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(
                color, //.with_alpha(0.1)
            ),
            transform: Transform::from_xyz(pos.x, pos.y, 103.0),
            ..default()
        },
    ));

    commands.spawn((
        SelectorText(hand),
        Text2dBundle {
            text: Text::from_section(
                match hand {
                    Hand::Left => "Left",
                    Hand::Right => "Right",
                },
                TextStyle {
                    font_size: SELECTOR_FONT_SIZE,
                    color: SELECTOR_TEXT_COLOR.into(),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(pos.x, pos.y, 104.0),
            ..default()
        },
    ));
}

#[allow(clippy::too_many_arguments)]
pub fn update_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut selector_r: ResMut<SelectorResource>,
    weapons_r: Res<WeaponsResource>,
    player_r: Res<PlayerResource>,
    selector_q: Query<Entity, With<Selector>>,
    selector_icon_q: Query<Entity, With<SelectorIcon>>,
    selector_text_q: Query<(Entity, &SelectorText), With<SelectorText>>,
    mut selector_segment_q: Query<(Entity, &mut Visibility, &mut Transform), With<SelectorSegment>>,

    gamepads: Res<Gamepads>,
    // segment_r: ResMut<SelectorResource>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    let nr_select = selector_r.texture_index.len() as f32;

    for gamepad in gamepads.iter() {
        // spawn new selector only if no selector is shown
        if selector_q.is_empty() {
            let spawn = if button_inputs
                .just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2))
            {
                trace!("{:?} just pressed LeftTrigger2", gamepad);
                Some(Hand::Left)
            } else if button_inputs.just_pressed(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            )) {
                trace!("{:?} just pressed RightTrigger2", gamepad);
                Some(Hand::Right)
            } else {
                None
            };

            // spawn selector
            if let Some(hand) = spawn {
                debug!("spawn {:?}", hand);
                selector_spawn(
                    player_r.player_pos,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &weapons_r,
                    &selector_r,
                    hand,
                );
            }
        } else {
            let (text_entity, SelectorText(hand)) = selector_text_q.single();
            let despawn = match hand {
                Hand::Left => {
                    if button_inputs
                        .just_released(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2))
                    {
                        trace!("{:?} just released LeftTrigger2", gamepad);
                        Some(Hand::Left)
                    } else {
                        None
                    }
                }
                Hand::Right => {
                    if button_inputs.just_released(GamepadButton::new(
                        gamepad,
                        GamepadButtonType::RightTrigger2,
                    )) {
                        trace!("{:?} just released RightTrigger2", gamepad);
                        Some(Hand::Right)
                    } else {
                        None
                    }
                }
            };

            // right stick control
            let right_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
                .unwrap();
            let x = if right_stick_x.abs() > 0.01 {
                trace!("{:?} RightStickX value is {}", gamepad, right_stick_x);
                right_stick_x
            } else {
                0.0
            };
            let right_stick_y = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
                .unwrap();
            let y = if right_stick_y.abs() > 0.01 {
                trace!("{:?} RightStickY value is {}", gamepad, right_stick_y);
                right_stick_y
            } else {
                0.0
            };

            // None if no weapon is selected
            let selected = if x != 0.0 || y != 0.0 {
                let seg = segment(x, y, nr_select as u8);
                debug!("in segment {}", seg);
                Some(seg)
            } else {
                None
            };

            // Set selector
            let (segment_entity, mut segment_visibility, mut segment_transform) =
                selector_segment_q.single_mut();

            *segment_visibility = match selected {
                Some(seg) => {
                    let angle = (seg as f32) * TAU / nr_select;
                    let rotation = Quat::from_axis_angle(Vec3::Z, angle);
                    segment_transform.rotation = rotation;
                    Visibility::Visible
                }
                None => Visibility::Hidden,
            };

            // despawn selector (either Left or Right)
            if let Some(hand) = despawn {
                debug!("despawn {:?}", hand);
                // update selector only if some selection is made on release
                if let Some(seg) = selected {
                    match hand {
                        Hand::Left => selector_r.current_left = Some(seg),
                        Hand::Right => selector_r.current_right = Some(seg),
                    }
                }
                // despawn selector
                for entity in selector_q.iter() {
                    commands.entity(entity).despawn();
                }
                for entity in selector_icon_q.iter() {
                    commands.entity(entity).despawn();
                }

                commands.entity(segment_entity).despawn();
                commands.entity(text_entity).despawn();
            }
        }
    }
}

#[inline(always)]
fn segment(x: f32, y: f32, nr_segs: u8) -> u8 {
    let angle = 1.5 * PI + y.atan2(x);

    let segment = nr_segs as f32 * angle / TAU;
    let segment_round = segment.round();
    trace!(
        "nr_segs {}, angle {}, div {}, div round {}",
        nr_segs,
        angle,
        segment,
        segment_round
    );
    segment_round as u8 % nr_segs as u8
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_segment(s: &str, x: f32, y: f32, nr_segs: u8, expected: u8) {
        let seg = segment(x, y, nr_segs);
        println!("{}, segment {}\n", s, seg);
        assert_eq!(seg, expected);
    }
    #[test]
    fn test_segmentation() {
        test_segment("right", 1.0, 0.0, 4, 3);
        test_segment("right right up", 1.0, 0.5, 4, 3);
        test_segment("right up", 1.0, 1.0, 4, 3);

        test_segment("right up up", 0.5, 1.0, 4, 0);
        test_segment("up ", 0.0, 1.0, 4, 0);
        test_segment("left up up ", -0.5, 1.0, 4, 0);

        test_segment("left up", -1.0, 1.0, 4, 1);
        test_segment("left left up", -1.0, 0.5, 4, 1);
        test_segment("left ", -1.0, 0.0, 4, 1);
        test_segment("left left down", -1.0, -0.5, 4, 1);

        test_segment("left down", -1.0, -1.0, 4, 2);
        test_segment("left down down", -0.5, -1.0, 4, 2);
        test_segment("down", 0.0, -1.0, 4, 2);
        test_segment("down down right", 0.5, -1.0, 4, 2);

        test_segment("down right", 1.0, -1.0, 4, 3);
        test_segment("down right right", 1.0, -0.5, 4, 3);

        test_segment("right", 1.0, 0.0, 2, 0);
        test_segment("right right up", 1.0, 0.5, 2, 0);
        test_segment("up", 0.0, 1.0, 2, 0);
        test_segment("left left up", -1.0, 0.5, 2, 0);

        test_segment("left", -1.0, 0.0, 2, 1);
    }
}
