pub use bevy::prelude::*;

pub struct Weapon {
    pub image: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct WeaponsResource {
    pub weapons: Vec<Weapon>,
}

// setup system, for now hard coded to 4 weapons
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut weapons = vec![];

    for _ in 0..4 {
        weapons.push(Weapon {
            image: asset_server.load("sprites/cross.png"),
        })
    }

    commands.insert_resource(WeaponsResource { weapons });
}
