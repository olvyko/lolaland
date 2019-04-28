use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::prelude::DispatcherBuilder;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
  fn build(self, _: &mut DispatcherBuilder<'a, 'b>) -> Result<(), amethyst::Error> {
    Ok(())
  }
}
