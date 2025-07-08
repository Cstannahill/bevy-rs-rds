# Project Plan: Magic Duel (ROUNDS-like Game in Rust & Bevy)

## Introduction and Concept

We are building a 1v1 arena shooter inspired by **ROUNDS**, but using **magic spells instead of guns**. In ROUNDS, each round is a frantic duel where the losing player picks a card power-up to improve their build. Our game will follow a similar _rogue-lite_ loop: players battle in short rounds, then gain **magical power-up cards** to shift the tide. The result will be a fast-paced, combo-heavy duel of sorcery. Each match is decided when one player wins a set number of rounds (e.g. first to 5 points, as in ROUNDS), making for intense comeback opportunities.

**Key Differences:** Instead of firearms and bullets, players cast spells and projectiles with elemental effects. This opens up creative mechanics like fireballs, ice shards, lightning bolts, etc. The card-based upgrade system will grant elemental powers, status effects (poison, slow, lifesteal), and stat boosts (damage, speed, spread) – similar in spirit to ROUNDS’ upgrades but with a fantasy twist. We want to capture the _synergy-driven gameplay_ of ROUNDS where combining multiple upgrades leads to wild results. _(For example, in ROUNDS certain card combos yielded effects like a “shotgun-rocket-launcher that shoots heat-seeking bouncy missiles,” demonstrating the outrageous combo potential.)_ Our design should encourage discovering broken combos and adapting strategies each round.

**Project Scope:** Initially, the game will support **1v1 local multiplayer** on PC (Windows/Linux) and the **Steam Deck**. We will implement local matches with two players on one machine (using gamepads or split keyboard controls). The code and architecture will be designed with future **online multiplayer** in mind, so we can later extend to networked play. As a solo developer new to Rust/Bevy, we’ll prioritize clear structure and leverage community tools/plugins to accelerate development. The following sections outline the tech stack, gameplay mechanics, and implementation plan in detail.

## Tech Stack Overview

To build this game, we will use a modern Rust-based game development stack. Below are the primary technologies and tools:

- **Programming Language – Rust:** The entire game will be written in Rust, known for its performance, reliability (memory safety), and strong package ecosystem. Rust ensures we can have smooth performance even on the relatively constrained Steam Deck hardware. Its ownership model helps avoid common game crashes (null pointers, data races), which is great for a solo dev project.

- **Game Engine – Bevy:** We will use the Bevy engine, a data-driven game engine built in Rust. Bevy is built around an **Entity-Component-System (ECS)** architecture, which suits our game well. ECS will let us define gameplay elements (players, projectiles, etc.) as entities composed of modular components (health, position, velocity, spell effects, etc.), with systems that operate on these components each frame. This makes it easier to manage game state and add/remove effects dynamically (ideal for applying card upgrades as component modifiers). Bevy is also cross-platform and open source, ensuring our game can run on Windows and Linux (Steam Deck uses Linux) with minimal hassle. We’ll use the latest Bevy version (0.11+), which includes an ergonomic ECS API and improved rendering, UI, and input support.

- **Rendering and Windowing:** Bevy’s rendering is built on **wgpu**, giving us WebGL/WebGPU support if needed. For our 2D game, Bevy’s built-in 2D sprite and shape rendering will be sufficient. We can use Bevy’s camera and orthographic projection for a 2D side-view arena. Bevy handles window creation, events, and can target desktop and web (though our focus is desktop native). The Steam Deck’s 800p resolution and gamepad input will be accounted for via responsive UI scaling and input mapping.

- **Physics and Collision – Rapier via Bevy:** For robust 2D physics (movement, gravity, collisions, bouncing projectiles, etc.), we will integrate the **Rapier** physics engine using the `bevy_rapier2d` plugin. Rapier is an efficient cross-platform physics library that provides rigid bodies, colliders, and event detection. By using `bevy_rapier2d`, we get easy physics integration through Bevy’s plugin system, so we can simulate gravity, detect when a projectile hits a player, handle bounces off walls, etc., without writing all collision logic from scratch. This is especially useful for a platformer-like game world. We will configure Rapier for 2D, defining colliders for players (probably capsules or rectangles) and for the arena boundaries and platforms. Projectiles can be lightweight entities with a collider and velocity – Rapier will generate contact events we can use to apply damage or effects. Physics simulation will also add juicy effects like knockback when players are hit, if desired.

- **Audio – Kira/Bevy Kira Audio:** Sound effects and music are important for game feel. While Bevy has basic audio support built-in, we’ll use the more robust **Kira** audio library via the `bevy_kira_audio` plugin. This plugin integrates Kira with Bevy’s ECS, letting us easily play sounds for events (e.g. casting a spell, hitting a player, round win music). Kira supports advanced features and multiple sound formats out-of-the-box. We’ll set up an audio system to load sound assets (casting swooshes, explosion booms, etc.) and play them at appropriate events. This ensures the game’s audio is dynamic and can be controlled through ECS (for example, adjusting volume or stopping sounds on game over).

- **User Interface – Bevy UI / Egui:** For menus, HUD (health bars, round score), and the card selection screen, we will use Bevy’s built-in UI system (which uses an immediate mode style, akin to CSS flexbox layouts) or consider `bevy_egui` for an easier HUD overlay. Bevy’s UI has improved in recent versions (with features like border styling and grid layouts), so it should suffice for our needs:

  - A main menu (Play Local, Options, Quit).
  - An in-game HUD showing each player’s health and current cards.
  - A **card selection screen** between rounds presenting 5 card choices. We’ll display card name, description, and stat changes (just like ROUNDS does).
  - All UI will be gamepad-friendly (navigable via d-pad or joystick) since on Steam Deck and couch play, mouse input isn’t guaranteed.

