use crate::input_cfg::*;
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui, EguiContext};
use input_linux_tools::{device::*, keyboard::*, mouse::*};
use rfd::FileDialog;
use std::{fs, path::PathBuf};

#[derive(Component)]
pub struct EditorCfgWindow;

// Used by the gui to select input device
#[derive(Resource, Debug)]
pub struct InputDevices {
    pub devices: Devices,
}

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

    // populate the input devices
    let devices = Devices::new().unwrap();
    commands.insert_resource(InputDevices { devices });
}

fn split_path(config_path: &PathBuf) -> (&std::path::Path, &str) {
    (
        config_path.parent().unwrap(),
        config_path.file_name().unwrap().to_str().unwrap(),
    )
}

pub fn update_system(
    mut config_input_r: ResMut<PlayersInput>,
    //mut inputs_r: ResMut<Inputs>,
    input_devices_r: Res<InputDevices>,
    mut egui_q: Query<&mut EguiContext, With<EditorCfgWindow>>,
) {
    let mut egui_context = egui_q.single_mut();

    egui::Window::new("Player Input Configuration").show(egui_context.get_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Save Config").clicked() {
                let (cfg_path, file_name) = split_path(&config_input_r.config_path);
                if let Some(selected_path) = FileDialog::new()
                    .add_filter("ron", &["ron"])
                    .set_title("Save input configuration")
                    .set_directory(cfg_path)
                    .set_file_name(file_name)
                    .save_file()
                {
                    // update path in case changed
                    config_input_r.config_path = selected_path.clone();

                    // serialize and write to file
                    let serialized_str = ron::ser::to_string_pretty(
                        &*config_input_r,
                        ron::ser::PrettyConfig::default(),
                    )
                    .unwrap();
                    let _ = fs::write(selected_path, serialized_str);
                }
            };

            if ui.button("Load Config").clicked() {
                let (cfg_path, file_name) = split_path(&config_input_r.config_path);
                debug!(
                    "load config from folder: {:?}, file: {:?}",
                    cfg_path, file_name
                );
                let os_str = config_input_r.config_path.as_os_str().to_str().unwrap();
                debug!("os_str {}", os_str);
                if let Some(selected_path) = FileDialog::new()
                    .add_filter("ron", &["ron"])
                    .set_title("Load input configuration")
                    .set_directory(cfg_path)
                    .set_file_name(file_name)
                    .pick_file()
                {
                    if let Ok(bytes) = fs::read(&selected_path) {
                        let ron: Result<PlayersInput, _> = ron::de::from_bytes(&bytes);
                        match ron {
                            Ok(players_input) => {
                                debug!("configuration loaded: {:?}", selected_path);
                                debug!("{:?}", players_input);
                                *config_input_r = players_input;
                            }
                            _ => {}
                        }
                    }
                }
            };
        });
        for (index, player_input) in config_input_r.player_input.iter_mut().enumerate() {
            let PlayerInput {
                pos_input,
                // aim_input,
                ..
            } = player_input;

            ui.label(format!("Player {}", index));

            select_device(ui, &input_devices_r.devices, pos_input);
        }
    });
}

fn select_device(ui: &mut egui::Ui, devices: &Devices, input: &mut Device) {
    egui::ComboBox::from_label(format!("Input"))
        .selected_text(format!("{:?} {:?}", input.device_type, input.path))
        .show_ui(ui, |ui| {
            ui.label("Mouse Devices");
            for i in 0..devices.mice.len() {
                let value = ui.selectable_value(
                    &mut input.path,
                    devices.mice[i].clone(),
                    devices.mice[i].to_str().unwrap(),
                );
                if value.clicked() {
                    input.path = devices.mice[i].clone();
                    input.device_type = DeviceType::Mouse;
                }
            }

            ui.separator();
            ui.label("Keyboard Devices");
            for i in 0..devices.keyboards.len() {
                let value = ui.selectable_value(
                    &mut input.path,
                    devices.keyboards[i].clone(),
                    devices.keyboards[i].to_str().unwrap(),
                );
                if value.clicked() {
                    input.path = devices.keyboards[i].clone();
                    input.device_type = DeviceType::Keyboard;
                }
            }

            ui.separator();
            ui.label("GamePad Devices");
            for i in 0..devices.gamepads.len() {
                let value = ui.selectable_value(
                    &mut input.path,
                    devices.gamepads[i].clone(),
                    devices.gamepads[i].to_str().unwrap(),
                );
                if value.clicked() {
                    input.path = devices.gamepads[i].clone();
                    input.device_type = DeviceType::GamePad;
                }
            }
        });
}
