#[cfg(feature = "debug")]
use std::fmt;

use bevy::prelude::States;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum AppState {
    #[default]
    LoadingModels,
    InsertingCurrentCharacter,
    InitialisingWorld,
    Playing,
    // Must be used to destroy entities to restart the level
    Clearing,
}

impl AppState {
    pub fn next(&self) -> Self {
        match self {
            Self::LoadingModels => Self::InsertingCurrentCharacter,
            Self::InsertingCurrentCharacter => Self::InitialisingWorld,
            Self::InitialisingWorld => Self::Playing,
            Self::Playing => Self::Clearing,
            Self::Clearing => Self::InitialisingWorld,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum CurrentBiome {
    #[cfg(feature = "debug")]
    Default,
    CrossyValley,
    // Also, for level restarting
    #[default]
    None,
}

#[cfg(feature = "debug")]
impl CurrentBiome {
    pub fn all_variant_names() -> Vec<(CurrentBiome, String)> {
        vec![
            (CurrentBiome::Default, "Default".into()),
            (CurrentBiome::CrossyValley, "Crossy Valley".into()),
            (CurrentBiome::None, "None".into()),
        ]
    }
}

#[cfg(feature = "debug")]
impl fmt::Display for CurrentBiome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let biome_name = match self {
            CurrentBiome::Default => "Default",
            CurrentBiome::CrossyValley => "Crossy Valley",
            CurrentBiome::None => "None",
        };

        write!(formatter, "{}", biome_name)
    }
}
