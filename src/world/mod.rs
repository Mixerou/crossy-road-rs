use std::collections::VecDeque;

use bevy::app::{App, Plugin, Update};
use bevy::math::IVec2;
#[cfg(feature = "debug")]
use bevy::prelude::EventReader;
use bevy::prelude::{
    in_state, Entity, EventWriter, IntoSystemConfigs, NextState, OnEnter, Query, Res, ResMut,
    Resource, State, ViewVisibility,
};
use bevy::utils::HashMap;
use oorandom::Rand32;

#[cfg(feature = "debug")]
use crate::events::DevRequestBiome;
use crate::events::{RequestNewChunkSpawning, RequestOldChunkDespawning};
use crate::player::CurrentCharacter;
use crate::states::{AppState, CurrentBiome};
use crate::world::biomes::crossy_valley::CrossyValleyBiome;
#[cfg(feature = "debug")]
use crate::world::biomes::default::DefaultBiome;

mod biomes;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.add_plugins(DefaultBiome);

        app.init_resource::<Map>()
            .add_plugins(CrossyValleyBiome)
            .add_systems(OnEnter(AppState::InitialisingWorld), init_world)
            .add_systems(
                Update,
                check_chunk_visibilities.run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Clone)]
pub struct Chunk {
    pub position_x: i32,
    pub entities: Vec<Entity>,
}

#[derive(Clone, Resource)]
pub struct Map {
    pub random_generator: Rand32,
    pub chunks: VecDeque<Chunk>,
    pub obstacles_xz: HashMap<IVec2, Entity>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            random_generator: Rand32::new(0),
            chunks: Default::default(),
            obstacles_xz: Default::default(),
        }
    }
}

fn init_world(
    #[cfg(feature = "debug")] mut biome_dev_requests: EventReader<DevRequestBiome>,
    mut chunk_generation_requester: EventWriter<RequestNewChunkSpawning>,
    mut app_state: ResMut<NextState<AppState>>,
    mut current_biome_setter: ResMut<NextState<CurrentBiome>>,
    mut map: ResMut<Map>,
    current_biome: Res<State<CurrentBiome>>,
    current_character: Res<CurrentCharacter>,
) {
    if current_biome.get().eq(&CurrentBiome::None) {
        #[cfg(feature = "debug")]
        match biome_dev_requests.read().next() {
            Some(biome) if biome.get().ne(&CurrentBiome::None) => {
                current_biome_setter.set(biome.get())
            }
            _ => current_biome_setter.set(current_character.get().biome),
        };

        #[cfg(not(feature = "debug"))]
        current_biome_setter.set(current_character.get().biome);
    }

    map.random_generator = Rand32::new(0);

    for _ in 0..16 {
        chunk_generation_requester.send(RequestNewChunkSpawning);
    }

    app_state.set(AppState::Playing);
}

fn check_chunk_visibilities(
    mut new_chunk_spawning_requester: EventWriter<RequestNewChunkSpawning>,
    mut old_chunk_despawning_requester: EventWriter<RequestOldChunkDespawning>,
    map: ResMut<Map>,
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
            new_chunk_spawning_requester.send(RequestNewChunkSpawning);
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

        old_chunk_despawning_requester.send(RequestOldChunkDespawning);
    }
}
