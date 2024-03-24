use std::collections::VecDeque;

use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{
    in_state, Commands, Entity, EventWriter, IntoSystemConfigs, NextState, OnEnter, Query, ResMut,
    Resource, ViewVisibility,
};

use crate::events::RequestChunkGeneration;
use crate::states::{AppState, CurrentBiome};
use crate::world::biomes::default::DefaultBiome;

mod biomes;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>()
            .add_plugins(DefaultBiome)
            .add_systems(OnEnter(AppState::InitialisingWorld), init_world)
            .add_systems(
                Update,
                check_chunk_visibilities.run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Clone)]
pub struct Chunk {
    pub position_x: isize,
    pub entities: Vec<Entity>,
}

#[derive(Clone, Default, Resource)]
pub struct Map {
    pub chunks: VecDeque<Chunk>,
}

fn init_world(
    mut chunk_generation_requester: EventWriter<RequestChunkGeneration>,
    mut app_state: ResMut<NextState<AppState>>,
    mut current_biome: ResMut<NextState<CurrentBiome>>,
) {
    current_biome.set(CurrentBiome::Default);

    for _ in 0..16 {
        chunk_generation_requester.send(RequestChunkGeneration);
    }

    app_state.set(AppState::Playing);
}

fn check_chunk_visibilities(
    mut commands: Commands,
    mut chunk_generation_requester: EventWriter<RequestChunkGeneration>,
    mut map: ResMut<Map>,
    entities: Query<&ViewVisibility>,
) {
    // Checks to create new chunks
    if let Some(chunk) = map.chunks.back() {
        if chunk
            .entities
            .iter()
            .any(|entity| match entities.get(*entity) {
                Ok(visibility) => visibility.get(),
                Err(_) => false,
            })
        {
            chunk_generation_requester.send(RequestChunkGeneration);
        }
    }

    // Checks to delete old chunks
    if let Some(chunk) = map.chunks.front() {
        if chunk
            .entities
            .iter()
            .any(|entity| match entities.get(*entity) {
                Ok(visibility) => visibility.get(),
                Err(_) => true,
            })
        {
            return;
        }

        for entity in &chunk.entities {
            if let Some(entity) = commands.get_entity(*entity) {
                entity.despawn_recursive();
            }
        }

        map.chunks.pop_front();
    }
}
