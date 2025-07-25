use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundManager {
    pub p1_score: u32,
    pub p2_score: u32,
    pub rounds_to_win: u32,
}

#[derive(Resource, Default)]
pub struct CardSelection {
    pub loser: Option<usize>,
    pub choices: Vec<crate::cards::Card>,
}

#[derive(Resource, Clone)]
pub struct GameAssets {
    pub player1: Handle<Image>,
    pub player2: Handle<Image>,
    pub projectile: Handle<Image>,
}

impl Default for RoundManager {
    fn default() -> Self {
        Self {
            p1_score: 0,
            p2_score: 0,
            rounds_to_win: 3,
        }
    }
}
