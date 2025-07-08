use bevy::prelude::*;
use crate::components::{Health, Player};
use crate::resources::RoundManager;

#[derive(Component)]
pub struct HealthText {
    pub player_id: usize,
}

#[derive(Component)]
pub struct ScoreText;

pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "P1: 0",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        }),
        HealthText { player_id: 1 },
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "P2: 0",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        }),
        HealthText { player_id: 2 },
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(40.0),
            ..default()
        }),
        ScoreText,
    ));
}

pub fn update_hud(
    mut health_texts: Query<(&HealthText, &mut Text)>,
    mut score_text: Query<&mut Text, With<ScoreText>>,
    players: Query<(&Player, &Health)>,
    manager: Res<RoundManager>,
) {
    for (marker, mut text) in &mut health_texts {
        for (player, health) in &players {
            if player.id == marker.player_id {
                text.sections[0].value = format!("P{}: {:.0}", player.id, health.current);
                break;
            }
        }
    }
    if let Ok(mut text) = score_text.get_single_mut() {
        text.sections[0].value = format!("Score {} - {}", manager.p1_score, manager.p2_score);
    }
}
