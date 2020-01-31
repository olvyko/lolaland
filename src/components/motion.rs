use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Motion {
    pub velocity: Vector2<f32>,
    pub max_velocity: Vector2<f32>,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            velocity: Vector2::new(0.0, 0.0),
            max_velocity: Vector2::new(0.0, 0.0),
        }
    }
}
