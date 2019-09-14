use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::prelude::DispatcherBuilder;

use crate::systems::{AnimationSystem, CameraTransformSystem, MovementSystem};

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
        Ok(())
    }
}
