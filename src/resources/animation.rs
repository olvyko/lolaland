use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::prelude::Entity,
    error::Error,
    renderer::{SpriteRender, sprite::{prefab::SpriteScenePrefab}},
};
use serde::*;

// Animation ids used in a AnimationSet
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    Idle,
    Walk,
    Jump,
}

// Loading data for one entity
#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct AnimationPrefabData {
    // Information for rendering a scene with sprites
    sprite_scene: SpriteScenePrefab,
    // Аll animations that can be run on the entity
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}
