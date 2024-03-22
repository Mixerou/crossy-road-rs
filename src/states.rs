use bevy::prelude::States;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum AppState {
    #[default]
    LoadingModels,
    InitialisingWorld,
    Playing,
}