- **Input Handling:** Bevy’s input system will let us manage keyboard and gamepad controls. We will support two players:

  - **Player 1:** Keyboard (WASD for movement, **space to jump**, **F to shoot**, maybe arrow keys to aim if using aim, etc.) or first gamepad.
  - **Player 2:** Second gamepad (or an alternative key scheme). Bevy assigns unique IDs to each connected gamepad, which we can use to map inputs to specific players. This makes local multiplayer input handling straightforward – we’ll read gamepad events and route them to the appropriate player entity. We will likely create an input mapping resource or use an input plugin (like `leafwing_input_manager`) to define actions (move, jump, shoot, block) and bind them for each player. Ensuring both keyboard and controller can be used will make testing easier (solo developer can control both players with one on keyboard, one on controller).

- **Development Tools & Other Libraries:** As a newcomer to the ecosystem, it’s wise to leverage some helper tools:

  - **Bevy Inspector Egui:** We can include the `bevy_inspector_egui` plugin during development. This provides a debug UI where we can inspect and edit entity components live. This will help in tweaking values (player speed, gravity, damage) without constant recompile, and for visualizing the game state (e.g., seeing if a projectile’s effect component is attached).
  - **Bevy Assets & Community Plugins:** The Bevy community has many useful plugins listed on Bevy Assets. We will keep an eye on anything helpful, such as `bevy_prototype_lyon` (to draw shapes for hitboxes or debug visuals), or `bevy_gizmos` for debugging trajectories, etc. Since our game is 2D, we may not need heavy 3D assets, but we might use simple shapes or sprites for effects (Bevy’s built-in sprite support or shape drawing for things like circular explosion radius).
  - **Cross-Platform Build Tools:** Rust and Bevy support cross-compilation. We’ll set up CI or scripts to build for Windows and Linux. For Steam Deck specifically, building a Linux release (possibly with `cargo build --target x86_64-unknown-linux-gnu`) and testing on the device will be part of the pipeline. SteamOS is just Linux, so compatibility is mostly ensured by using SDL2 (which Bevy uses under the hood for windowing/input). We will ensure _gamepad rumble_ integration if possible (Bevy 0.11 introduced a Gamepad Rumble API which could enhance feedback on controllers).

- **Version Control & Project Management:** We will use Git for version control. The project can be a Cargo workspace if needed (though a single crate is fine to start). We’ll modularize the code by functionality (e.g., a module for components, one for systems like combat, one for UI, etc.) to keep it organized.

Overall, this tech stack centers on **Rust + Bevy** for core development, supplemented by specialized crates for physics, audio, and more. This modern toolkit gives us the power to realize a fluid, responsive game on PC and Steam Deck. Next, we detail the gameplay design and how we’ll implement it using these tools.

## Gameplay Mechanics and Design

### Core Arena Gameplay

&#x20;_Example of a **ROUNDS** match in action – a 1v1 duel in a 2D arena with simple geometric terrain. Players maneuver and jump on platforms while shooting projectiles at each other. The visual style is minimalistic and uses physics (you can see characters with “noodle arms” mid-jump, and bullet impacts) to create chaotic, fun combat._

Our game will adopt this _2D side-view arena_ format. Two wizard characters face off in a destructible or procedurally generated arena. The core mechanics include:

- **Movement:** Players can run left/right and jump (possibly double-jump or dash if we design spells for that). Gravity pulls players down, and they must navigate platforms and obstacles. Movement should be tight and responsive, as players will be dodging enemy attacks. Using Rapier, we’ll give players a **RigidBody** (with gravity enabled) and a **Collider** (for the floor/platform collisions). A `PlayerInput` component (with fields like `move_direction`, `is_jumping`, etc.) can be set each frame from input systems, and a movement system applies forces or directly sets velocities accordingly. We also must handle **facing direction** (players face the aim direction or direction of last movement) because spells could shoot in the direction the player is aiming. Aiming could be 360-degree (with right stick or mouse) or 8-directional; to keep it simple for gamepad, we might have aim tied to movement direction or use a second stick for aim if available.

- **Combat (Shooting/Casting):** Each player can cast spells that manifest as **projectiles or effects**. At game start, players have a basic attack (e.g., a small magic bolt). Pressing the “shoot/cast” button will spawn a projectile entity. In ROUNDS, you have a cooldown (reload time) between shots; we will similarly have a cast rate. We might add a secondary action like **Block** (as in ROUNDS, block was a shield that could also trigger certain card effects). In our magic context, “block” could be a **magic shield spell** with a cooldown, used to negate or reflect an incoming projectile, or trigger defensive power-ups. We will implement casting by listening to input events: when shoot is pressed and the player’s cooldown is ready, create a projectile entity at the player’s position. The projectile will have components: `Transform` (position/rotation), `Velocity` (movement vector), `Collider` (for collision detection), `Damage` value, and maybe an `Effect` component (to carry elemental effect info like “poison 2s” or “slow 50%”). The projectile’s motion is handled either by physics (Rapier can advance it with its velocity each frame) or by a custom system that translates it. We must also manage lifespan (destroy projectile after e.g. 3 seconds or if it goes out of bounds) to avoid infinite objects.

- **Health and Damage:** Players have a health pool. Getting hit by a projectile reduces health. Different spells can do different damage amounts. If health drops to zero, that player loses the round (we’ll handle round end logic separately). We’ll create a `Health` component for players. On collision events (projectile hits player), a system will detect the collision, identify the involved entities, and if a projectile hit a player that isn’t its owner, apply damage: subtract projectile’s damage from player’s health, possibly spawn hit effects (particles, sound), and then despawn the projectile. If health <= 0, trigger the round-end sequence for that player’s defeat. We should also consider **knockback**: certain powerful projectiles might impart force on hit – Rapier allows us to apply impulse to the player’s rigidbody for a knockback effect.

