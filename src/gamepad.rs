use bevy::prelude::*;

use crate::{player::DeltaResource, shooting::ShotEvent};

pub fn update_system(
    gamepads: Res<Gamepads>,
    mut target_resource: ResMut<DeltaResource>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    // button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut fire_lazer_ew: EventWriter<ShotEvent>,
) {
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger)) {
            debug!("{:?} just pressed LeftTrigger", gamepad);
            fire_lazer_ew.send(ShotEvent);
        }

        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2))
        {
            debug!("{:?} just pressed LeftTrigger2", gamepad);
            fire_lazer_ew.send(ShotEvent);
        }

        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger))
        {
            debug!("{:?} just pressed RightTrigger", gamepad);
            fire_lazer_ew.send(ShotEvent);
        }

        if button_inputs.just_pressed(GamepadButton::new(
            gamepad,
            GamepadButtonType::RightTrigger2,
        )) {
            debug!("{:?} just pressed RightTrigger2 ", gamepad);
            fire_lazer_ew.send(ShotEvent);
        }

        // let right_trigger = button_axes
        //     .get(GamepadButton::new(
        //         gamepad,
        //         GamepadButtonType::RightTrigger2,
        //     ))
        //     .unwrap();
        // if right_trigger.abs() > 0.01 {
        //     trace!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        // }

        // let left_stick_x = axes
        //     .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
        //     .unwrap();
        // if left_stick_x.abs() > 0.01 {
        //     trace!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        // }

        // right stick control
        let right_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
            .unwrap();
        target_resource.aim_delta.x = if right_stick_x.abs() > 0.01 {
            trace!("{:?} RightStickX value is {}", gamepad, right_stick_x);
            right_stick_x
        } else {
            0.0
        };
        let right_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
            .unwrap();
        target_resource.aim_delta.y = if right_stick_y.abs() > 0.01 {
            trace!("{:?} RightStickY value is {}", gamepad, right_stick_y);
            right_stick_y
        } else {
            0.0
        };

        // left stick control
        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        target_resource.player_delta.x = if left_stick_x.abs() > 0.01 {
            trace!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
            left_stick_x
        } else {
            0.0
        };
        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        target_resource.player_delta.y = if left_stick_y.abs() > 0.01 {
            trace!("{:?} LefttStickY value is {}", gamepad, left_stick_y);
            left_stick_y
        } else {
            0.0
        };
    }
}
