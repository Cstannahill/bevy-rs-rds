use bevy::prelude::*;

use crate::components::{Health, Player, Projectile};
use crate::resources::RoundManager;
use crate::states::GameState;

#[derive(Component)]
pub struct GameOverUiRoot;

pub fn setup_game_over(mut commands: Commands, manager: Res<RoundManager>) {
    let winner = if manager.p1_score > manager.p2_score { 1 } else { 2 };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            GameOverUiRoot,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Game Over! Player {winner} wins. Press R to restart."),
                TextStyle {
                    font_size: 32.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverUiRoot>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

pub fn game_over_input(
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut manager: ResMut<RoundManager>,
    mut players: Query<(&Player, &mut Transform, &mut Health)>,
    projectiles: Query<Entity, With<Projectile>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::R) {
        manager.p1_score = 0;
        manager.p2_score = 0;
        for entity in &projectiles {
            commands.entity(entity).despawn();
        }
        for (player, mut transform, mut health) in players.iter_mut() {
            health.current = health.max;
            transform.translation = if player.id == 1 {
                Vec3::new(-100.0, 0.0, 0.0)
            } else {
                Vec3::new(100.0, 0.0, 0.0)
            };
        }
        next_state.set(GameState::InGame);
    }
}
