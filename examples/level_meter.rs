//! A shader and a material that uses it.

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use bevy_bullet_hell::{hud::fps, utils::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (setup, fps::setup))
        .add_systems(
            Update,
            (keyboard_input, animate_materials, fps::update_system),
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
    // let res = ExciteResource
    commands.insert_resource(ExciteResource {
        shake: TimerEnvelope::new(Envelope {
            start_value: 1.0,
            points: vec![
                EnvPoint {
                    delta_time: 0.5,
                    value: 4.0,
                },
                EnvPoint {
                    delta_time: 3.0,
                    value: 1.0,
                },
            ],
        }),
        level: 0.0,
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

#[derive(Resource)]
pub struct ExciteResource {
    pub shake: TimerEnvelope,
    pub level: f32,
}

fn animate_materials(
    time: Res<Time>,
    mut excite_r: ResMut<ExciteResource>,
    material_handles: Query<&Handle<CustomMaterial>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for material_handle in material_handles.iter() {
        let t = time.elapsed_seconds();

        if let Some(material) = materials.get_mut(material_handle) {
            material.settings.time = t;
            material.settings.level += 0.1
                * time.delta_seconds()
                * if material.settings.level < excite_r.level {
                    1.0
                } else {
                    -1.0
                };
            material.settings.impulse = excite_r.shake.get(time.delta());

            // println!(
            //     "t {:?} material.settings.impulse {}, set level {}, material level {}",
            //     t, material.settings.impulse, excite_r.level, material.settings.level
            // );
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

fn keyboard_input(mut excite_r: ResMut<ExciteResource>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        excite_r.level += 0.1;
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        excite_r.level -= 0.1;
    }
    if keys.just_pressed(KeyCode::Enter) {
        excite_r.shake.timer.reset();
    }
}
