use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use specs_physics::{
    colliders::Shape, PhysicsBody, PhysicsBodyBuilder, PhysicsCollider, PhysicsColliderBuilder,
};

use crate::components::Player;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (input, players, mut transforms): Self::SystemData) {
        // if pos == neg { 0.0 } else if pos { 1.0 } else { -1.0 }
        let x_move = input.axis_value("x_move").unwrap();
        let y_move = input.axis_value("y_move").unwrap();

        for (player, transform) in (&players, &mut transforms).join() {
            transform.prepend_translation_x(x_move * player.velocity);
            transform.prepend_translation_y(y_move * player.velocity);
        }
    }
}
