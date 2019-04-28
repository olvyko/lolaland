use amethyst::core::math::Vector3;
use amethyst::ecs::prelude::{Component, VecStorage};

// Simple movement component
#[derive(Debug, Clone)]
pub struct Velocity {
  pub velocity: Vector3<f64>,
}

impl Component for Velocity {
  type Storage = VecStorage<Self>;
}
