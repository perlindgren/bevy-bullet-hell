use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use std::net::UdpSocket;

#[derive(Reflect, Resource, Default, InspectorOptions)]
pub struct NetUIResource {
    pub network_path: String,
    pub connected: bool,
}

pub struct NetResource {
    pub socket: Option<UdpSocket>,
}

fn setup(mut command: Commands) {
    let net = NetUIResource {
        network_path: "".to_string(),
        connected: false,
    };
    command.insert_resource(net);
}
