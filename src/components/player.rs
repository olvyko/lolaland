use amethyst::ecs::prelude::{Component, VecStorage};

// Component represents the shape of an entity
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Player {
    pub width: f64,
    pub height: f64,
    pub velocity: f64,
}

impl Player {
    pub fn new(width: f64, height: f64, velocity: f64) -> Player {
        Player {
            width,
            height,
            velocity,
        }
    }
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
