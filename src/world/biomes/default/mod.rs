//! This is the bare minimum required to create a biome.

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    in_state, Commands, EventReader, IntoSystemConfigs, Mut, OnEnter, OnExit, Res, ResMut,
    Resource, World,
};
use bevy::utils::HashMap;

use crate::constants::MAP_MIN_X;
use crate::events::{RequestNewChunkSpawning, RequestOldChunkDespawning};
use crate::states::CurrentBiome;
use crate::world::biomes::default::regions::meadow::GrassRegion;
use crate::world::biomes::default::regions::{meadow, Region};
use crate::world::biomes::StandardBiomeSystems;
use crate::world::Map;

mod regions;

const CURRENT_BIOME: CurrentBiome = CurrentBiome::Default;

pub struct DefaultBiome;

impl Plugin for DefaultBiome {
    /// We have [StandardBiomeSystems] `struct` that contains systems with frequently used logic.
    /// Nobody forbids using a system from the [StandardBiomeSystems] `struct`
    /// and from the current biome at the same time.
    ///
    ///
    /// We connect each region as a plugin because some of them may have unique systems
    /// for moving cars, for example.
    fn build(&self, app: &mut App) {
        app.add_plugins(GrassRegion)
            .add_systems(
                OnEnter(CURRENT_BIOME),
                (StandardBiomeSystems::enter_biome, Self::enter_biome),
            )
            .add_systems(
                Update,
                (
                    Self::spawn_new_chunk,
                    StandardBiomeSystems::despawn_old_chunk,
                )
                    .distributive_run_if(in_state(CURRENT_BIOME)),
            )
            .add_systems(
                OnExit(CURRENT_BIOME),
                (StandardBiomeSystems::leave_biome, Self::leave_biome),
            );
    }
}

/// Common systems for each biome, but some can be used from [StandardBiomeSystems] `struct`.
impl DefaultBiome {
    /// Should run when [CurrentBiome] enters {current biome variant}.
    /// This is where we initialise [BiomeData] and other stuff for the biome to work.
    fn enter_biome(world: &mut World) {
        let biome_data = BiomeData {
            regions: HashMap::new(),
            grass_region: meadow::SystemIds::register(world),
        };

        world.insert_resource(biome_data);
    }

    /// This system is triggered if a [RequestNewChunkSpawning] event is sent somewhere in the app.
    /// You must remember that there can be multiple events per update,
    /// and they must be handled all at once.
    fn spawn_new_chunk(
        mut commands: Commands,
        mut new_chunk_spawning_requests: EventReader<RequestNewChunkSpawning>,
        mut biome_data: ResMut<BiomeData>,
        map: Res<Map>,
    ) {
        let from_x = match map.chunks.is_empty() {
            true => MAP_MIN_X,
            false => match map.chunks.back() {
                Some(chunk) => chunk.position_x + 1,
                None => MAP_MIN_X,
            },
        };
        let range = from_x..(from_x + new_chunk_spawning_requests.len() as i32);

        for x in range {
            let region = biome_data.regions.entry(x).or_insert(Region::Meadow);

            match region {
                Region::Meadow => {
                    commands.run_system_with_input(biome_data.grass_region.spawn_ground, x);
                    commands.run_system_with_input(biome_data.grass_region.spawn_obstacles, x);
                }
            };

            biome_data.regions.remove(&x);
        }

        new_chunk_spawning_requests.clear();
    }

    /// This system is triggered
    /// if a [RequestOldChunkDespawning] event is sent somewhere in the app.
    /// You must remember that there can be multiple events per update,
    /// and they must be handled all at once.
    fn _despawn_old_chunk(
        mut _old_chunk_despawning_requests: EventReader<RequestOldChunkDespawning>,
    ) {
    }

    /// Should run when [CurrentBiome] enters {current biome variant}.
    /// This is where we cleanse the world of the current biome.
    fn leave_biome(world: &mut World) {
        world.resource_scope(|world, biome_data: Mut<BiomeData>| {
            biome_data.grass_region.unregister(world);
        });

        world.remove_resource::<BiomeData>();
    }
}

/// `BiomeData` structures can be used to store biome-specific data
#[derive(Resource)]
struct BiomeData {
    regions: HashMap<i32, Region>,
    grass_region: meadow::SystemIds,
}
