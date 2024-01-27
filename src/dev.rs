use bevy::app::App;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::Plugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DevelopmentPlugin;

impl Plugin for DevelopmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            WorldInspectorPlugin::new(),
        ));
    }
}
