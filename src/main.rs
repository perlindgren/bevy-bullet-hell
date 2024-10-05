use bevy::prelude::*;

fn list_gamepads(gamepads: Res<Gamepads>) {
    println!("Currently connected gamepads:");
    for gamepad in gamepads.iter() {
        println!(
            "ID: {:?}; Name: {}",
            gamepad,
            gamepads.name(gamepad).unwrap_or("unknown")
        );
    }
}

fn main() {
    App::new().run();
}
