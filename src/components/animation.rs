use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::Entity,
    ecs::{Component, DenseVecStorage},
    error::Error,
    renderer::sprite::{prefab::SpriteScenePrefab, SpriteRender},
};

use serde::{Deserialize, Serialize};

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    Idle,
    Walk,
    Jump,
}

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct AnimationPrefabData {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Animation {
    pub current: AnimationId,
    pub types: Vec<AnimationId>,
    pub show: bool,
}

impl Animation {
    pub fn new(current: AnimationId, types: Vec<AnimationId>) -> Self {
        Self {
            current,
            types,
            show: true,
        }
    }
}
