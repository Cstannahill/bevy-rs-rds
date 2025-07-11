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
    pub projectile_speed: f32,
    pub shot_cooldown: f32,
    pub cooldown_timer: f32,
    pub poison_damage: f32,
    pub slow_amount: f32,
    pub aim_angle: f32,
}

#[derive(Component)]
pub struct Projectile {
    pub owner: usize,
    pub damage: f32,
}

#[derive(Component)]
pub struct Lifetime {
    pub time_left: f32,
}

#[derive(Component, Default)]
pub struct Inventory {
    pub cards: Vec<crate::cards::CardId>,
}

#[derive(Component)]
pub struct PoisonEffect {
    pub damage_per_second: f32,
    pub duration: f32,
}

#[derive(Component)]
pub struct Poisoned {
    pub damage_per_second: f32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct SlowEffect {
    pub amount: f32,
    pub duration: f32,
}

#[derive(Component)]
pub struct Slowed {
    pub amount: f32,
    pub timer: Timer,
}
