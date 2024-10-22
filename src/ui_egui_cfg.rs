use crate::input_cfg::*;
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui, EguiContext};
use input_linux_tools::device::*;

#[derive(Component)]
pub struct EditorCfgWindow;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Window {
            position: WindowPosition::At((400, 400).into()),
            resolution: WindowResolution::new(400.0, 800.0),
            title: "Input Configuration".to_string(),
            ..default()
        },
        EditorCfgWindow,
    ));
}

fn select_device(ui: &mut egui::Ui, devices: &Devices, input: &mut Option<Device>) {
    egui::ComboBox::from_label(format!("Input"))
        .selected_text(format!("{:?}", input))
        .show_ui(ui, |ui| {
            if ui.button("Select None").clicked() {
                *input = None;
            }
            ui.separator();
            ui.label("Mouse Devices");
            for i in 0..devices.mice.len() {
                let value = ui.selectable_value(
                    input,
                    Some(Device::Mouse(devices.mice[i].clone())),
                    devices.mice[i].to_str().unwrap(),
                );
                if value.clicked() {
                    *input = Some(Device::Mouse(devices.mice[i].clone()));
                }
            }
            ui.separator();
            ui.label("Keyboard Devices");
            for i in 0..devices.keyboards.len() {
                let value = ui.selectable_value(
                    input,
                    Some(Device::Mouse(devices.keyboards[i].clone())),
                    devices.keyboards[i].to_str().unwrap(),
                );
                if value.clicked() {
                    *input = Some(Device::Mouse(devices.keyboards[i].clone()));
                }
            }
            ui.separator();
            ui.label("Gamepad Devices");
            for i in 0..devices.gamepads.len() {
                let value = ui.selectable_value(
                    input,
                    Some(Device::Mouse(devices.gamepads[i].clone())),
                    devices.gamepads[i].to_str().unwrap(),
                );
                if value.clicked() {
                    *input = Some(Device::Mouse(devices.gamepads[i].clone()));
                }
            }
        });
}
pub fn update_system(
    mut player_input_r: ResMut<PlayerInput>,
    mut egui_q: Query<&mut EguiContext, With<EditorCfgWindow>>,
) {
    let mut egui_context = egui_q.single_mut();

    egui::Window::new("Player Input Configuration").show(egui_context.get_mut(), |ui| {
        ui.label("Player");

        let PlayerInput {
            devices,
            pos_input,
            aim_input,
        } = &mut *player_input_r; // reborrow

        egui::CollapsingHeader::new("Pos").show(ui, |ui| {
            select_device(ui, devices, pos_input);
        });
        egui::CollapsingHeader::new("Aim").show(ui, |ui| {
            select_device(ui, devices, aim_input);
        });
    });
}
