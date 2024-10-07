use crate::player::PlayerResource;
use bevy::prelude::*;

pub fn update_system(
    player_resource: Res<PlayerResource>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.single_mut();

    // let mut direction = Vec3::ZERO;

    let _z = transform.translation.z;

    // let delta = time.delta_seconds() * direction * 500.;

    transform.translation = player_resource.player_pos.extend(0.0) * 0.25;
    // Important! We need to restore the Z values when moving the camera around.
    // Bevy has a specific camera setup and this can mess with how our layers are shown.
    // transform.translation.z = z;

    //     for (mut layer_transform, Layer(l)) in layer_query.iter_mut() {
    //         if *l == 1 {
    //             //  println!("here, delta {:?}", delta);
    //             layer_transform.translation.x += delta.x * (0.1);
    //             layer_transform.translation.y += delta.y * (0.1);
    //         }
    //     }
    //
}
