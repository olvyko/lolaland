use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::prelude::DispatcherBuilder;

use crate::systems::*;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), amethyst::Error> {
        Ok(())
    }
}
