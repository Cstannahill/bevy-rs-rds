use crate::components::{Health, Player, Projectile, Stats, Velocity};
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
        Velocity {
            linvel: Vec2::new(0.0, 300.0),
        },
    ));
}
