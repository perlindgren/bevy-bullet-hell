use crate::player::PlayerResource;
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui, EguiContext};

#[derive(Component)]
pub struct EditorWindow;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Window {
            position: WindowPosition::At((400, 400).into()),
            resolution: WindowResolution::new(400.0, 800.0),
            title: "Inspector".to_string(),
            ..default()
        },
        EditorWindow,
    ));
}

pub fn update_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<EditorWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::CollapsingHeader::new("PlayerResource").show(ui, |ui| {
            bevy_inspector_egui::bevy_inspector::ui_for_resource::<PlayerResource>(world, ui);
        });
    });
}
