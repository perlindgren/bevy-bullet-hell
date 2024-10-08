use avian2d::{math::*, prelude::*};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_bullet_hell::{block, camera, common::*, gamepad, overlay, player, shooting, tile};
use bevy_ecs_tilemap::prelude::*;
// use bevy_egui::*;
// use bevy_egui::EguiContext;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::bevy_inspector;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::{egui, prelude::*, quick::WorldInspectorPlugin};
use std::any::TypeId;

#[derive(Component)]
struct EditorWindow;

fn setup(mut commands: Commands) {
    // we might want to setup a custom camera, for now just default
    commands.spawn(Camera2dBundle::default());
}

fn spawn_editor_window(mut commands: Commands) {
    commands.spawn((Window::default(), EditorWindow));
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<EditorWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}

// fn editor_ui(world: &mut World) {
//     let mut egui_ctx = world
//         .query_filtered::<&mut EguiContext, With<EditorWindow>>()
//         .single(world)
//         .clone();

//     egui::CentralPanel::default().show(egui_ctx.get_mut(), |ui| {
//         egui::ScrollArea::vertical()
//             .auto_shrink([false, false])
//             .show(ui, |ui| {
//                 bevy_inspector::ui_for_world(world, ui);
//             })
//     });
// }

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(RES_X, RES_Y),
                        resizable: false,
                        title: "Bevy-Bullet-Hell".to_string(),
                        desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(1.0),
            //WorldInspectorPlugin::new(),
            bevy_egui::EguiPlugin,
            // DefaultInspectorConfigPlugin,
        ))
        .add_plugins(TilemapPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Gravity(Vector::ZERO))
        .add_event::<shooting::ShotEvent>()
        .add_systems(
            Startup,
            (
                setup,
                overlay::setup,
                player::setup,
                block::setup,
                shooting::setup,
                tile::setup,
                spawn_editor_window,
            )
                .chain(),
        )
        .add_systems(
            Update,
            // (
            (
                gamepad::update_system,
                player::update_system,
                player::collider_system,
                block::update_system,
                shooting::new_shot_system,
                shooting::update_system,
                shooting::collider_system,
                overlay::fps_update_system,
                camera::update_system,
                inspector_ui,
            )
                .chain(), // .before(shooting::new_shot_system),
        )
        // )
        .run();
}
