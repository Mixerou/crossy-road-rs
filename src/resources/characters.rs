use bevy::asset::{AssetServer, Assets};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Commands, Mesh, Res, ResMut, Resource};

use crate::resources::{AssetLoading, Model};
use crate::states::CurrentBiome;

#[derive(Clone, Debug)]
pub struct Character {
    pub biome: CurrentBiome,
    pub model: Model,
}

#[derive(Clone, Debug, Resource)]
pub struct CharacterCollection {
    pub chicken: Character,
}

impl CharacterCollection {
    pub(super) fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut asset_loading: ResMut<AssetLoading>,
        asset_server: Res<AssetServer>,
    ) {
        let collection = Self {
            chicken: Character {
                biome: CurrentBiome::CrossyValley,
                model: Model::load(
                    &mut materials,
                    &mut asset_loading,
                    &asset_server,
                    "characters/chicken",
                    "characters/chicken",
                ),
            },
        };

        commands.insert_resource(collection);
    }

    pub fn calculate_mesh_sizes(mut characters: ResMut<Self>, meshes: Res<Assets<Mesh>>) {
        characters.chicken.model.calculate_mesh_size(&meshes);
    }
}
