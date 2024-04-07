use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets, Handle, LoadState, UntypedHandle};
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    in_state, run_once, Color, Commands, Cuboid, IntoSystemConfigs, Mesh, NextState, Res, ResMut,
    Resource, State, States,
};

use crate::states::{AppState, CurrentBiome};
use crate::utils;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
enum InitModelsState {
    #[default]
    Loading,
    CalculatingMeshSizes,
    Finished,
}

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InitModelsState>()
            .init_resource::<AssetLoading>()
            .add_systems(
                Update,
                (
                    (
                        GroundCollection::setup,
                        ObjectCollection::setup,
                        CharacterCollection::setup,
                    )
                        .distributive_run_if(run_once()),
                    check_assets_ready.run_if(in_state(InitModelsState::Loading)),
                    (
                        (
                            ObjectCollection::calculate_mesh_sizes,
                            CharacterCollection::calculate_mesh_sizes,
                        ),
                        finish_mesh_size_calculations,
                    )
                        .chain()
                        .run_if(in_state(InitModelsState::CalculatingMeshSizes)),
                )
                    .chain()
                    .run_if(in_state(AppState::LoadingModels)),
            );
    }
}

#[derive(Default, Resource)]
pub struct AssetLoading {
    pub handles: Vec<UntypedHandle>,
}

fn check_assets_ready(
    mut commands: Commands,
    mut init_models_state: ResMut<NextState<InitModelsState>>,
    server: Res<AssetServer>,
    loading: Res<AssetLoading>,
) {
    for handle in &loading.handles {
        match server.load_state(handle.id()) {
            LoadState::Loaded => {}
            LoadState::Failed => panic!("Failed to load some assets"),
            _ => return,
        }
    }

    init_models_state.set(InitModelsState::CalculatingMeshSizes);
    commands.remove_resource::<AssetLoading>();
}

fn finish_mesh_size_calculations(
    mut init_models_state: ResMut<NextState<InitModelsState>>,
    mut app_state_setter: ResMut<NextState<AppState>>,
    app_state: Res<State<AppState>>,
) {
    init_models_state.set(InitModelsState::Finished);
    app_state_setter.set(app_state.get().next());
}

#[derive(Clone, Debug)]
pub struct Model {
    pub mesh: Handle<Mesh>,
    pub mesh_size: Vec3,
    pub material: Handle<StandardMaterial>,
}

impl Model {
    pub fn new(mesh: Handle<Mesh>, mesh_size: Vec3, material: Handle<StandardMaterial>) -> Self {
        Self {
            mesh,
            mesh_size,
            material,
        }
    }

    pub fn load(
        asset_server: &Res<AssetServer>,
        asset_loading: &mut ResMut<AssetLoading>,
        path: impl ToString,
    ) -> Self {
        let mesh = asset_server.load(format!("models/{}.glb#Mesh0/Primitive0", path.to_string()));
        let material = asset_server.load(format!("models/{}.glb#Material0", path.to_string()));

        asset_loading.handles.push(mesh.clone().untyped());
        asset_loading.handles.push(material.clone().untyped());

        Self {
            mesh,
            mesh_size: Vec3::NAN,
            material,
        }
    }

    pub fn load_multiple(
        asset_server: &Res<AssetServer>,
        asset_loading: &mut ResMut<AssetLoading>,
        path: impl ToString,
        quantity: u8,
    ) -> Vec<Self> {
        let mut models = vec![];

        for number in 0..quantity {
            let mesh = asset_server.load(format!(
                "models/{}.glb#Mesh{number}/Primitive0",
                path.to_string(),
            ));
            let material =
                asset_server.load(format!("models/{}.glb#Material{number}", path.to_string()));

            asset_loading.handles.push(mesh.clone().untyped());
            asset_loading.handles.push(material.clone().untyped());

            models.push(Self {
                mesh,
                mesh_size: Vec3::NAN,
                material,
            });
        }

        models
    }

    pub fn calculate_mesh_size(&mut self, meshes: &Res<Assets<Mesh>>) {
        let mesh = meshes
            .get(self.mesh.id())
            .expect("Failed to get Mesh from Model");
        self.mesh_size = utils::calculate_mesh_size(mesh);
    }
}

#[derive(Debug, Resource)]
pub struct GroundCollection {
    pub light_cube: Model,
    pub light_dimmed_block: Model,
    pub dark_cube: Model,
    pub dark_dimmed_cube: Model,
}

impl GroundCollection {
    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let cube = meshes.add(Cuboid::from_size(Vec3::splat(1.)));

        let light_green: StandardMaterial =
            Color::rgb(174. / 255., 224. / 255., 102. / 255.).into();
        let dark_green: StandardMaterial = Color::rgb(167. / 255., 217. / 255., 94. / 255.).into();
        let light_dimmed_green = StandardMaterial {
            metallic: 0.5,
            ..light_green.clone()
        };
        let dark_dimmed_green = StandardMaterial {
            metallic: 0.5,
            ..dark_green.clone()
        };

        let light_green = materials.add(light_green);
        let light_dimmed_green = materials.add(light_dimmed_green);
        let dark_green = materials.add(dark_green);
        let dark_dimmed_green = materials.add(dark_dimmed_green);

        let collection = Self {
            light_cube: Model::new(cube.clone(), Vec3::ONE, light_green),
            light_dimmed_block: Model::new(cube.clone(), Vec3::ONE, light_dimmed_green),
            dark_cube: Model::new(cube.clone(), Vec3::ONE, dark_green),
            dark_dimmed_cube: Model::new(cube, Vec3::ONE, dark_dimmed_green),
        };

        commands.insert_resource(collection);
    }
}

#[derive(Debug, Resource)]
pub struct ObjectCollection {
    pub boulder: Model,
    pub trees: Vec<Model>,
}

impl ObjectCollection {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut asset_loading: ResMut<AssetLoading>,
    ) {
        let collection = Self {
            boulder: Model::load(&asset_server, &mut asset_loading, "objects/boulder"),
            trees: Model::load_multiple(&asset_server, &mut asset_loading, "objects/trees", 4),
        };

        commands.insert_resource(collection);
    }

    pub fn calculate_mesh_sizes(mut objects: ResMut<Self>, meshes: Res<Assets<Mesh>>) {
        objects.boulder.calculate_mesh_size(&meshes);

        for model in objects.trees.iter_mut() {
            model.calculate_mesh_size(&meshes);
        }
    }
}

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
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut asset_loading: ResMut<AssetLoading>,
    ) {
        let collection = Self {
            chicken: Character {
                biome: CurrentBiome::CrossyValley,
                model: Model::load(&asset_server, &mut asset_loading, "characters/chicken"),
            },
        };

        commands.insert_resource(collection);
    }

    pub fn calculate_mesh_sizes(mut characters: ResMut<Self>, meshes: Res<Assets<Mesh>>) {
        characters.chicken.model.calculate_mesh_size(&meshes);
    }
}
