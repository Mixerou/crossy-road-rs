use std::fmt;
use std::fmt::Display;

use bevy::prelude::States;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum AppState {
    #[default]
    LoadingModels,
    InitialisingWorld,
    Playing,
    // Must be used to destroy entities to restart the level
    Clearing,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum CurrentBiome {
    Default,
    // Also, for level restarting
    #[default]
    None,
}

#[cfg(feature = "debug")]
impl CurrentBiome {
    pub fn all_variant_names() -> Vec<(CurrentBiome, String)> {
        vec![
            (CurrentBiome::Default, "Default".into()),
            (CurrentBiome::None, "None".into()),
        ]
    }
}

#[cfg(feature = "debug")]
impl Display for CurrentBiome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let biome_name = match self {
            CurrentBiome::Default => "Default",
            CurrentBiome::None => "None",
        };

        write!(formatter, "{}", biome_name)
    }
}
