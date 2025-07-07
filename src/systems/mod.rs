use crate::components::Lifetime;
use crate::components::{Health, Inventory, Player, Projectile, Stats, Velocity};
use crate::events::PlayerKilled;
use crate::resources::{CardSelection, RoundManager};
use crate::states::GameState;
use bevy::prelude::*;

const GRAVITY: f32 = -600.0;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-100.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::splat(30.0)),
                ..default()
            },
            ..default()
        },
        Player { id: 1 },
        Health {
            current: 100.0,
            max: 100.0,
        },
        Stats {
            move_speed: 200.0,
            jump_force: 400.0,
            damage: 10.0,
            projectile_speed: 300.0,
            shot_cooldown: 0.5,
            cooldown_timer: 0.0,
        },
        Velocity::default(),
        crate::components::Inventory::default(),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::splat(30.0)),
                ..default()
            },
            ..default()
        },
        Player { id: 2 },
        Health {
            current: 100.0,
            max: 100.0,
        },
        Stats {
            move_speed: 200.0,
            jump_force: 400.0,
            damage: 10.0,
            projectile_speed: 300.0,
            shot_cooldown: 0.5,
            cooldown_timer: 0.0,
        },
        Velocity::default(),
        crate::components::Inventory::default(),
    ));
}

pub fn player_input(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(&Player, &mut Stats, &Transform, &mut Velocity)>,
) {
    for (player, mut stats, transform, mut velocity) in query.iter_mut() {
        let mut direction = 0.0;
        match player.id {
            1 => {
                if keyboard.pressed(KeyCode::A) {
                    direction -= 1.0;
                }
                if keyboard.pressed(KeyCode::D) {
                    direction += 1.0;
                }
                if keyboard.just_pressed(KeyCode::W) && transform.translation.y <= 0.0 {
                    velocity.linvel.y = stats.jump_force;
                }
                if keyboard.pressed(KeyCode::Space) && stats.cooldown_timer <= 0.0 {
                    spawn_projectile(&mut commands, player.id, &*stats, transform);
                    stats.cooldown_timer = stats.shot_cooldown;
                }
            }
            2 => {
                if keyboard.pressed(KeyCode::Left) {
                    direction -= 1.0;
                }
                if keyboard.pressed(KeyCode::Right) {
                    direction += 1.0;
                }
                if keyboard.just_pressed(KeyCode::Up) && transform.translation.y <= 0.0 {
                    velocity.linvel.y = stats.jump_force;
                }
                if keyboard.pressed(KeyCode::Return) && stats.cooldown_timer <= 0.0 {
                    spawn_projectile(&mut commands, player.id, &*stats, transform);
                    stats.cooldown_timer = stats.shot_cooldown;
                }
            }
            _ => {}
        }
        velocity.linvel.x = direction * stats.move_speed;
    }
}

pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, Option<&mut Stats>)>,
) {
    for (mut transform, mut velocity, stats) in query.iter_mut() {
        velocity.linvel.y += GRAVITY * time.delta_seconds();
        transform.translation.x += velocity.linvel.x * time.delta_seconds();
        transform.translation.y += velocity.linvel.y * time.delta_seconds();
        if transform.translation.y < 0.0 {
            // simple ground
            transform.translation.y = 0.0;
            velocity.linvel.y = 0.0;
        }
        if let Some(mut s) = stats {
            if s.cooldown_timer > 0.0 {
                s.cooldown_timer -= time.delta_seconds();
            }
        }
    }
}

