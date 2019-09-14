use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use specs_physics::PhysicsBody;

use crate::components::{Lola, Motion};

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Lola>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, PhysicsBody<f32>>,
    );

    fn run(&mut self, (input, lolas, motions, mut bodies): Self::SystemData) {
        let x_move = input.axis_value("x_move").unwrap();
        //let y_move = input.axis_value("y_move").unwrap();

        for (lola, motion, body) in (&lolas, &motions, &mut bodies).join() {
            body.velocity.linear.x = x_move * motion.velocity.x;
            //body.velocity.linear.y = y_move * player.velocity;
        }
    }
}
