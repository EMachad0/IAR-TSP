use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    Simulating,
}

pub fn transition_to_simulating(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Simulating));
}
