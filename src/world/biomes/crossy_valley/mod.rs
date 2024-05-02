use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    in_state, Commands, EventReader, IntoSystemConfigs, Mut, OnEnter, OnExit, Res, ResMut,
    Resource, World,
};
use bevy::utils::HashMap;

use crate::constants::{MAP_MIN_X, PLAYER_SPAWN_POINT};
use crate::events::RequestNewChunkSpawning;
use crate::states::CurrentBiome;
use crate::world::biomes::crossy_valley::regions::forest::GrassRegion;
use crate::world::biomes::crossy_valley::regions::spawn_point::SpawnPointRegion;
use crate::world::biomes::crossy_valley::regions::{forest, spawn_point, Region};
use crate::world::biomes::StandardBiomeSystems;
use crate::world::Map;

mod regions;

const CURRENT_BIOME: CurrentBiome = CurrentBiome::CrossyValley;

pub struct CrossyValleyBiome;

impl Plugin for CrossyValleyBiome {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpawnPointRegion, GrassRegion))
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

impl CrossyValleyBiome {
    fn enter_biome(world: &mut World) {
        let grass_region = forest::SystemIds::register(world);
        let mut biome_data = BiomeData {
            regions: HashMap::new(),
            spawn_point_region: spawn_point::SystemIds::register(world, grass_region.spawn_ground),
            grass_region,
        };

        (MAP_MIN_X..PLAYER_SPAWN_POINT.x as i32 + 2).for_each(|x| {
            biome_data.regions.insert(x, Region::SpawnPoint);
        });

        world.insert_resource(biome_data);
    }

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

        // Please don't ask why we first insert a value and then immediately remove it.
        // It's a trick for the future.
        for x in range {
            let region = biome_data.regions.entry(x).or_insert(Region::Forest);

            match region {
                Region::SpawnPoint => {
                    commands.run_system_with_input(biome_data.spawn_point_region.spawn_ground, x);
                    commands
                        .run_system_with_input(biome_data.spawn_point_region.spawn_obstacles, x);
                }
                Region::Forest => {
                    commands.run_system_with_input(biome_data.grass_region.spawn_ground, x);
                    commands.run_system_with_input(biome_data.grass_region.spawn_obstacles, x);
                }
            };

            biome_data.regions.remove(&x);
        }

        new_chunk_spawning_requests.clear();
    }

    fn leave_biome(world: &mut World) {
        world.resource_scope(|world, biome_data: Mut<BiomeData>| {
            biome_data.spawn_point_region.unregister(world);
            biome_data.grass_region.unregister(world);
        });

        world.remove_resource::<BiomeData>();
    }
}

#[derive(Resource)]
struct BiomeData {
    regions: HashMap<i32, Region>,
    spawn_point_region: spawn_point::SystemIds,
    grass_region: forest::SystemIds,
}
