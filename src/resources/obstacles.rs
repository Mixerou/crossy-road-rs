use bevy::asset::{AssetServer, Assets};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Commands, Mesh, Res, ResMut, Resource};
use oorandom::Rand32;

use crate::resources::{AssetLoading, Model};

#[derive(Debug, Resource)]
pub struct ObstacleCollection {
    pub boulder: Model,
    pub stump: Model,
    pub trees: TreeObstacles,
}

impl ObstacleCollection {
    pub(super) fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut asset_loading: ResMut<AssetLoading>,
        asset_server: Res<AssetServer>,
    ) {
        let xl_tree = Model::load(
            &mut materials,
            &mut asset_loading,
            &asset_server,
            "obstacles/trees/xl",
            "obstacles/trees",
        );
        let trees = TreeObstacles {
            small: Model::load_with_material(
                &mut asset_loading,
                xl_tree.material.clone(),
                &asset_server,
                "obstacles/trees/small",
            ),
            medium: Model::load_with_material(
                &mut asset_loading,
                xl_tree.material.clone(),
                &asset_server,
                "obstacles/trees/medium",
            ),
            large: Model::load_with_material(
                &mut asset_loading,
                xl_tree.material.clone(),
                &asset_server,
                "obstacles/trees/large",
            ),
            xl: xl_tree,
        };

        let collection = Self {
            boulder: Model::load(
                &mut materials,
                &mut asset_loading,
                &asset_server,
                "obstacles/boulder",
                "obstacles/boulder",
            ),
            stump: Model::load(
                &mut materials,
                &mut asset_loading,
                &asset_server,
                "obstacles/stump",
                "obstacles/stump",
            ),
            trees,
        };

        commands.insert_resource(collection);
    }

    pub fn calculate_mesh_sizes(mut obstacles: ResMut<Self>, meshes: Res<Assets<Mesh>>) {
        obstacles.boulder.calculate_mesh_size(&meshes);
        obstacles.stump.calculate_mesh_size(&meshes);

        obstacles.trees.small.calculate_mesh_size(&meshes);
        obstacles.trees.medium.calculate_mesh_size(&meshes);
        obstacles.trees.large.calculate_mesh_size(&meshes);
        obstacles.trees.xl.calculate_mesh_size(&meshes);
    }
}

#[derive(Debug)]
pub struct TreeObstacles {
    small: Model,
    medium: Model,
    large: Model,
    xl: Model,
}

impl TreeObstacles {
    pub fn get_random(&self, random_generator: &mut Rand32) -> &Model {
        match random_generator.rand_range(0..4) {
            0 => &self.small,
            1 => &self.medium,
            2 => &self.large,
            _ => &self.xl,
        }
    }
}
