use crate::components::{Health, Player, Projectile, Stats, Velocity};
use crate::components::Lifetime;
use crate::events::PlayerKilled;
use crate::resources::RoundManager;
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
        },
        Velocity::default(),
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
        },
        Velocity::default(),
    ));
}

pub fn player_input(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(&Player, &Stats, &Transform, &mut Velocity)>,
) {
    for (player, stats, transform, mut velocity) in query.iter_mut() {
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
                if keyboard.just_pressed(KeyCode::Space) {
                    spawn_projectile(&mut commands, player.id, transform);
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
                if keyboard.just_pressed(KeyCode::Return) {
                    spawn_projectile(&mut commands, player.id, transform);
                }
            }
            _ => {}
        }
        velocity.linvel.x = direction * stats.move_speed;
    }
}

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.linvel.y += GRAVITY * time.delta_seconds();
        transform.translation.x += velocity.linvel.x * time.delta_seconds();
        transform.translation.y += velocity.linvel.y * time.delta_seconds();
        if transform.translation.y < 0.0 {
            // simple ground
            transform.translation.y = 0.0;
            velocity.linvel.y = 0.0;
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
        for (player_entity, player, mut health, player_transform) in players.iter_mut() {
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
                    kill_writer.send(PlayerKilled { winner: projectile.owner });
                }
                break;
            }
        }
    }
}

pub fn round_manager(
    mut commands: Commands,
    mut manager: ResMut<RoundManager>,
    mut reader: EventReader<PlayerKilled>,
    mut players: Query<(&Player, &mut Health, &mut Transform)>,
    projectiles: Query<Entity, With<Projectile>>,
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

        info!(
            "Scores - P1: {} P2: {}",
            manager.p1_score, manager.p2_score
        );

        if manager.p1_score >= manager.rounds_to_win || manager.p2_score >= manager.rounds_to_win {
            info!("Game Over");
        }
    }
}

fn spawn_projectile(commands: &mut Commands, owner: usize, transform: &Transform) {
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
            damage: 10.0,
        },
        Lifetime { time_left: 2.0 },
        Velocity {
            linvel: Vec2::new(0.0, 300.0),
        },
    ));
}
