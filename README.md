# Bevy Bullet Hell

Yea!!!

Some rough game ideas, combining:

- Skills based shooter within the bullet hell genre.
- Boss battles on size limited top down arena.
- Each arena has a theme, a set of spawn points for different enemies.
- Goal is to eliminate all enemies without dying (not surprising).
- Bonus for finishing arena withing specific time (bronze, silver, gold, s-class).
- Enemies might drop lives, ammo, weapons, time reduction, etc., during fight.
- Each arena provides a loot box for finishing enemy, essentially drops of higher value.
- Shop between arenas, allow you to tune your load-out.
- Limited set of arenas (say 10), each with its own enemies and boss, chose your load out carefully to match the next arena.
- You can hold only two weapons and a shield, picking
- Roguelite

So far:

- Steering/aiming and shooting with joystick control.
- Spinning rectangles bouncing at walls (no physics).
- Collision detection by Avian (using Parry).
- Tiled background (with parallax, but that is currently not enabled).

## Playing

Just a POC for now. No hit-boxes, yet. Not sure whether to go as long as there will be lots of bullets its gonna be fine.

- Left stick for steering
- Right stick for aiming
- Shoot with any back button, bring it on!!!!

```shell
cargo run --release
```

Takes < 3% CPU on my oldish laptop, so seems fairly efficient. Not that it currently does much.

## To Debug

Logging level can be set e.g., by :

```shell
RUST_LOG=bevy_bullet_hell=debug cargo run --release
```
