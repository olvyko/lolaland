use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use specs_physics::PhysicsBody;

use crate::components::Player;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, PhysicsBody<f32>>,
    );

    fn run(&mut self, (input, players, mut bodies): Self::SystemData) {
        // if pos == neg { 0.0 } else if pos { 1.0 } else { -1.0 }
        let x_move = input.axis_value("x_move").unwrap();
        //let y_move = input.axis_value("y_move").unwrap();

        for (player, body) in (&players, &mut bodies).join() {
            body.velocity.linear.x = x_move * player.velocity;
            //body.velocity.linear.y = y_move * player.velocity;
        }
    }
}
