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
}

impl Default for Lola {
    fn default() -> Self {
        Self {
            state: LolaState::Idling,
        }
    }
}
