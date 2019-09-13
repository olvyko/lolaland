mod animation;

use amethyst::ecs::prelude::World;

use specs_physics::{nalgebra::Vector2, parameters::Gravity};

pub use self::animation::{AnimationId, AnimationPrefabData};

// Add all resources needed at the start to the world
pub fn add_resources(world: &mut World) {
    world.add_resource(Gravity::<f32>(Vector2::<f32>::new(0.0, -9.8)));
}
