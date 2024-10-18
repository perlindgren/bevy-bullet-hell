use multiinput::{DeviceType, KeyId, RawEvent, RawInputManager, State};
use std::collections::HashMap;

fn main() {
    let mut manager = RawInputManager::new().unwrap();

    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Mice);

    let mut player_keyboard_ids: HashMap<_, _> = HashMap::new();
    //let player_mouse_ids: Vec<_> = vec![];
    let mut i = 0;
    'outer: loop {
        println!("Player {}, press space, anyone press escape when ready", i);
        loop {
            if let Some(event) = manager.get_event() {
                match event {
                    RawEvent::KeyboardEvent(device_id, KeyId::Space, State::Pressed) => {
                        if !player_keyboard_ids.get(&device_id).is_some() {
                            player_keyboard_ids.insert(device_id, i);
                            break;
                        }
                    }
                    RawEvent::KeyboardEvent(_, KeyId::Escape, State::Pressed) => break 'outer,
                    _ => {}
                }
            }
        }
        i += 1;
    }

    'outer: loop {
        if let Some(event) = manager.get_event() {
            match event {
                RawEvent::KeyboardEvent(_, KeyId::Escape, State::Pressed) => break 'outer,
                RawEvent::KeyboardEvent(device_id, key_id, state) => {
                    match player_keyboard_ids.get(&device_id) {
                        Some(player_id) => {
                            println!("Player {} {:?} {:?}", player_id, state, key_id);
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }
    }
}
