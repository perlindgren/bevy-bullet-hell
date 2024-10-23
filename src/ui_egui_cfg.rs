use crate::input_cfg::*;
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui, EguiContext};
use input_linux_tools::{device::*, keyboard::*, mouse::*};
use rfd::FileDialog;
use std::fs;

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
                    Some(Device::Keyboard(devices.keyboards[i].clone())),
                    devices.keyboards[i].to_str().unwrap(),
                );
                if value.clicked() {
                    *input = Some(Device::Keyboard(devices.keyboards[i].clone()));
                }
            }
            ui.separator();
            ui.label("Gamepad Devices");
            for i in 0..devices.gamepads.len() {
                let value = ui.selectable_value(
                    input,
                    Some(Device::GamePad(devices.gamepads[i].clone())),
                    devices.gamepads[i].to_str().unwrap(),
                );
                if value.clicked() {
                    *input = Some(Device::GamePad(devices.gamepads[i].clone()));
                }
            }
        });
}

pub fn update_system(
    mut config_input_r: ResMut<ConfigInput>,
    mut inputs_r: ResMut<Inputs>,
    input_devices_r: Res<InputDevices>,
    mut egui_q: Query<&mut EguiContext, With<EditorCfgWindow>>,
) {
    let mut egui_context = egui_q.single_mut();

    egui::Window::new("Player Input Configuration").show(egui_context.get_mut(), |ui| {
        if ui
            .radio(
                inputs_r.pos_input.is_some(),
                format!("Pos :{:?}", inputs_r.pos_input),
            )
            .clicked()
        {
            inputs_r.pos_input = DeviceType::connect(&config_input_r.pos_input);
        }
        ui.horizontal(|ui| {
            if ui.button("Save Config").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("ron", &["ron"])
                    .set_directory(config_input_r.path.parent().unwrap())
                    .set_file_name(config_input_r.path.file_name().unwrap().to_str().unwrap())
                    .save_file()
                {
                    // update path in case changed
                    config_input_r.path = path.clone();
                    let str = ron::ser::to_string_pretty(
                        &*config_input_r,
                        ron::ser::PrettyConfig::default(),
                    )
                    .unwrap();
                    let _ = fs::write(path, str);
                }
            };

            if ui.button("Load Config").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("ron", &["ron"])
                    .set_directory(config_input_r.path.parent().unwrap())
                    .set_file_name(config_input_r.path.file_name().unwrap().to_str().unwrap())
                    .pick_file()
                {
                    if let Ok(bytes) = fs::read(path) {
                        let ron: Result<ConfigInput, _> = ron::de::from_bytes(&bytes);
                        match ron {
                            Ok(player_input) => {
                                *config_input_r = player_input;
                            }
                            _ => {}
                        }
                    }
                }
            };
        });

        let ConfigInput {
            pos_input,
            aim_input,
            path: _,
        } = &mut *config_input_r; // reborrow
        ui.label("Player");

        let InputDevices { devices } = &*input_devices_r;

        egui::CollapsingHeader::new("Pos").show(ui, |ui| {
            select_device(ui, devices, pos_input);
        });
        egui::CollapsingHeader::new("Aim").show(ui, |ui| {
            select_device(ui, devices, aim_input);
        });
    });
}
