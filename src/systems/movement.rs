use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use specs_physics::{colliders::Shape, PhysicsBody, PhysicsBodyBuilder, PhysicsColliderBuilder};

use crate::components::Player;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PhysicsBody<f32>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (players, mut transforms, mut physics_bodies, input): Self::SystemData) {
        // if pos == neg { 0.0 } else if pos { 1.0 } else { -1.0 }
        let x_move = input.axis_value("x_move").unwrap();
        let y_move = input.axis_value("y_move").unwrap();

        for (player, transform, physics_body) in
            (&players, &mut transforms, &mut physics_bodies).join()
        {
            //info!("{:?}", physics_body.velocity.linear.x);

            transform.prepend_translation_x(x_move * player.velocity);
            //physics_body.velocity.linear.x = x_move as f32 * player.velocity as f32;
            transform.prepend_translation_y(y_move * player.velocity);
            //physics_body.velocity.linear.y = y_move as f32 * player.velocity as f32;
        }
    }
}
