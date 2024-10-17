use avian2d::prelude::*;

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Player,
    PlayerBullet,
    Enemy,
}
