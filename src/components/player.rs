use amethyst::ecs::prelude::{Component, VecStorage};

// Component represents the shape of an entity
#[derive(Debug, Clone)]
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub velocity: f32,
}

impl Player {
    pub fn new(width: f32, height: f32, velocity: f32) -> Player {
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
