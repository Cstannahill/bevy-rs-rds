use bevy::prelude::*;

mod cards;
mod components;
mod events;
mod resources;
mod states;
mod systems;

use events::PlayerKilled;
use resources::{CardSelection, RoundManager};
use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Magic Duel".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<RoundManager>()
        .init_resource::<CardSelection>()
        .add_state::<GameState>()
        .add_event::<PlayerKilled>()
        .add_systems(Startup, systems::setup)
        .add_systems(
            Update,
            (
                systems::player_input,
                systems::apply_velocity,
                systems::projectile_cleanup,
                systems::lifetime_system,
                systems::projectile_player_collision,
                systems::round_manager,
                systems::card_input_system,
            ),
        )
        .run();
}
