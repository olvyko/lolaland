use amethyst::{
    core::{bundle::SystemBundle, Transform},
    ecs::prelude::DispatcherBuilder,
};

use specs_physics::register_physics_systems;

use crate::systems::DebugSystem;

#[derive(Default)]
pub struct PhysicsBundle {
    debug_lines: bool,
}

impl PhysicsBundle {
    /// Enables the `DebugSystem` which draws `DebugLines` around
    /// `PhysicsCollider` shapes.
    pub fn with_debug_lines(mut self) -> Self {
        self.debug_lines = true;
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for PhysicsBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), amethyst::Error> {
        register_physics_systems::<f32, Transform>(builder);

        if self.debug_lines {
            builder.add(DebugSystem::default(), "debug_system", &[]);
        }

        Ok(())
    }
}
