use bevy::prelude::Event;

#[cfg(feature = "debug")]
use crate::states::CurrentBiome;

#[cfg(feature = "debug")]
#[derive(Event)]
pub struct DevRequestBiome(CurrentBiome);

#[cfg(feature = "debug")]
impl DevRequestBiome {
    pub fn new(biome: CurrentBiome) -> Self {
        Self(biome)
    }

    pub fn get(&self) -> CurrentBiome {
        self.0
    }
}

#[derive(Event)]
pub struct RequestNewChunkSpawning;

#[derive(Event)]
pub struct RequestOldChunkDespawning;