- **Status Effects:** This is where our magic twist shines. Projectiles can carry status effects (like **Poison**, **Slow**, **Life Drain**, etc.). Implementation: We can encode effects in the projectile, and on hit, attach an effect component to the player or directly modify their stats for a duration. For example:

  - _Poison_: After hit, player takes damage over time (DOT) for a few seconds. We could add a `Poisoned` component to the player with fields like damage_per_second and remaining_time. A system each tick will reduce remaining time and apply damage.
  - _Slow_: After hit, player’s movement speed is reduced for a duration. We could implement by adding a `Slowed` component (with a timer and speed modifier). Our movement system would check for `Slowed` on players and modify their speed.
  - _Life Drain (Vampiric)_: This effect heals the shooter by some amount when they deal damage. We can handle this at the moment of collision: if projectile has a Drain flag, when it deals damage, also heal its owner for X% of damage.
  - _Others_: **Stun** (prevent movement for a short time), **Burn** (a variant of poison with maybe different visuals), **Knockback** (as mentioned, impart a bigger impulse), **Area Damage** (explosive splash on hit), etc.

  We will use **cards (upgrades)** to grant these effects, so at the start a basic projectile might have none, but as players gain cards, their attacks start incorporating these properties. The ECS design makes this relatively easy: an upgrade can simply add a component or set a flag on the player or their projectiles. For example, if a player has the _Poison Shot_ card, we can attach a `PoisonOnHit` component to that player (or store it in a list of active modifiers). The projectile spawning system will see that and mark new projectiles with `PoisonOnHit` effect data. This keeps our logic modular.

- **Environment and Arenas:** The arenas will be composed of static geometry (like platforms, walls). ROUNDS had 70+ maps, often random assemblages of shapes. We can start with a few simple layouts (some rectangles as platforms at varying heights). We are free to use _procedural generation_ to create arenas – e.g., place a few platforms at random positions, ensuring they are reachable. This can give variety without manually designing many maps. Since “generated areas is fine” for our plan, we can implement a simple random level generator that creates, say, 5 floating platforms within bounds, maybe with some random variation in size. All these platforms will have colliders so players can land on them and projectiles can hit them (we might allow projectiles to either collide and destroy, or bounce off surfaces for trick shots). We can also add some environmental hazards via cards or design (for example, a card that creates spikes on walls that damage on contact, etc.). Initially, keep environment basic to focus on player combat.

- **Game Flow (Rounds):** The game will be organized in rounds. Both players start round 1 with base abilities. They fight; when one dies, that round ends. The surviving player gets a point (half-point in ROUNDS terms, but essentially one round win). Then we enter an **upgrade phase**: the losing player (in ROUNDS, only the loser) gets to pick **one out of several random cards** to upgrade their abilities. We will do the same:

  1. Pause the action and present (for now) 5 random cards to the loser. _(We can adjust the number – ROUNDS uses 5 by default.)_
  2. The loser uses controls to highlight and select one card. That card’s effects are then applied to that player’s character.
  3. Both players’ health are reset, maybe positions reset, and the next round begins with the new power in effect.
  4. This repeats each round. If one player was significantly weaker, over a few rounds the upgrades should help them catch up – this is a built-in comeback mechanic.
  5. The game ends when a player reaches the win condition (e.g. 5 round wins). Optionally, we could also end if someone collects too many cards (ROUNDS has a rule if someone loses after picking 5 cards, the match ends, to prevent infinite game length).

  We will implement a **GameState** finite state machine using Bevy’s state system or simply control flow:

  - States: MainMenu, InGame, RoundEnd, CardSelection, GameOver.
  - When in InGame state, if a player dies, transition to RoundEnd -> CardSelection state.
  - In CardSelection, show UI, pick card, apply it, then transition back to InGame for next round (or GameOver if win condition met).
  - This structure keeps the logic separated (we won’t run combat systems during card selection, etc., by using state-run criteria).

### Card Power-Ups and Upgrades

&#x20;_Illustration of the **card upgrade selection** (from ROUNDS): After a round, the losing player is presented with a choice of 5 upgrade cards, e.g. “Wind Up” (more bullet speed, damage), “Thruster” (bullets push targets), “Bombs Away” (spawn bombs when blocking), “Healing Field” (block creates healing aura), “Frost Slam” (slow enemies on block), etc. Each card shows its effects on stats (red/blue text). This system allows the defeated player to come back with a new advantage._

The **card system** is the heart of the game’s replayability and fun combo-making. We will design a set of elemental and magical upgrades that players can stack. Some design considerations for our cards:

- **Types of Cards:**

  - **Stat Boosters:** e.g. _Power Surge_ (increase damage by +20%), _Swift Casting_ (+20% projectile speed), _Rapid Fire_ (-0.2s cooldown between shots), _Glass Cannon_ (+50% damage but -25% HP).
  - **Elemental Effects:** e.g. _Poison_ (attacks inflict poison DOT), _Frostbite_ (attacks slow enemies), _Vampiric_ (lifesteal on hit), _Ignite_ (sets area on fire for area denial), _Shock_ (chance to stun).
  - **Projectile Behavior:** e.g. _Multi-Shot_ (shoot an extra projectile or a spread of 3 with reduced damage), _Bouncy_ (projectiles bounce off surfaces X times), _Homing_ (slight turn toward enemy), _Piercing_ (bullets go through shields or even through the player and can hit multiple times), _Explosive_ (AoE splash damage on hit).
  - **Defensive/Movement:** e.g. _Teleport_ (replace block with a short teleport dash), _Shield Bubble_ (on block, create a temporary bubble that blocks projectiles), _Healing Block_ (like Healing Field in Rounds – blocking heals you over time), _Thorns_ (melee attackers or close-range attackers take damage).
  - **Special Spells:** e.g. _Summon Turret_ (spawn a stationary orb that shoots for you), _Meteor_ (replace your shot with a slower, heavier high-damage projectile), etc. We can go wild here, but since it’s a first version, we’ll start with manageable, clear ones and expand.

