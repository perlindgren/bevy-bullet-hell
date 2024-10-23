use bevy::prelude::*;
use input_linux_tools::{device::*, input_linux::EvdevHandle};
use serde::{Deserialize, Serialize};
use std::{
    any::Any,
    env::current_dir,
    fs::{self, File},
    path::PathBuf,
};

#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct ConfigInput {
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

    let mut config_input = ConfigInput {
        pos_input: None,
        aim_input: None,
        path: path.clone(),
    };

    if path.exists() {
        if let Ok(bytes) = fs::read(path) {
            let ron: Result<ConfigInput, _> = ron::de::from_bytes(&bytes);
            match ron {
                Ok(ron_config) => config_input = ron_config,
                _ => {}
            }
        }
    }
    println!("config_input :{:?}", config_input);
    commands.insert_resource(config_input);
    commands.insert_resource(Inputs::default());
}

#[derive(Resource, Default, Debug)]
pub struct Inputs {
    pub pos_input: Option<DeviceType>,
    pub aim_input: Option<DeviceType>,
}
