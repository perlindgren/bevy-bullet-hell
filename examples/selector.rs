use bevy::{
    color::palettes::css,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::f32::consts::{PI, TAU};

const X_EXTENT: f32 = 900.;

struct Weapon {
    image: Handle<Image>,
}

#[derive(Resource, Default)]
struct Weapons {
    weapons: Vec<Weapon>,
}

pub fn weapon_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut weapons = vec![];

    for _ in 0..3 {
        weapons.push(Weapon {
            image: asset_server.load("sprites/cross.png"),
        })
    }

    commands.insert_resource(Weapons { weapons });
}

#[derive(Component)]
struct Selector(u8);

#[derive(Component)]
struct SelectorIcon;

#[derive(Resource, Default)]
struct SelectorResource {
    weapons: Vec<usize>, // index to the weapon
}

pub fn selector_setup(mut commands: Commands) {
    commands.insert_resource(SelectorResource {
        weapons: vec![0, 1, 2, 3],
    });
}

fn selector_spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    weapons_r: &Weapons,
) {
    let nr_weapons = weapons_r.weapons.len() as f32;

    for (i, Weapon { image }) in weapons_r.weapons.iter().enumerate() {
        let shape =
            Mesh2dHandle(meshes.add(CircularSector::from_radians(100.0, 0.95 * TAU / nr_weapons)));
        println!("i {}", i);

        let color: Color = css::DIM_GRAY.into();
        let angle = (i as f32) * TAU / nr_weapons;

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
                texture: image.clone(),
                transform: Transform::from_translation(
                    (50.0 * angle.sin(), 50.0 * angle.cos(), 12.0).into(),
                ),
                ..default()
            },
        ));
    }
}

pub fn selector_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    weapons_r: Res<Weapons>,
    mut selector_q: Query<Entity, With<Selector>>,
    mut selector_icon_q: Query<Entity, With<SelectorIcon>>,

    gamepads: Res<Gamepads>,
    mut segment_r: ResMut<SelectorResource>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in gamepads.iter() {
        // spawn selector
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2))
        {
            debug!("{:?} just pressed LeftTrigger2", gamepad);
            selector_spawn(&mut commands, &mut meshes, &mut materials, &weapons_r);
        }

        // despawn selector
        if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2))
        {
            debug!("{:?} just released LeftTrigger2", gamepad);
            for entity in selector_q.iter() {
                commands.entity(entity).despawn();
            }
            for entity in selector_icon_q.iter() {
                commands.entity(entity).despawn();
            }
        }

        if segment_r.weapons.len() > 0 {
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

            let nr_segs = segment_r.weapons.len();
            let angle = y.atan2(x) + PI; // + 2.0 * PI; //  * (1.0 + 1.0 / nr_segs as f32));

            // 0 degree pointing at (1, 0)
            debug!("x {}, y {}, angle {:?}", x, y, angle);

            fn check_in_segment(angle: f32, nr_segs: usize) -> usize {
                // check which segment the angle belongs to
                for i in 0..nr_segs {
                    let seg_end = TAU * (0.5 + i as f32) / nr_segs as f32;
                    debug!("{i} {}", seg_end);

                    if angle < seg_end {
                        return i;
                    }
                }
                0
            }
            println!("in segment {}", check_in_segment(angle, nr_segs));
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
mod test {}