- **Card Implementation:** Each card’s effects will be implemented as changes to the player’s components or the projectiles they spawn. We can maintain a `PlayerStats` or similar component that holds current values for things like damage, projectile speed, attack cooldown, etc. Picking a stat card just modifies those values. For effects that add new behavior (like poison on hit), we add a marker component or boolean flag in `PlayerStats` (e.g. `has_poison = true`). Alternatively, we maintain a list of “modifiers” in a component (like an enum for each effect active). The game logic systems (like the projectile firing system or the collision system) will check these and produce the appropriate outcome:

  - Example: If `PlayerStats.has_poison` is true, when firing a projectile, we attach a `PoisonEffect{damage: X, duration: Y}` component to the projectile.
  - Example: If `PlayerStats.multishot_level = 1`, when firing, instead of one projectile we spawn, say, 2 projectiles with a spread angle.
  - If a card changes the block ability (e.g. _Bombs on Block_), we would handle that in the block action logic: detect block usage and spawn bomb entities as specified. This might require a component like `BombsOnBlock` on the player to know to do that.

  We will create a data structure for cards, something like:

  ```rust
  struct Card {
      name: String,
      description: String,
      effects: CardEffects // maybe an enum or struct that details what it does
  }
  ```

  And have a list of all cards. When presenting random choices, we’ll randomly pick 5 distinct cards from the pool that the player hasn’t taken yet (to avoid duplicates unless we allow stacking the same card, which ROUNDS does allow stacking to some degree). For the first version, we can hard-code a handful of cards for each category to test the concept.

- **Synergy and Balancing:** Part of the fun is how cards interact. We should design them to **synergize** in unpredictable ways. For example, combining _Multi-Shot_ + _Poison_ + _Life Drain_ means you fire multiple projectiles, each inflicting poison and healing you – a very powerful combo if you land hits. In ROUNDS, some cards explicitly combo (like **Bombs Away + Echo** doubling the bombs). We might include combos like _Fire + Wind_ = creates a flame gust that lingers, etc. We won’t hard-code special interactions initially (aside from what naturally happens by stacking effects), but we will test to ensure no combo is game-breaking to the point of ruining fun (though a bit of imbalance is okay for a silly game, and players can always adapt since both sides get upgrades).

- **Progression during a Match:** Typically, the losing player gets a card. We might consider if the winning player ever gets something (in base ROUNDS, only the loser gets upgrades each round to help them catch up). We will follow that: only the player who lost the last round picks a new card. The winner retains whatever build they have. This naturally rubber-bands the game. If one player keeps losing, they might accumulate several cards and become quite formidable (which the leading player can still try to overcome with skill or their own one or two cards). This design helps prevent steamrolls and makes matches exciting. We will communicate this clearly in-game (e.g., an on-screen prompt: “Player 2 lost the round – select a card to power up!”).

- **Number of Rounds and Cards:** We can start with a win condition of 5 wins (first to 5 wins the match, same as ROUNDS default). That means at most one player could get 4 cards (if one player wins 5-0, the loser got 4 cards by the final round but still lost). Matches might typically go something like 5-3, meaning one player got 3 cards, the other 0 by game end. We should ensure even a single card can tilt the scales enough to occasionally cause a comeback, otherwise the first win might always lead to match win (we don’t want that). This balancing will be through playtesting values (damage increments, etc.). We may also implement an option to change round limit or allow both players to pick a card every round (as a custom mode) for more chaos.

### Visual & Audio Design

Though this is primarily a coding project, having a vision for visuals and audio helps:

- **Visual Style:** We will likely start with _minimalist shapes and simple effects_, similar to ROUNDS which had colored blob characters and abstract backgrounds. For characters, we can use simple shapes (circles or sprites) or even ASCII placeholders until we add art. One idea: The players could be represented as robed mages or elemental avatars. But as a solo dev with limited art resources, we may use colored circles or low-detail sprites initially. We can leverage Bevy’s shape drawing or simple textured quads. The **projectiles** can be small glowing orbs or elemental icons (a fireball sprite, an ice shard triangle, etc.). **Effects** like poison or slow can be indicated with particle systems or overlays (e.g., a green particle cloud for poison, a snowflake sprite on the victim for slow). Bevy has a rudimentary particle plugin (or one can be made using spawning lots of small sprites). We might later integrate a particle system crate if needed, but initially, a simple approach (spawn a few particles with diminishing scale) could work.

- **Feedback and Juice:** To make combat satisfying, we’ll add screen shake on big hits, flashing sprites on damage, and sound effects. **Camera shake** can be done by slight random offset of camera for a few frames on explosions. **Hitflash**: when a player is hit, briefly tint their sprite red or white – this can be done by a material swap or using a shader/material that we toggle. **Numbers or health bars**: show damage numbers floating or just rely on health bars at top. **Screen freeze**: a very brief pause on impact can emphasize powerful hits (we can achieve this by intentionally delaying physics or game update for a few milliseconds, but carefully to not break determinism if we go online). All these polish items make the game feel good. We should code them in a flexible way (maybe a helper system that listens for a “ImpactEvent” and then triggers camera shake, etc.).

- **Audio:** As mentioned, we will use `bevy_kira_audio`. We should gather or create some sounds: e.g. a _shoot sound_ (fire whoosh), _impact sound_ (thud or explosion for hits), _block sound_ (magical clang), _jump sound_, _death sound_, and background music for matches. Since we target possibly Steam Deck play, ensure the audio mix is clear on small speakers (no overpowering bass that the Deck can’t handle well). We can allow volume adjustments via an options menu (Bevy UI slider that changes `Audio` resource volume). Using Kira, we could also do nice tricks like altering pitch for variety on repeated sounds.

