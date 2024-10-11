use crate::common::*;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct WeaponsResource {
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

// setup system, for now hard coded to 4 weapons
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    debug!("load sprite_sheet {:?}", SPRITE_SHEET);
    let texture = asset_server.load(SPRITE_SHEET);

    // the sprite sheet has 10 sprites arranged in a row, and they are all 32px x 32px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 11, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(WeaponsResource {
        texture_atlas_layout,
        texture,
    });
}
