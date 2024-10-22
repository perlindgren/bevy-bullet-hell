use bevy::prelude::*;
use input_linux_tools::device::*;
use serde::{Deserialize, Serialize};
use std::{env::current_dir, fs, path::PathBuf};

#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct PlayerInput {
    pub pos_input: Option<Device>,
    pub aim_input: Option<Device>,
    pub path: PathBuf,
}

#[derive(Resource, Debug)]
pub struct InputDevices {
    pub devices: Devices,
}

pub fn setup(mut commands: Commands) {
    let devices = Devices::new().unwrap();
    commands.insert_resource(InputDevices { devices });

    let mut path = current_dir().unwrap();
    path.push("input_cfg");
    path.set_extension("ron");

    let mut player = PlayerInput {
        pos_input: None,
        aim_input: None,
        path: path.clone(),
    };

    if path.exists() {
        if let Ok(bytes) = fs::read(path) {
            let ron: Result<PlayerInput, _> = ron::de::from_bytes(&bytes);
            match ron {
                Ok(ron_player) => player = ron_player,
                _ => {}
            }
        }
    }
    println!("player :{:?}", player);
    commands.insert_resource(player);
}
