use bevy::{
    color::palettes::css,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::f32::consts::TAU;

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
struct Selector;

#[derive(Resource, Default)]
struct SelectorResource {
    weapons: Vec<usize>, // index to the weapon
}

pub fn selector_setup(mut commands: Commands) {
    commands.insert_resource(SelectorResource {
        weapons: vec![0, 1, 3],
    });
}

fn selector_spawn(
    mut commands: &mut Commands,
    mut meshes: &mut Assets<Mesh>,
    mut materials: &mut Assets<ColorMaterial>,
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
            Selector,
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_xyz(0.0, 0.0, 10.0)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, angle)),
                ..default()
            },
        ));

        commands.spawn((
            Selector,
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

    //     // right stick control
    //     let right_stick_x = axes
    //         .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
    //         .unwrap();
    //     target_resource.aim_delta.x = if right_stick_x.abs() > 0.01 {
    //         trace!("{:?} RightStickX value is {}", gamepad, right_stick_x);
    //         right_stick_x
    //     } else {
    //         0.0
    //     };
    //     let right_stick_y = axes
    //         .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
    //         .unwrap();
    //     target_resource.aim_delta.y = if right_stick_y.abs() > 0.01 {
    //         trace!("{:?} RightStickY value is {}", gamepad, right_stick_y);
    //         right_stick_y
    //     } else {
    //         0.0
    //     };

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
    app.add_plugins((DefaultPlugins))
        .add_systems(Startup, (setup, weapon_setup, selector_setup).chain())
        .add_systems(Update, selector_system);
    app.run();
}
