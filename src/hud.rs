use crate::{
    common::*,
    selector::{Hand, SelectorResource},
    weapon::WeaponsResource,
};
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

#[derive(Resource, Default, Clone, ShaderType, Debug)]
pub struct LevelMeterSettings {
    time: f32,
    level: f32,
    impulse: f32,
}
// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomUIMaterial {
    #[uniform(0)]
    settings: LevelMeterSettings,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

// impl Material2d for CustomUIMaterial {
impl UiMaterial for CustomUIMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/excite.wgsl".into()
    }
}

pub fn update_excite_system(
    time: Res<Time>,
    material_handles: Query<&Handle<CustomUIMaterial>>,
    mut materials: ResMut<Assets<CustomUIMaterial>>,
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
            // println!(
            //     "t {:?} material.settings.impulse {}",
            //     t, material.settings.impulse
            // );
        }
    }
}

#[derive(Component)]
pub struct InHand(Hand);

pub fn setup(
    mut commands: Commands,
    weapons: Res<WeaponsResource>,
    mut ui_materials: ResMut<Assets<CustomUIMaterial>>,
    asset_server: Res<AssetServer>,
) {
    const LARGE_ICON: Val = Val::Px(64.0);
    let icon_style = Style {
        height: LARGE_ICON,
        width: LARGE_ICON,
        ..default()
    };
    commands
        .spawn(NodeBundle {
            style: Style {
                // width: Val::Px(400.0),
                height: HID_HEIGHT,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                left: Val::Px(15.0),
                bottom: Val::Px(15.0),
                position_type: PositionType::Absolute,
                column_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: HID_WIDTH,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(HID_BORDER),
                        ..default()
                    },
                    background_color: HID_BACKGOUND_COLOR.into(),
                    border_color: HID_BORDER_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        InHand(Hand::Left),
                        ImageBundle {
                            transform: Transform::from_translation((0.0, 0.0, 102.0).into()),
                            image: UiImage {
                                texture: weapons.texture.clone(),
                                ..default()
                            },

                            style: icon_style.clone(),
                            ..default()
                        },
                        TextureAtlas {
                            layout: weapons.texture_atlas_layout.clone(),
                            index: 0,
                        },
                    ));
                    parent.spawn(TextBundle::from_section("Left", TextStyle { ..default() }));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: HID_WIDTH,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(HID_BORDER),
                        ..default()
                    },
                    background_color: HID_BACKGOUND_COLOR.into(),
                    border_color: HID_BORDER_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        InHand(Hand::Right),
                        ImageBundle {
                            transform: Transform::from_translation((0.0, 0.0, 102.0).into()),
                            image: UiImage {
                                texture: weapons.texture.clone(),
                                ..default()
                            },

                            style: icon_style.clone(),
                            ..default()
                        },
                        TextureAtlas {
                            layout: weapons.texture_atlas_layout.clone(),
                            index: 0,
                        },
                    ));
                    parent.spawn(TextBundle::from_section("Right", TextStyle { ..default() }));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: HID_WIDTH,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: HID_BACKGOUND_COLOR.into(),
                    border_color: HID_BORDER_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(MaterialNodeBundle {
                        transform: Transform::from_translation((0.0, 0.0, 102.0).into()),
                        material: ui_materials.add(CustomUIMaterial {
                            settings: LevelMeterSettings::default(),
                            color_texture: Some(asset_server.load("sprites/excite.png")),
                        }),
                        style: icon_style,
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Excite",
                        TextStyle { ..default() },
                    ));
                });
        });
}

pub fn update_system(
    state: Res<SelectorResource>,
    mut _query: Query<&mut Text>,
    mut hand_q: Query<(&mut TextureAtlas, &InHand)>,
) {
    for (mut texture_atlas, InHand(hand)) in hand_q.iter_mut() {
        match hand {
            Hand::Left => {
                if let Some(i) = state.current_left {
                    texture_atlas.index = state.texture_index[i as usize] as usize;
                }
            }

            Hand::Right => {
                if let Some(i) = state.current_right {
                    texture_atlas.index = state.texture_index[i as usize] as usize;
                }
            }
        }
    }
}
