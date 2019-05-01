mod animation;

use amethyst::ecs::prelude::World;

pub use self::animation::{AnimationId, AnimationPrefabData};

// Add all resources needed at the start to the world
pub fn add_resources(_world: &mut World) {
    // world.add_resource(res: T)
}
