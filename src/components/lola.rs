use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum LolaState {
    Idling,
    Jumping,
    Running,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Lola {
    pub state: LolaState,
    pub max_ground_speed: f32,
    pub max_air_speed: f32,
}

impl Lola {
    pub fn new() -> Lola {
        Lola {
            state: LolaState::Idling,
            max_ground_speed: 10.0,
            max_air_speed: 20.0,
        }
    }
}
