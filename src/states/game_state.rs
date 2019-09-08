use amethyst::{GameData, SimpleState, StateData};

use crate::components::register_components;
use crate::entities::init_entities;
use crate::resources::add_resources;

#[derive(Default)]
pub struct GameState;

impl<'a, 'b> SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<GameData>) {
        let StateData { world, .. } = state_data;

        register_components(world);
        add_resources(world);
        init_entities(world);
    }
}
