use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: usize,
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Stats {
    pub move_speed: f32,
    pub jump_force: f32,
    pub damage: f32,
}

#[derive(Component, Default)]
pub struct Velocity {
    pub linvel: Vec2,
}

#[derive(Component)]
pub struct Projectile {
    pub owner: usize,
    pub damage: f32,
}
