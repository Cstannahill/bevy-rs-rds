use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

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
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .init_resource::<RoundManager>()
        .init_resource::<CardSelection>()
        .add_state::<GameState>()
        .add_event::<PlayerKilled>()
        .add_systems(Startup, (systems::setup, systems::setup_hud))
        .add_systems(
            Update,
            (
                systems::player_input,
                systems::update_cooldowns,
                systems::projectile_cleanup,
                systems::lifetime_system,
                systems::poison_damage_system,
                systems::projectile_player_collision,
                systems::round_manager,
                systems::card_input_system,
                systems::update_hud,
            ),
        )
        .run();
}
