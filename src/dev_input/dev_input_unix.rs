use crate::player::DeltaResource;
use bevy::prelude::*;
use nonblock::NonBlockingReader;
use std::fs::File;

#[derive(Resource)]
pub struct DevInput {
    nb_dev_input: NonBlockingReader<File>,
    pos: Vec2,
}

pub fn setup(mut commands: Commands) {
    let file = File::open("/dev/input/by-id/usb-Mouse_USB_Laser_Mouse-mouse").unwrap();
    let nb_dev_input = NonBlockingReader::from_fd(file).unwrap();
    commands.insert_resource(DevInput {
        nb_dev_input,
        pos: Vec2::ZERO,
    });
    debug!("opened: /dev/input/mouse0");
}

pub fn update_system(mut dev_input_r: ResMut<DevInput>, mut delta_r: ResMut<DeltaResource>) {
    let mut data = vec![];

    if let Ok(n) = dev_input_r.nb_dev_input.read_available(&mut data) {
        trace!("n {}", n);
        // take care of the (rare) case where we received multiple chunks
        data.chunks(3).for_each(|d| {
            trace!("d {:?}", d);
            dev_input_r.pos.x += d[1] as i8 as f32 * 0.5;
            dev_input_r.pos.y += d[2] as i8 as f32 * 0.5;
        });
    }
    let delta = dev_input_r.pos / 500.0;
    let delta = delta.clamp_length(0.0, 1.0);
    delta_r.aim_delta = delta;
}

// sudo usermod -a -G input pln
