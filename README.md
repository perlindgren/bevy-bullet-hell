# Bevy Bullet Hell

Yea

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

Logging level can be set e.g., by   :

```shell
RUST_LOG=bevy_bullet_hell=debug cargo run --release
```
