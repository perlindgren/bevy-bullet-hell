use bevy::{math::Vec3, prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
struct Layer(u8);

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles/simple_tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };

    // Layer 1
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    fill_tilemap(
        TileTextureIndex(0),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert((
        Layer(1),
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle.clone()),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
    ));

    // Layer 2
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    fill_tilemap(
        TileTextureIndex(2),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert((
        Layer(2),
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
            //  * Transform::from_xyz(32.0, 32.0, 0.0),
            ..Default::default()
        },
    ));

    commands.spawn((
        // PlayerComponent,
        // RigidBody::Dynamic,
        // Collider::rectangle(10.0, 10.0),
        // Sensor,
        SpriteBundle {
            texture: asset_server.load("sprites/cross.png"),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        // MassPropertiesBundle::new_computed(&Collider::rectangle(10.0, 10.0), 2.0),
    ));
}

// we assume camera is at layer 1, layer 0 is the background and layer 3 and above is above

fn parallax_update_system(
    time: Res<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    mut layer_query: Query<(&mut Transform, &Layer), Without<Camera>>,
) {
    let camera_translation = camera_query.single().translation;

    for (mut layer_transform, Layer(l)) in layer_query.iter_mut() {
        if *l == 1 {
            println!("here");
            layer_transform.translation.x = camera_translation.x * (-0.5);
            layer_transform.translation.y = camera_translation.y * (-0.5);
        }
    }
}

// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn camera_update_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut layer_query: Query<(&mut Transform, &Layer), Without<Camera>>,
) {
    let (mut transform, mut ortho) = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::KeyZ) {
        ortho.scale += 0.1;
    }

    if keyboard_input.pressed(KeyCode::KeyX) {
        ortho.scale -= 0.1;
    }

    if ortho.scale < 0.5 {
        ortho.scale = 0.5;
    }

    let z = transform.translation.z;

    let delta = time.delta_seconds() * direction * 500.;
    transform.translation += delta;
    // Important! We need to restore the Z values when moving the camera around.
    // Bevy has a specific camera setup and this can mess with how our layers are shown.
    // transform.translation.z = z;

    for (mut layer_transform, Layer(l)) in layer_query.iter_mut() {
        if *l == 1 {
            //  println!("here, delta {:?}", delta);
            layer_transform.translation.x += delta.x * (0.1);
            layer_transform.translation.y += delta.y * (0.1);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Layers Example"),
                        desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                //parallax_update_system,
                camera_update_system
            )
                .chain(),
        )
        .run();
}