- **HUD Elements:** Show each player’s HP in a bar or numerical. Possibly show icons for the cards they’ve picked (so you can see what your opponent has – ROUNDS displays the icons of upgrades on the side of the screen for each player). We can implement this by having an UI area for each player listing their active card icons (we need small icons for each card – initially maybe just colored squares or letters if we don’t have art). The card selection screen itself we saw in the image \[20] – we will make a simplified version: perhaps just five boxes with text (since implementing animated cards might be time-consuming). Over time, we can enhance it with card art.

- **Steam Deck Considerations:** The game should be tested at **1280x800** resolution (the Deck’s screen) to ensure UI is readable at that size. We should also ensure controller-only input works for everything (no reliance on keyboard/mouse in menus). Steam Deck’s GPU can handle Bevy fine, especially for 2D, but we should still optimize (avoid too many unnecessary entities, use fixed time step for physics to avoid inconsistencies). Also, support pausing (the Deck player may suspend the game – our game should handle losing focus gracefully by pausing).

## Architecture and Implementation Plan

With the design established, we break down how to implement this step by step using our tech stack. The project will follow an ECS architecture with clear separation of concerns. Here’s a plan of the major components and systems:

### Entities & Components

We will define the primary **entity types** in the game along with key components they use:

- **Player Entity**: Represents a player (wizard).

  - Components:

    - `Transform` (position, rotation) – provided by Bevy.
    - `Velocity` or Rapier physics `RigidBody` – for movement.
    - `Collider` – for collision (likely a capsule or circle for the body).
    - `Health { current, max }` – to track HP.
    - `PlayerTag { id: 1 or 2 }` – to distinguish Player 1 vs Player 2.
    - `PlayerInput` – custom component or resource binding for current input state (could also just read input resources directly in systems, but a component might store things like desired direction, or we can directly apply forces in an input system).
    - `Stats` – holds tunable stats (move_speed, jump_force, shot_damage, shot_speed, shot_cooldown, etc.). This gets modified by cards.
    - `Upgrades` – perhaps a list of Card IDs the player has taken (for reference/UI).
    - **Optional**: `ActiveEffects` – if player is currently under an effect like Poison or Slow, we might attach components like `Poisoned` or `Slowed` with timers. Alternately, we manage these via events or within the `Stats` (e.g. have `stats.speed_modifier = 0.5` if slowed). Components are cleaner for separating concerns.
    - **Optional**: `WeaponCooldown` – could be a component with a timer until next shot (or we incorporate this into Stats or as a resource per player).

- **Projectile Entity**: Represents a spell projectile (fireball, etc.) that moves and can hit things.

  - Components:

    - `Transform`, `Velocity` – initial position set to player’s position, velocity set based on aim.
    - `Collider` (likely a circle or small shape) – to detect hits.
    - `Damage` (amount of damage on hit).
    - `Owner` (player id who shot it, so we don’t damage the shooter or to attribute kill credit).
    - `Lifetime` – a timer component to self-destruct after a certain duration if it doesn’t hit anything.
    - `Effects` – this could be several components or one component with flags for special behaviors:

      - e.g. `PoisonEffect { damage_per_sec, duration }` if it inflicts poison.
      - `SlowEffect { amount, duration }`.
      - We could combine these into one component like `OnHitEffects { poison: Option<PoisonData>, slow: Option<SlowData>, lifesteal: f32, ... }` to bundle, but it might be simpler to attach multiple marker components (PoisonTag, SlowTag, etc.) each carrying their info. ECS allows an entity to have multiple effect components and the collision system can handle each accordingly.

    - Possibly a `Sprite` or some visual marker (or we might render them as shapes if not using sprites).

- **Arena/Level Entities**:

  - _Static Terrain:_ Entities for each platform or wall.

    - Components: `Transform`, `Collider` (probably `RigidBody::Fixed` since they don’t move), maybe a `Renderable` (could be a simple colored rectangle sprite or mesh).
    - We might also tag them with something like `Environment` or `Obstacle` if needed.

  - We may have one entity for the background if needed (purely visual, no collider).
  - If we include hazards (like lava zones or spikes), those would be environment entities with colliders and maybe a `Hazard { damage_per_sec }` component so that if a player touches it, they take damage.

- **UI Entities**:

  - Bevy UI is typically not entities per se for each element (they are, but managed through UI system). We will create UI in systems: e.g., spawn Text or Image entities for health bars, card selection buttons, etc., when entering a state.
  - We’ll maintain a `UIState` that tracks what UI is visible. For example, a health bar could be an entity with a `PlayerTag` to know which player it represents, and we update its width based on health in a system.
  - For card selection, each card option might be a UI node entity with an index, and navigable with gamepad/keyboard.

- **Misc**:

  - We might have an entity for an **Explosion** effect if we want a lingering AoE (e.g., after an explosive projectile hits, spawn an entity with a `ExplosionArea` component that deals damage in area for a short time, and a collider/sensor to detect players in range).
  - Particles could be entities too (like spark particles that just have a short life and a moving sprite), or we might use an automated particle system.

### Systems and Gameplay Logic

We will implement various **systems** (Bevy systems are Rust functions run each frame or triggered by events) to handle game logic. Major systems include:

- **Input Systems:**

  - `gamepad_input_system` – reads `Gamepad` and `Keyboard` input resources each frame, maps them to actions for each player. For instance, if Gamepad 1 left stick is tilted, set Player 1’s movement direction. If Gamepad1 “A” (South button) is pressed, trigger jump for Player 1. We can update the `PlayerInput` component or directly apply to physics (like apply an impulse for jump).
  - Alternatively, we could use an input mapping crate to simplify, but it’s manageable to do custom. We just need to ensure it works for two players distinctly. We’ll utilize Bevy’s support where each gamepad has an ID, so we know which player it belongs to.
  - This system will also handle menu navigation input (depending on current state, e.g., if state is CardSelection, the input goes to highlight cards instead of moving player).
  - **Note:** We must disable or ignore player movement input during card selection or menus. This can be done by adding conditions to systems (Bevy allows filtering systems by state).

