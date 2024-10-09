use bevy::{
    color::palettes::css,
    prelude::*,
    render::view::visibility,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::f32::consts::{PI, TAU};

use bevy_bullet_hell::common::*;

struct Weapon {
    image: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct WeaponsResource {
    weapons: Vec<Weapon>,
}

pub fn weapon_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut weapons = vec![];

    for _ in 0..4 {
        weapons.push(Weapon {
            image: asset_server.load("sprites/cross.png"),
        })
    }

    commands.insert_resource(WeaponsResource { weapons });
}

#[derive(Component)]
pub struct Selector(u8);

#[derive(Component)]
pub struct SelectorIcon;

#[derive(Component)]
pub struct SelectorText(Hand);

#[derive(Resource, Default)]
pub struct SelectorResource {
    weapons: Vec<usize>, // index to the weapon
    current_left: Option<u8>,
    current_right: Option<u8>,
}

pub fn selector_setup(mut commands: Commands) {
    commands.insert_resource({
        let weapons = vec![0, 1, 2, 3];
        SelectorResource {
            weapons: vec![0, 1, 2, 3],
            current_left: Some(weapons[0]),
            current_right: None,
        }
    });
}

#[derive(Copy, Clone, Debug)]
enum Hand {
    Left,
    Right,
}

fn selector_spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    weapons_r: &WeaponsResource,
    selector_r: &SelectorResource,
    hand: Hand,
) {
    let nr_weapons = selector_r.weapons.len() as f32;

    for (i, weapons) in selector_r.weapons.iter().enumerate() {
        let shape =
            Mesh2dHandle(meshes.add(CircularSector::from_radians(100.0, 0.95 * TAU / nr_weapons)));
        println!("i {}", i);

        let weapon_held = match hand {
            Hand::Left => selector_r.current_left,
            Hand::Right => selector_r.current_right,
        };

        let color: Color = {
            if let Some(w) = weapon_held {
                if w == i as u8 {
                    css::DARK_GRAY
                } else {
                    css::DIM_GRAY
                }
            } else {
                css::DIM_GRAY
            }
        }
        .into();

        let angle = (i as f32) * TAU / nr_weapons;
        let weapon = &weapons_r.weapons[*weapons];

        // TODO, here we might want to use a component with children instead
        commands.spawn((
            Selector(i as u8),
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_xyz(0.0, 0.0, 10.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, angle)),
                ..default()
            },
        ));

        commands.spawn((
            SelectorIcon,
            SpriteBundle {
                texture: weapon.image.clone(),
                transform: Transform::from_translation(
                    (50.0 * angle.sin(), 50.0 * angle.cos(), 12.0).into(),
                ),
                ..default()
            },
        ));
    }
    commands.spawn((
        SelectorText(hand),
        TextBundle::from_section(
            match hand {
                Hand::Left => "Left Weapon/Ability",
                Hand::Right => "Right Weapon/Ability",
            },
            TextStyle {
                font_size: SELECTOR_FONT_SIZE,
                color: SELECTOR_TEXT_COLOR.into(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            align_self: AlignSelf::Center,
            ..default()
        }),
    ));
}

#[allow(clippy::too_many_arguments)]
pub fn selector_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut selector_r: ResMut<SelectorResource>,
    weapons_r: Res<WeaponsResource>,
    mut selector_q: Query<(Entity, &mut Visibility), With<Selector>>,
    selector_icon_q: Query<Entity, With<SelectorIcon>>,
    selector_text_q: Query<(Entity, &SelectorText), With<SelectorText>>,

    gamepads: Res<Gamepads>,
    // segment_r: ResMut<SelectorResource>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
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
                let seg = segment(x, y, selector_r.weapons.len());
                println!("in segment {}", seg);
                Some(seg)
            } else {
                None
            };

            for (i, (selector, mut visibility)) in selector_q.iter_mut().enumerate() {
                match selected {
                    Some(seg) => {
                        *visibility = if seg == i as u8 {
                            Visibility::Visible
                        } else {
                            Visibility::Hidden
                        }
                    }
                    None => *visibility = Visibility::Hidden,
                }
            }

            if let Some(hand) = despawn {
                // update selector only if some selection is made on release
                if let Some(seg) = selected {
                    match hand {
                        Hand::Left => selector_r.current_left = Some(seg),
                        Hand::Right => selector_r.current_right = Some(seg),
                    }
                }
                // despawn selector
                for (entity, _) in selector_q.iter() {
                    commands.entity(entity).despawn();
                }
                for entity in selector_icon_q.iter() {
                    commands.entity(entity).despawn();
                }

                commands.entity(text_entity).despawn();
            }
        }
    }

    // if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger)) {
    //     debug!("{:?} just pressed LeftTrigger", gamepad);
    // }

    //     if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger))
    //     {
    //         debug!("{:?} just pressed RightTrigger", gamepad);
    //     }

    //     if button_inputs.just_pressed(GamepadButton::new(
    //         gamepad,
    //         GamepadButtonType::RightTrigger2,
    //     )) {
    //         debug!("{:?} just pressed RightTrigger2 ", gamepad);
    //     }

    //     // left stick control
    //     let left_stick_x = axes
    //         .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
    //         .unwrap();
    //     target_resource.player_delta.x = if left_stick_x.abs() > 0.01 {
    //         trace!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
    //         left_stick_x
    //     } else {
    //         0.0
    //     };
    //     let left_stick_y = axes
    //         .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
    //         .unwrap();
    //     target_resource.player_delta.y = if left_stick_y.abs() > 0.01 {
    //         trace!("{:?} LefttStickY value is {}", gamepad, left_stick_y);
    //         left_stick_y
    //     } else {
    //         0.0
    //     };
    // }
}

#[inline(always)]
fn segment(x: f32, y: f32, nr_segs: usize) -> u8 {
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, weapon_setup, selector_setup).chain())
        .add_systems(Update, selector_system);
    app.run();
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_segment(s: &str, x: f32, y: f32, nr_segs: usize, expected: u8) {
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
