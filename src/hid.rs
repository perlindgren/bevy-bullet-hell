// hid

use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{common::*, selector::SelectorResource, weapon::WeaponsResource};

pub fn update_system(weapons_r: Res<WeaponsResource>, selector_r: Res<SelectorResource>) {}

#[derive(Component)]
pub struct HID;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Capsule2d {
        radius: HID_HEIGHT,
        half_length: HID_WIDTH,
    }));

    let color: Color = HID_BACKGROUND_COLOR.into();
    commands.spawn((
        HID,
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: GlobalTransform::from_xyz(HID_POS.x, HID_POS.y, 90.0),
            ..default()
        },
    ));
}
