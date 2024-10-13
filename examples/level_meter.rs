//! A shader and a material that uses it.

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use bevy_bullet_hell::overlay;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (setup, overlay::setup))
        .add_systems(
            Update,
            (
                keyboard_input,
                animate_materials,
                overlay::fps_update_system,
            ),
        )
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(256.)),
        material: materials.add(CustomMaterial {
            settings: LevelMeterSettings::default(),
            color_texture: Some(asset_server.load("sprites/level_meter.png")),
        }),
        ..default()
    });
}

#[derive(Resource, Default, Clone, ShaderType, Debug)]
pub struct LevelMeterSettings {
    time: f32,
    level: f32,
    impulse: f32,
}
// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    settings: LevelMeterSettings,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

fn animate_materials(
    time: Res<Time>,
    material_handles: Query<&Handle<CustomMaterial>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for material_handle in material_handles.iter() {
        let t = time.elapsed_seconds();

        if let Some(material) = materials.get_mut(material_handle) {
            material.settings.time = t;
            material.settings.level = (material.settings.level + (time.delta_seconds())).min(0.5);
            // | delta |
            // |    delta     |
            material.settings.impulse =
                (material.settings.impulse - 1.0 * time.delta_seconds()).max(1.0);
            // we should use time instead
            println!(
                "t {:?} material.settings.impulse {}",
                t, material.settings.impulse
            );
        }
    }
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/level_meter.wgsl".into()
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    material_handles: Query<&Handle<CustomMaterial>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for material_handle in material_handles.iter() {
        println!("impulse");
        if let Some(material) = materials.get_mut(material_handle) {
            if keys.just_pressed(KeyCode::Space) {
                material.settings.impulse = 5.0;
            }
            if keys.just_pressed(KeyCode::Enter) {
                material.settings.level = 0.0;
            }
        }
    }
}