pub fn projectile_cleanup(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (entity, transform) in &query {
        if transform.translation.y > 600.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut life) in query.iter_mut() {
        life.time_left -= time.delta_seconds();
        if life.time_left <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn aabb_collision(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < (a_size.x + b_size.x) * 0.5
        && (a_pos.y - b_pos.y).abs() < (a_size.y + b_size.y) * 0.5
}

pub fn projectile_player_collision(
    mut commands: Commands,
    mut players: Query<(Entity, &Player, &mut Health, &Transform)>,
    mut projectiles: Query<(Entity, &Projectile, &Transform)>,
    mut kill_writer: EventWriter<PlayerKilled>,
) {
    let player_size = Vec2::splat(30.0);
    let proj_size = Vec2::splat(10.0);
    for (proj_entity, projectile, proj_transform) in projectiles.iter_mut() {
        for (_player_entity, player, mut health, player_transform) in players.iter_mut() {
            if player.id == projectile.owner {
                continue;
            }
            if aabb_collision(
                proj_transform.translation,
                proj_size,
                player_transform.translation,
                player_size,
            ) {
                health.current -= projectile.damage;
                commands.entity(proj_entity).despawn();
                if health.current <= 0.0 {
                    kill_writer.send(PlayerKilled {
                        winner: projectile.owner,
                        loser: player.id,
                    });
                }
                break;
            }
        }
    }
}

pub fn round_manager(
    mut commands: Commands,
    mut manager: ResMut<RoundManager>,
    mut selection: ResMut<CardSelection>,
    mut reader: EventReader<PlayerKilled>,
    mut players: Query<(&Player, &mut Health, &mut Transform)>,
    projectiles: Query<Entity, With<Projectile>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in reader.iter() {
        match event.winner {
            1 => manager.p1_score += 1,
            2 => manager.p2_score += 1,
            _ => {}
        }

        for entity in &projectiles {
            commands.entity(entity).despawn();
        }

        for (player, mut health, mut transform) in players.iter_mut() {
            health.current = health.max;
            transform.translation = if player.id == 1 {
                Vec3::new(-100.0, 0.0, 0.0)
            } else {
                Vec3::new(100.0, 0.0, 0.0)
            };
        }

        info!("Scores - P1: {} P2: {}", manager.p1_score, manager.p2_score);

        if manager.p1_score >= manager.rounds_to_win || manager.p2_score >= manager.rounds_to_win {
            info!("Game Over");
            next_state.set(GameState::GameOver);
        } else {
            selection.loser = Some(event.loser);
            selection.choices = crate::cards::random_choices(3);
            info!("Player {} choose a card:", event.loser);
            for (i, c) in selection.choices.iter().enumerate() {
                info!("{}: {} - {}", i + 1, c.name, c.description);
            }
            next_state.set(GameState::CardSelection);
        }
    }
}

fn spawn_projectile(commands: &mut Commands, owner: usize, stats: &Stats, transform: &Transform) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::splat(10.0)),
                ..default()
            },
            transform: Transform::from_translation(
                transform.translation + Vec3::new(0.0, 10.0, 0.0),
            ),
            ..default()
        },
        Projectile {
            owner,
            damage: stats.damage,
        },
        Lifetime { time_left: 2.0 },
        Velocity {
            linvel: Vec2::new(0.0, stats.projectile_speed),
        },
    ));
}

pub fn card_input_system(
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut selection: ResMut<CardSelection>,
    mut players: Query<(&Player, &mut Stats, &mut Inventory)>,
    state: Res<State<GameState>>,
) {
    if state.get() != &GameState::CardSelection {
        return;
    }
    let loser = match selection.loser {
        Some(id) => id,
        None => return,
    };
    let mut picked = None;
    if keyboard.just_pressed(KeyCode::Key1) {
        picked = Some(0);
    } else if keyboard.just_pressed(KeyCode::Key2) {
        picked = Some(1);
    } else if keyboard.just_pressed(KeyCode::Key3) {
        picked = Some(2);
    }
    if let Some(idx) = picked {
        if let Some(card) = selection.choices.get(idx) {
            for (player, mut stats, mut inv) in players.iter_mut() {
                if player.id == loser {
                    crate::cards::apply(card.id, &mut stats);
                    inv.cards.push(card.id);
                }
            }
        }
        selection.loser = None;
        selection.choices.clear();
        next_state.set(GameState::InGame);
    }
}
