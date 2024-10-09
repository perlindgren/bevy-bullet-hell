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
- Weapon and ability system:

  - Aim with right stick, distance shown indicates target position for throw or ability, or radius for EMG like weapons.
  - L1 fire left hand weapon, hold for charge, fire on release, abort by pressing L2.
  - L2 select left weapon/ability.
  - R1 fire right hand weapon, hold for charge, fire on release, abort by pressing L2.
  - R2 select right weapon/ability.

  Weapon and ability selection from a circular/slotted overlay where the right stick angle determines selection. Letting go of the selection button (L2/R2) confirms selection. If right stick in middle (neutral position, selection is aborted). This should go well with abort of charged weapon.

  You have a limited set of weapon slots, starting from a single slot when when born, upgrading over time by opening chests.

- Weapons:
  - Melee style, short range, lots of damage at high risk of colliding. You may charge an attack either aimed or round house by right stick.
  - Thrown, bombs or EMG like (radius based), position/radius selected by right stick.
  - Shoot, lazer/gun/arrow like. Direction set by aim point. Range determined by weapon type/charging.
- Speed running:
  As mentioned each arena has timing goals for each arena per level. Additionally, there will be an on-line ranking system to match your score against fellow players worldwide.

  Abilities:
  Along the game, you may pick up new abilities including:

  - Dodge roll (level 1, straight in current direction, level 2 steerable with left stick, level 3 targeting aim-point).
  - Superhero stomp, (level 1, low-jump to target, no impact, level 2 mid-jum to target with impact, level 3 high-jump to target, causing shock wave, level 4, chainable level 3 jumps, i.e. charging a next jump mid air will cause a chained stomp). Get ready to wreck havoc!!!
  - Abilities and charged attacks require excitement, you earn excitement by staying active, fast paced movements, causing damage and such. If you stay idle, you loose excitement over time as in real life.

- Roguelite challenge:
  Start from Arena 1, Level 1, and try to reach as far as possible in one run. Challenges comes in different flavours, ranging from exploratory (starting with 3 lives and a bag of healing pills) to ultimate instant kill (take any damage and you have to start all over again).

So far:

- Smooth steering/aiming and shooting with joystick control.
- Spinning rectangles bouncing at walls (no physics), showcasing enemy modelling.
- Pixel/frame perfect collision detection by Avian (using Parry), showcasing game play mechanics.
- Tiled background (with parallax, but that is currently not enabled), showcasing arena design.

## Playing

- Left stick for steering
- Right stick for aiming
- Shoot with any back button, bring it on!!!!

(For now no weapon selection.)

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

To be determined.
