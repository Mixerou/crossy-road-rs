use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets, Handle, LoadState, UntypedHandle};
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    in_state, run_once, Commands, IntoSystemConfigs, Mesh, NextState, Res, ResMut, Resource, State,
    States,
};

use crate::resources::characters::CharacterCollection;
use crate::resources::grounds::GroundCollection;
use crate::resources::obstacles::ObstacleCollection;
use crate::states::AppState;
use crate::utils;

pub mod characters;
pub mod grounds;
pub mod obstacles;

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
                        CharacterCollection::setup,
                        GroundCollection::setup,
                        ObstacleCollection::setup,
                    )
                        .distributive_run_if(run_once()),
                    check_assets_ready.run_if(in_state(InitModelsState::Loading)),
                    (
                        (
                            CharacterCollection::calculate_mesh_sizes,
                            ObstacleCollection::calculate_mesh_sizes,
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
struct AssetLoading {
    handles: Vec<UntypedHandle>,
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

    fn load(
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_loading: &mut ResMut<AssetLoading>,
        asset_server: &Res<AssetServer>,
        model_path: &str,
        texture_path: &str,
    ) -> Self {
        let mesh = asset_server.load(format!("models/{model_path}.glb#Mesh0/Primitive0"));
        asset_loading.handles.push(mesh.clone_weak().untyped());

        let texture = asset_server.load(format!("textures/{texture_path}.png"));
        asset_loading.handles.push(texture.clone_weak().untyped());

        let material = StandardMaterial {
            base_color_texture: Some(texture),
            ..Default::default()
        };
        let material = materials.add(material);

        Self {
            mesh,
            mesh_size: Vec3::NAN,
            material,
        }
    }

    fn load_with_material(
        asset_loading: &mut ResMut<AssetLoading>,
        material: Handle<StandardMaterial>,
        asset_server: &Res<AssetServer>,
        path: &str,
    ) -> Self {
        let mesh = asset_server.load(format!("models/{path}.glb#Mesh0/Primitive0"));
        asset_loading.handles.push(mesh.clone_weak().untyped());

        Self {
            mesh,
            mesh_size: Vec3::NAN,
            material,
        }
    }

    pub fn calculate_mesh_size(&mut self, meshes: &Res<Assets<Mesh>>) {
        let mesh = meshes
            .get(self.mesh.id())
            .expect("Failed to get Mesh from Model");
        self.mesh_size = utils::calculate_mesh_size(mesh);
    }
}
