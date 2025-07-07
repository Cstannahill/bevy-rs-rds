use bevy::prelude::*;
use crate::components::{Player, Health, Stats};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(-100.0, 0.0, 0.0),
                sprite: Sprite { color: Color::BLUE, custom_size: Some(Vec2::splat(30.0)), ..default() },
                ..default()
            },
            Player { id: 1 },
            Health { current: 100.0, max: 100.0 },
            Stats { move_speed: 200.0, jump_force: 400.0, damage: 10.0 },
        ));
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(100.0, 0.0, 0.0),
                sprite: Sprite { color: Color::RED, custom_size: Some(Vec2::splat(30.0)), ..default() },
                ..default()
            },
            Player { id: 2 },
            Health { current: 100.0, max: 100.0 },
            Stats { move_speed: 200.0, jump_force: 400.0, damage: 10.0 },
        ));
}

pub fn player_input(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction = 0.0;
        match player.id {
            1 => {
                if keyboard.pressed(KeyCode::A) { direction -= 1.0; }
                if keyboard.pressed(KeyCode::D) { direction += 1.0; }
            }
            2 => {
                if keyboard.pressed(KeyCode::Left) { direction -= 1.0; }
                if keyboard.pressed(KeyCode::Right) { direction += 1.0; }
            }
            _ => {}
        }
        transform.translation.x += direction * 200.0 * time.delta_seconds();
    }
}
