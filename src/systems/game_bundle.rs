use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
};

use crate::systems::CameraTransformSystem;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        dispatcher.add(CameraTransformSystem, "camera_transform_system", &[]);
        Ok(())
    }
}
