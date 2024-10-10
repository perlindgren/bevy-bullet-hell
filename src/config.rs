use bevy::prelude::*;

#[derive(Resource, Reflect, PartialEq)]
pub struct ConfigResource {
    pub gamepad: bool,
}

pub fn setup(mut commands: Commands) {
    // load from JSON via serde?
    let config = ConfigResource { gamepad: false };

    commands.insert_resource(config);
}
