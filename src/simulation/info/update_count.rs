use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct UpdateCountInfo {
    pub update_count: u64,
}

pub fn update_count_update(mut state: ResMut<UpdateCountInfo>) {
    state.update_count = state.update_count.wrapping_add(1);
}
