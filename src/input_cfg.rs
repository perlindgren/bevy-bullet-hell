use crate::common::*;
use bevy::prelude::*;
use input_linux_tools::device::*;
use serde::{Deserialize, Serialize};
use std::{env::current_dir, fs, path::PathBuf};

// Input configuration for one player
// None indicates that the input is not yet bound
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlayerInput {
    pub pos_input: Device,
    pub aim_input: Device,
}

// Input configuration for all players
// Vector index connesponds to Player(index)
#[derive(Resource, Debug, Serialize, Deserialize, Default)]
pub struct PlayersInput {
    pub player_input: Vec<PlayerInput>,
    pub config_path: PathBuf,
}

pub fn setup(mut commands: Commands) {
    // default location for configuation
    let mut path = current_dir().unwrap();
    path.push("input_cfg");
    path.set_extension("ron");

    let mut players_input = PlayersInput::default();
    players_input.config_path = path;

    // setup each player
    for _ in 0..NR_PLAYERS {
        players_input.player_input.push(PlayerInput::default());
    }

    // load default config
    // if path.exists() {
    //     if let Ok(bytes) = fs::read(path) {
    //         let ron: Result<ConfigInput, _> = ron::de::from_bytes(&bytes);
    //         match ron {
    //             Ok(ron_config) => config_input = ron_config,
    //             _ => {}
    //         }
    //     }
    // }
    debug!("config_input :{:?}", players_input);
    commands.insert_resource(players_input);
}
