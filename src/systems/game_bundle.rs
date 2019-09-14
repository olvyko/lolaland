use amethyst::{
    assets::PrefabLoaderSystem, core::bundle::SystemBundle, ecs::prelude::DispatcherBuilder,
};

use crate::{
    components::AnimationPrefabData,
    systems::{AnimationSystem, CameraTransformSystem, MovementSystem},
};

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), amethyst::Error> {
        builder.add(MovementSystem, "movement_system", &["input_system"]);
        builder.add(
            AnimationSystem,
            "animation_system",
            &[
                "input_system",
                "animation_control_system",
                "sampler_interpolation_system",
            ],
        );
        builder.add(CameraTransformSystem, "camera_transform_system", &[]);
        builder.add(
            PrefabLoaderSystem::<AnimationPrefabData>::default(),
            "prefab_loader_system",
            &[],
        );
        Ok(())
    }
}
