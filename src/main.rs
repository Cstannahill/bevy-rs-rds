use bevy::prelude::*;
use bevy::render::{
    settings::{Backends, WgpuSettings},
    RenderPlugin,
};
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
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Magic Duel".into(),
                        resolution: (800., 600.).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    },
                }),
        )
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .init_resource::<RoundManager>()
        .init_resource::<CardSelection>()
        .add_state::<GameState>()
        .add_event::<PlayerKilled>()
        .add_systems(Startup, (systems::setup, systems::setup_hud))
        .add_systems(OnEnter(GameState::CardSelection), systems::setup_card_ui)
        .add_systems(OnExit(GameState::CardSelection), systems::cleanup_card_ui)
        .add_systems(OnEnter(GameState::GameOver), systems::setup_game_over)
        .add_systems(OnExit(GameState::GameOver), systems::cleanup_game_over)
        .add_systems(
            Update,
            (
                systems::player_input,
                systems::update_cooldowns,
                systems::projectile_cleanup,
                systems::lifetime_system,
                systems::poison_damage_system,
                systems::slow_system,
                systems::projectile_player_collision,
                systems::round_manager,
                systems::update_hud,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            (systems::card_input_system, systems::card_click_system)
                .run_if(in_state(GameState::CardSelection)),
        )
        .add_systems(
            Update,
            (systems::game_over_input).run_if(in_state(GameState::GameOver)),
        )
        .run();
}
