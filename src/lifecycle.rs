use bevy::app::{App, Plugin};
use bevy::prelude::{in_state, IntoSystemConfigs, NextState, OnEnter, ResMut};

use crate::states::{AppState, CurrentBiome};

pub struct LifecyclePlugin;

impl Plugin for LifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(CurrentBiome::None),
            handle_none_biome.run_if(in_state(AppState::Playing)),
        )
        .add_systems(OnEnter(AppState::Clearing), clear);
    }
}

fn handle_none_biome(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Clearing);
}

fn clear(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InitialisingWorld);
}