- **Movement & Physics Systems:**

  - If using Rapier, a lot of movement integration is handled by Rapier’s physics stepping. We will configure Rapier to run each frame (possibly fixed timestep). We will still need to apply control: e.g., set the horizontal velocity of the player’s rigidbody based on input (so players can run). We might write a system `apply_player_movement` that reads `PlayerInput` and sets the velocity accordingly, or uses Rapier’s forces.
  - Jumping can be handled by applying an upward impulse to the rigidbody when jump input is detected (ensuring player is “grounded” first – we might track on the player if they are on ground via collision events or a sensor at their feet).
  - A separate system or the same can handle things like clamp velocity (maybe max fall speed) or apply friction (or we rely on physics material friction).
  - We will integrate this carefully with Rapier to avoid fighting the physics engine (for example, if we set velocity directly each frame for movement, we should probably set mode to kinematic or use forces for a dynamic body).

- **Shooting/Casting System:**

  - This system checks if players are trying to shoot and can shoot (i.e., input pressed and cooldown ready).
  - For each player with a shoot command, if their `WeaponCooldown` is <= 0, spawn a Projectile entity:

    - Determine spawn position (usually player’s position + some offset forward).
    - Determine velocity vector. If we allow aiming with analog stick, we use that direction; otherwise, maybe shoot towards the other player or in the facing direction.
    - Spawn the entity with all components as discussed. If using Rapier, we add a `RigidBody` (probably a dynamic body for the projectile) and set its velocity. Alternatively, we use kinematic bodies for bullets (no gravity unless a specific spell has it).
    - Set the projectile’s damage from player’s stats.
    - Attach effect components for any active effects this player has (poison, etc.).
    - Set a despawn timer (Lifetime).
    - Start the shooter’s cooldown (e.g., set `WeaponCooldown = base_cooldown * any modifiers`).

  - This system, along with movement, runs only in the InGame state.

- **Projectile Update & Despawn:**

  - If not fully relying on physics stepping, a system can move projectiles forward (but if using Rapier, it updates positions automatically).
  - A simple system will decrement `Lifetime` timers and despawn projectiles whose time is up to avoid clutter.
  - If we allow projectiles to bounce, we can either:

    - Use Rapier’s restitution (bounciness) on colliders to have them bounce off walls.
    - Or manually detect collisions and reflect velocity. Rapier collision events will let us know when a projectile hits a wall; we could then multiply its velocity by -1 (with some damping to simulate energy loss) if we want limited bounces. Or just rely on restitution = 1 for perfect bounce.
    - Possibly limit number of bounces by adding a component `BouncesRemaining` on projectile, decrementing each bounce and if 0 then make the projectile non-bouncy or schedule despawn on next hit.

- **Collision Handling (Projectile Hits):**

  - Using Rapier’s events, we will have a system listening for collision events between projectiles and players. Rapier can give us collision pairs and some info. We filter:

    - If `EntityA` is projectile and `EntityB` is player (or vice versa), and the projectile’s owner is not that player (so they didn’t hit themselves, though friendly fire in 1v1 is moot), then:

      - Apply damage to the player’s Health.
      - If the projectile has effects, apply those:

        - If projectile has `PoisonEffect`, attach a `Poisoned` component to player (or set a status in player).
        - If `SlowEffect`, attach `Slowed`.
        - If life drain, heal the shooter’s health by the specified amount (capped at max maybe).

      - Possibly generate a hit particle or sound here.
      - Despawn the projectile (unless we want piercing shots that continue – if piercing, then don’t despawn and allow it to hit another or until its lifetime ends).

    - If projectile hits environment (wall/floor):

      - If not bouncy, despawn it.
      - If explosive effect, trigger explosion (spawn explosion entity that deals AoE).
      - If bouncy, let Rapier handle bounce via restitution (or manually adjust).

  - If using Rapier, ensure to mark certain collisions as sensors if needed (like explosion AoE might be a sensor area).
  - We’ll need to register collision filter rules (so perhaps players don’t collide with each other physically, or maybe they can bump – we can decide if players have collision or pass through each other. ROUNDS players do collide I think, but “noodle arms” means not too precise; we might allow slight push).
  - This system will also handle player death: if after applying damage, Health <= 0, then mark that player as dead and trigger round over.

    - We might not remove the player entity, but just set a flag or directly trigger the state change to RoundEnd. Possibly despawn all projectiles too at round end.

- **Round Management System:**

  - Keeps track of score and transitions between round phases.
  - Perhaps use a resource `RoundManager` with fields: `p1_score`, `p2_score`, `round_active: bool`, etc. Or use States more explicitly.
  - When a player dies (we can fire an Event like `PlayerDied { winner: PlayerId }` from collision system), this system catches it:

    - Increment winner’s score.
    - Check if winner’s score reached win_condition (e.g. 5). If yes, transition to GameOver state (we’ll then show final results).
    - If not, transition to CardSelection state, recording who was loser (the one who died) as the one who gets to pick a card.

  - On entering the CardSelection state, spawn the UI for card choices as described.
  - On exiting CardSelection (after picking), apply the card effects to the loser’s components.
  - Reset round: restore both players’ health to max, reposition them (e.g. on opposite sides of the map or a default spawn point), clear any transient effects (like remove Poisoned status, etc., to start fresh each round unless the effect is permanent via a card).
  - Then transition back to InGame state to start the next round countdown.

