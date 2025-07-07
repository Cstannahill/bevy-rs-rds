use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerKilled {
    pub winner: usize,
    pub loser: usize,
}
