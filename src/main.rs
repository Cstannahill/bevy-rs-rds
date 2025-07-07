use bevy::prelude::*;

mod components;
mod systems;

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
        .add_systems(Startup, systems::setup)
        .add_systems(
            Update,
            (
                systems::player_input,
                systems::apply_velocity,
                systems::projectile_cleanup,
            ),
        )
        .run();
}