- **Card Selection System:**

  - While in the CardSelection state, a system will handle input to navigate and select cards.
  - We can treat the 5 card choices as an array. If using keyboard/gamepad: the player can press left/right to cycle highlight (or up/down if we lay them out differently), and A or Enter to pick.
  - We will highlight the selected card (maybe enlarge it or change color). The image \[20] shows the selected card on the right being highlighted.
  - Once selection is made, we retrieve that card’s data and call a function to apply it to the player. Then trigger state transition (back to InGame or GameOver).
  - The UI entities for cards will be despawned or hidden after selection.
  - We also need to be careful the other player can’t accidentally control this menu – likely we restrict input to only the loser’s input device during this phase. That can be done by logic (if Player2 lost, only read Player2’s gamepad for the selection, and ignore Player1’s inputs or don’t spawn a cursor for them).

- **Apply Card Effects:**

  - This can be done either immediately at selection or via the Round Manager. Essentially, for each possible effect:

    - If stat boost, modify the Stats component on that player (e.g. stats.damage \*= 1.2 for +20% damage).
    - If new effect on hit, set a flag or add a component to player as discussed (like add `PoisonOnHit` marker).
    - If something altering block ability (like Bombs Away), add `BombsOnBlock` component.
    - If adding a new ability (like double jump), set a flag or increase a stat (could treat double jump as just jump_count_allowed = 2).
    - We might want to display a brief confirmation (like “Chosen: Frost Slam – Your blocks now slow enemies!”) – could be an UI popup for a second.

  - It’s good to separate this out so it’s easy to add new card effects without monolithic code. Possibly use a scripting approach: maybe define card effects in data, but as a first pass, straightforward if-else or match on card name in a function to apply changes is fine.

- **Status Effect Systems:**

  - For each status like Poison or Slow that persists:

    - A `poison_system` will tick on players with `Poisoned` component: reduce timer, apply damage over time (maybe every second or continuously each tick proportional to time). If timer <= 0, remove the component.
    - A `slow_system` similarly will handle slow timers and maybe reset speed when done if we applied a modifier.
    - Alternatively, we handle the effects in one system that goes through players and checks if they have any status components and updates them.

  - These run during gameplay rounds.

- **UI Update Systems:**

  - Health bar updater: reads each player’s Health and updates the corresponding UI bar fill or text.
  - Score display: update score UI (e.g., “Round 3: Player1 2 – Player2 1” or such).
  - Card UI during game: possibly update a small icon list of each player’s cards.
  - These systems ensure the HUD reflects the game state every frame or on relevant changes.

- **Menu Systems:**

  - In MainMenu state, we might have a system that listens for input (like pressing Start or clicking “Play”) to transition to starting a match. Since our focus is gameplay, menu can be simple (straight to game or allow level select, etc., later).

### Data and Configuration

We will keep configurable values either in code constants or load from external files for easy tweaking:

- Example config parameters: player base health, base move speed, base jump impulse, gravity scale, base projectile damage, base cooldown, etc., and effect magnitudes (poison damage per second, slow percentage, etc.).
- It might be beneficial to store card definitions in a JSON or Ron file that we load at startup. But given the complexity of “effects” (which might involve attaching components), it could be simpler to define them in code (we can define a bunch of card structs in a Rust module).
- We can group balance-related constants in one place for easy tuning.

### Multiplayer and Networking Considerations

While initial implementation is local only, we want to keep the door open for **online multiplayer** in the future. Some design choices to facilitate this:

- **Deterministic Logic:** If we ever use rollback netcode (like GGPO style via GGRS), the game’s logic needs to be deterministic given the same inputs. Using a fixed-update loop for physics and avoiding random number usage (or seeding any randomness consistently) is necessary. Rapier can be deterministic if using fixed timesteps and same hardware, but across different hardware might have floating point differences. If we intend rollback, we may need to lock step the physics or consider an engine like **GGRS** (Good Game Rollback System) which has Bevy integration. GGRS can save and rewind state; it requires a deterministic update. We might in the future replace Rapier physics with a simpler custom physics for determinism, or ensure we run the same Rapier version/algorithm on all clients and hope for determinism (risky). For now, we acknowledge that significant work is needed for true rollback networking, but our architecture of separating state (components) and using ECS systems is compatible with rollback approaches (in fact, `bevy_ggrs` exists to integrate GGRS with ECS).

- **Lockstep or Client-Server:** Another approach for online is a client-server model (with one authority simulating physics). This might be simpler to implement but can introduce latency in a fast-action game. We could consider a hybrid: since it’s 1v1, peer-to-peer with rollback is ideal (like fighting games or ROUNDS presumably uses a form of rollback). We will design our game loop so that it can run in a networked context:

  - Abstract the input source: currently input comes from local devices. We can design the game such that if networking is on, inputs come in via network packets instead but drive the same systems.
  - Keep game state synchronized: for client-server, we’d have to sync entity transforms, health, etc. We could use a crate like **bevy_quinnet** (for a simple QUIC-based networking) or **bevy_renet** (which is designed for fast-paced games with some snapshotting). These can send component updates over the wire.
  - We won’t implement networking now, but by not hardcoding anything specifically single-player (like avoid usage of singletons for player, instead always refer to player entities by id), we keep flexibility. Also, by using events for things like “PlayerDied” or “ProjectileHit”, those events could later be made to also send network messages.

- **Multiplayer Sync and Prediction:** With a physics engine and many possible projectiles, rollback netcode (via GGRS) might actually be the smoother approach (simulate both players’ inputs locally and correct as needed). If in future, we attempt this:

  - Use GGRS crate: it wraps input delay and frame rollback. The Johan Helsing blog series demonstrates making a Bevy game with GGRS and Matchbox (for WebRTC matchmaking). Adapting that, we’d feed GGRS the local and remote inputs, and it will step our game. We’d likely have to mark which components should rollback (like position, health, etc.) and which not (like score maybe).
  - Alternatively, for a simpler initial online implementation, we could do a turn-based or slow server-authoritative sync (but that wouldn’t feel good in an action game).

