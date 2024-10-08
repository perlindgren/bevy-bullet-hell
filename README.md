# Bevy Bullet Hell, No Pain No Gain (NPNG)

Yea!!!

Some rough game ideas, combining shooter/looter, bullet-hell, boss-battles, speed running, rogue lite elements. The game should be compact and super simple to grasp but equally hard to leave, aiming at infinite replayability and addiction. 

- Skills based shooter with high pace and relentless action.
- Limited set of arenas (say 12, one for each month), each with its own enemies and boss.Chose your load-out carefully to match your next arena challenge(s).

- Arenas:
  - Top down limited size arenas.
  - Arenas have different styles/themes (e.g., representative of the month or similar).
  - Each arena has a set of spawn points for different enemies and its boss. 
  - Each arena has one or several a loot box(es) available when arena is completed.
  - Bonus for finishing arena withing specific time (bronze, silver, gold, s-class) per arena difficulty.

- Game play:
  - The goal is to as fast as possible eliminate all enemies (without dying/taking damage etc. for additional bonuses).
  - Spawn rate and enemy difficulty scales with level.
  - Enemies (might) drop lives, health pills, ammo, weapons, time reduction, etc.
  - You can hold only a limited set of weapons, picking up new gear replace the currently held if not empty handed.
  - Only held weapons are kept, so pick wisely before leaving arena.
  - Kept loot goes to persistent chest available between arenas.
  - Load-out can be chosen from chest.
  - When entering an arena you can choose difficulty among individually unlocked arena levels (no-pain, no-gain for a reason).
  
- Weapon system:
  - Aim with right stick, distance shown indicate target on throw or radious for EMG like weapons.
  - L1 fire left hand weapon, hold for charge, fire on release, abort by pressing L2.
  - L2 select left weapon.
  - R1 fire right hand weapon, hold for charge, fire on release, abort by pressing L2.
  - R2 select right weapon.

  Weapon selection from a slot circle where the right stick angle determines selection.

  You have a limited set of weapon slots, starting from a single slot when when born, upgrading over time by opening chests.

- Weapons:
  - Melee style, short range, lots of damage at high risc of colliding. You may charge an attack either aimed or round house by right stick.
  - Thrown, bombs or EMG like (radius based), position/radius selected by right stick.
  - Shoot, lazer/gun/arrow like. Direction set by aim point. Range determined by weapon type/charging.
  
- Speed running:
  As mentioned each arena has timing goals for each arena per level. Additionally, there will be an on-line ranking system to match your score against fellow players worldwide.

- Roguelite challenge:
   Start from Arena 1, Level 1, and try to reach as far as possible in one run. Challenges comes in different flavours, ranging from exploratory (starting with 3 lives and a bag of healing pills) to ultimate instant kill (take any damage and you have to start all over again).

So far:

- Smooth steering/aiming and shooting with joystick control.
- Spinning rectangles bouncing at walls (no physics), showcasing enemy modelling.
- Pixel/frame perfect collision detection by Avian (using Parry), showcasing game play mechanics.
- Tiled background (with parallax, but that is currently not enabled), showcasing arena design.

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

## License

Free to play, server access might be restricted to 