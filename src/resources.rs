use std::any::TypeId;

use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets, Handle, LoadState, UntypedHandle};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    in_state, run_once, Commands, IntoSystemConfigs, Mesh, NextState, Res, ResMut, Resource,
};

use crate::states::AppState;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLoading>().add_systems(
            Update,
            (setup.run_if(run_once()), check_assets_ready)
                .chain()
                .run_if(in_state(AppState::LoadingModels)),
        );
    }
}

#[derive(Default, Resource)]
pub struct AssetLoading {
    pub handles: Vec<UntypedHandle>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    asset_loading: ResMut<AssetLoading>,
) {
    commands.insert_resource(CharacterCollection::new(asset_server, asset_loading));
}

fn check_assets_ready(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
    loading: Res<AssetLoading>,
) {
    for handle in &loading.handles {
        match server.load_state(handle.id()) {
            LoadState::Loaded => {
                if handle.type_id() == TypeId::of::<StandardMaterial>() {
                    let Some(material) = materials.get_mut(handle.id()) else {
                        panic!("Failed to initialise some assets");
                    };

                    if material.diffuse_transmission < 0.125 {
                        material.diffuse_transmission = 0.125;
                    }
                }
            }
            LoadState::Failed => panic!("Failed to load some assets"),
            _ => return,
        }
    }

    next_state.set(AppState::Playing);
    commands.remove_resource::<AssetLoading>();
}

#[derive(Clone, Debug)]
pub struct Model {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl Model {
    pub fn new(
        asset_server: &Res<AssetServer>,
        asset_loading: &mut ResMut<AssetLoading>,
        path: impl ToString,
    ) -> Self {
        let mesh = asset_server.load(format!("models/{}.glb#Mesh0/Primitive0", path.to_string()));
        let material = asset_server.load(format!("models/{}.glb#Material0", path.to_string()));

        asset_loading.handles.push(mesh.clone().untyped());
        asset_loading.handles.push(material.clone().untyped());

        Self { mesh, material }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct CharacterCollection {
    pub chicken: Model,
}

impl CharacterCollection {
    pub fn new(asset_server: Res<AssetServer>, mut asset_loading: ResMut<AssetLoading>) -> Self {
        Self {
            chicken: Model::new(&asset_server, &mut asset_loading, "characters/chicken"),
        }
    }
}