For now, the plan is: **implement the game fully for local play**, but keep functions modular so that input and update can be hijacked or augmented for online. We may also structure the game loop with a constant tick rate (say 60 FPS fixed logic tick) which is a common practice for network sync, rather than tying to render frames. Bevy’s schedule can be configured to run fixed update via `FixedTimeStep` schedule.

One concrete thing: we won’t use any truly non-deterministic functions without control. If we need randomness (like procedural level generation or random card draw), we will seed a RNG at match start and use that seed for the whole match. That way both players (in a network scenario) could generate the same random results for level layout or card RNG if they share the seed. This is how rollback games maintain sync on random events.

### Project Phases (Milestones)

Given the complexity, it’s useful to break development into phases with incremental goals:

1. **Basic Movement & Shooting (Single-player sandbox):** Set up Bevy app, add one player entity, allow moving and jumping on a simple platform, and shooting a projectile. Get collisions working (player shooting a wall or target dummy). This tests physics, input, rendering.
2. **1v1 Local Multiplayer Basics:** Add a second player entity, set up input for two controllers (or one keyboard vs one controller), ensure both can move and shoot without interference. No rounds or scoring yet; just get two players existing and interacting (projectiles can hurt the other).
3. **Health & Round System:** Introduce health for players and detect when one dies. Implement round resetting: declare one player winner of round, respawn or reset players. At this stage, implement a basic score count and win condition to declare match over (e.g., print to console or simple text).
4. **Card Selection System:** Develop the upgrade system:

   - Create a set of sample card definitions (maybe 5-10 to start).
   - After each round, pause and let the losing player choose a card (could initially be just pressing a number key for simplicity, later full UI).
   - Apply the card’s effect to that player.
   - Resume next round and verify the effect works (e.g., pick a damage+ card and see that next round you kill enemy faster, etc.).
   - This is a crucial phase to test that stacking upgrades works without breaking anything.

5. **User Interface & Polish:** Build the UI elements:

   - Health bars, score display.
   - Card selection menu with nice layout (as in image \[20], even if not as pretty).
   - Main menu flow (start match, maybe choose some options like number of rounds).
   - Feedback: add sound effects for key actions, screen shake, on-hit flash, etc., as time permits.

6. **Testing & Balancing:** Play the game (with a friend if possible) to see if the upgrades are noticeable and fun. Adjust values of cards (maybe one card is too strong or too weak). Check that matches tend to be balanced (if one player wins first two rounds, does the other have a fair chance with the upgrades they get?). We may fine-tune things like how many cards to offer, whether to allow duplicates, etc.
7. **Platform-specific Adjustments:** Test on Windows and on a Steam Deck (or at least simulate lower resolution and controller input on PC). Optimize performance if needed: though our game should be lightweight, ensure no obvious bottlenecks (profile if required). Make sure the control scheme is comfortable on a controller.
8. **Future (Online Multiplayer):** Once the core is solid, investigate adding online:

   - Could be another milestone on its own. Possibly start by integrating `bevy_quinnet` or `bevy_renet` to do a simple client-server sync of positions just to see something moving remotely.
   - Or try `bevy_ggrs`: this is more complex but aligns with how fighting games handle networking. There is already an example of hooking Bevy game with rollback, which we can study.
   - This will require heavy testing to ensure determinism (floating point differences might be a bugbear).
   - Given the scope, we might defer this until we have the local game polished.

Throughout development, maintain good coding practices: use Rust’s strengths (type checking, pattern matching for game states, etc.), add comments, and perhaps write some tests for isolated logic (like a function that applies card effects could be unit-tested).

## Conclusion and Resources

In summary, this project will create a spell-based 1v1 roguelite shooter with a rich card upgrade system, using Rust and the Bevy engine. We have outlined the technical stack (Bevy ECS, Rapier physics, Kira audio, etc.) and the gameplay design in depth. By following the ECS architecture and leveraging Bevy’s plugins, we can iteratively build this game, testing as we go. The end result should be a highly replayable local multiplayer game that can potentially extend to online play, much like the inspiration game ROUNDS but with our own magical theme and twists.

Since you are new to Rust/Bevy, here are some **resources and tips** to help you ramp up:

- **Bevy Official Book:** The Bevy website has a great \[Quick Start Guide and official book] which goes over fundamentals of setting up a Bevy app, adding systems, resources, etc. It’s a must-read to get familiar with Bevy’s conventions.
- **Bevy Cheatbook:** An unofficial but very comprehensive guide (you saw the Gamepad section from it). It covers common patterns and how to do various things in Bevy (input, physics, UI, etc.) with examples. Use it whenever you’re unsure how to implement something in Bevy.
- **Rust Language Resources:** If you find certain Rust concepts tricky (borrowing, lifetimes), the official Rust Book and Rustlings exercises are very helpful. Also, the Rust and Bevy communities on Discord are friendly if you get stuck on an error.
- **Community Examples:** There are many open-source example projects using Bevy. For instance, check out Bevy’s official GitHub repository for the **examples** directory – they have small demo programs showing how to use features (like 2d physics, UI, etc.). Seeing working code can guide your implementation.
- **Small Steps:** Develop and test in small increments. With Rust, you can use `cargo run` for a quick cycle. Make use of `cargo watch -x run` to auto-recompile on changes (or `cargo watch -cx "run --features bevy/trace"` if you want to see logs with trace info). This helps in rapid iteration, especially since compile times can grow as project grows.
- **Debugging:** Use println!/logging generously to track what’s happening (e.g., print when a collision occurs, or when a card is applied). Bevy’s log can be enabled to debug levels for more insight.
- **Profile/Optimize Later:** Initially focus on correctness and fun gameplay. Rust and Bevy are quite efficient; only optimize if you notice performance issues. For example, thousands of entities might slow down, but our game likely will stay in the low hundreds of entities (which is fine).
