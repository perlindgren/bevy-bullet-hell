use crate::utils::*;
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

#[derive(Resource, Default, Clone, ShaderType, Debug)]
pub struct LevelMeterSettings {
    pub time: f32,
    pub level: f32,
    pub impulse: f32,
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomUIMaterial {
    #[uniform(0)]
    pub settings: LevelMeterSettings,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
}

impl UiMaterial for CustomUIMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/excite.wgsl".into()
    }
}

#[derive(Resource)]
pub struct ExciteResource {
    pub shake: TimerEnvelope,
    pub level: f32,
}

pub fn update_system(
    time: Res<Time>,
    mut excite_r: ResMut<ExciteResource>,
    material_handles: Query<&Handle<CustomUIMaterial>>,
    mut materials: ResMut<Assets<CustomUIMaterial>>,
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

            trace!(
                "t {:?} material.settings.impulse {}, set level {}, material level {}",
                t,
                material.settings.impulse,
                excite_r.level,
                material.settings.level
            );
        }
    }
}
pub fn setup(mut commands: Commands) {
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

pub fn keyboard_input(mut excite_r: ResMut<ExciteResource>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        debug!("arrow_up");
        excite_r.level += 0.1;
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        debug!("arrow_down");
        excite_r.level -= 0.1;
    }
    if keys.just_pressed(KeyCode::Enter) {
        debug!("enter");
        excite_r.shake.timer.reset();
    }
}
