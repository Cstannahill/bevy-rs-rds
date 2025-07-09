use bevy::prelude::*;

use crate::cards;
use crate::components::{Inventory, Player, Stats};
use crate::resources::CardSelection;
use crate::states::GameState;

#[derive(Component)]
pub struct CardUiRoot;

#[derive(Component)]
pub struct CardButton {
    pub index: usize,
}

pub fn setup_card_ui(mut commands: Commands, selection: Res<CardSelection>) {
    // root full screen node
    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            CardUiRoot,
        ))
        .id();

    // Display which player is selecting cards
    let player_id = selection.loser.unwrap_or(0);
    commands.entity(root).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    format!("Player {player_id} - choose a card"),
                    TextStyle {
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
    });

    commands.entity(root).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|row| {
                for (i, card) in selection.choices.iter().enumerate() {
                    row
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(180.0),
                                    height: Val::Px(120.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: Color::DARK_GRAY.into(),
                                ..default()
                            },
                            CardButton { index: i },
                        ))
                        .with_children(|p| {
                            p.spawn(TextBundle::from_sections([
                                TextSection::new(
                                    card.name,
                                    TextStyle { font_size: 24.0, color: Color::WHITE, ..default() },
                                ),
                                TextSection::new(
                                    format!("\n{}", card.description),
                                    TextStyle { font_size: 16.0, color: Color::WHITE, ..default() },
                                ),
                            ]));
                        });
                }
            });
    });
}

pub fn cleanup_card_ui(mut commands: Commands, query: Query<Entity, With<CardUiRoot>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

pub fn card_click_system(
    mut interactions: Query<(&Interaction, &CardButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut selection: ResMut<CardSelection>,
    mut players: Query<(&Player, &mut Stats, &mut Inventory)>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction == Interaction::Pressed {
            if let Some(card) = selection.choices.get(button.index) {
                if let Some(loser) = selection.loser {
                    for (player, mut stats, mut inv) in players.iter_mut() {
                        if player.id == loser {
                            cards::apply(card.id, &mut stats);
                            inv.cards.push(card.id);
                        }
                    }
                }
            }
            selection.loser = None;
            selection.choices.clear();
            next_state.set(GameState::InGame);
            break;
        }
    }
}
