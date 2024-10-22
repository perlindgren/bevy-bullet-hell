use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use input_linux_tools::device::*;
use std::path::PathBuf;

#[derive(Resource)]

pub struct PlayerInput {
    pub devices: Devices,
    pub pos_input: Option<Device>,
    pub aim_input: Option<Device>,
}

pub fn setup(mut commands: Commands) {
    let devices = Devices::new().unwrap();
    commands.insert_resource(PlayerInput {
        devices,
        pos_input: None,
        aim_input: None,
    });
}
