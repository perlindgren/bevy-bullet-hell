use bevy::{prelude::*, window::PrimaryWindow};
use heapless::mpmc::*;
use multiinput::{DeviceType, RawEvent, RawInputManager};

use crate::{
    common::CustomCamera,
    player::{DeltaResource, PlayerResource},
};

const QUEUE_SIZE: usize = 1024; // this should be enough for 1 sec / amount of players worth of data (mouse is polled at 1kHz typically)

// ID (Delta_x, Delta_y)
#[derive(Clone, Copy, Debug)]
pub struct MousePacket(u8, (i32, i32));

pub type MouseQueue = MpMcQueue<MousePacket, QUEUE_SIZE>;

//pub type MouseReceiver = Consumer<'static, MousePacket, QUEUE_SIZE>;

#[derive(Resource)]
pub struct DevInput<'a> {
    mouse_queue: &'a MouseQueue,
    pos: Vec2,
}

pub fn setup(mut commands: Commands) {
    // technically i think there is only one consumer so not wrapping this in a queue would probably be fine,
    // but i don't know how to prove this to bevy.
    let mouse_mpmc: &'static MouseQueue = {
        static mut MPMC: MouseQueue = MpMcQueue::new();
        #[allow(static_mut_refs)]
        unsafe {
            &mut MPMC
        }
    };

    std::thread::spawn(|| {
        let mut manager = RawInputManager::new().unwrap();

        manager.register_devices(DeviceType::Mice);

        loop {
            if let Some(event) = manager.get_event() {
                match event {
                    RawEvent::MouseMoveEvent(id, x, y) => {
                        let mouse_packet = MousePacket(id as u8, (x, y));
                        mouse_mpmc.enqueue(mouse_packet).ok();
                    }
                    _ => {}
                }
            }
        }
    });

    commands.insert_resource(DevInput {
        mouse_queue: mouse_mpmc,
        pos: (0.0, 0.0).into(),
    });
}

pub fn update_system(
    mut dev_input_r: ResMut<DevInput<'static>>,
    mut delta_r: ResMut<DeltaResource>,
    player_r: Res<PlayerResource>,
    // q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), Without<CustomCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    loop {
        match dev_input_r.mouse_queue.dequeue() {
            Some(packet) => {
                trace!("Input: {:?}", packet);

                dev_input_r.pos.x += packet.1 .0 as f32 * 0.5;
                // windows apparently uses fly simulator deltas so y is inverted xD
                dev_input_r.pos.y -= packet.1 .1 as f32 * 0.5;
            }
            None => break, // all input now dequeued
        }
    }
    dev_input_r.pos = dev_input_r.pos.clamp_length(0.0, 500.0);
    //dev_input_r.pos = dev_input_r.pos / 500.0;
    println!("pos: {:?}", dev_input_r.pos);
    //let world_position = camera
    //    .viewport_to_world(camera_transform, dev_input_r.pos)
    //    .map(|ray| ray.origin.truncate())
    //    .unwrap();

    // let delta = world_position - player_r.player_pos;
    //let delta = delta / 500.0;
    // let delta = delta.clamp_length(0.0, 1.0);
    // delta_r.aim_delta = delta;

    let delta: Vec2 = dev_input_r.pos / 500.0;
    let delta = delta.clamp_length(0.0, 1.0);
    delta_r.aim_delta = delta;
}
