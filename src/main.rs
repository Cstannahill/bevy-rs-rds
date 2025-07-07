use bevy::prelude::*;

mod components;
mod systems;
mod resources;
mod events;

use events::PlayerKilled;
use resources::RoundManager;

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
            ),
        )
        .run();
}
