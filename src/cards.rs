use crate::components::Stats;
use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardId {
    Power,
    Speed,
    Jump,
    Poison,
    Slow,
}

#[derive(Clone, Copy)]
pub struct Card {
    pub id: CardId,
    pub name: &'static str,
    pub description: &'static str,
}

pub const ALL_CARDS: &[Card] = &[
    Card {
        id: CardId::Power,
        name: "Power",
        description: "Increase damage",
    },
    Card {
        id: CardId::Speed,
        name: "Speed",
        description: "Increase move speed",
    },
    Card {
        id: CardId::Jump,
        name: "Jump",
        description: "Increase jump force",
    },
    Card {
        id: CardId::Poison,
        name: "Poison",
        description: "Projectiles apply poison",
    },
    Card {
        id: CardId::Slow,
        name: "Frost",
        description: "Projectiles slow enemies",
    },
];

pub fn random_choices(n: usize) -> Vec<Card> {
    let mut cards = ALL_CARDS.to_vec();
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);
    cards.into_iter().take(n).collect()
}

pub fn apply(card: CardId, stats: &mut Stats) {
    match card {
        CardId::Power => stats.damage *= 1.2,
        CardId::Speed => stats.move_speed *= 1.2,
        CardId::Jump => stats.jump_force *= 1.2,
        CardId::Poison => stats.poison_damage += 5.0,
        CardId::Slow => stats.slow_amount = 0.5,
    }
}
