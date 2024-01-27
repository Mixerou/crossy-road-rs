#![windows_subsystem = "windows"]

#[macro_use]
extern crate log;

use bevy::app::{App, PluginGroup};
use bevy::prelude::{ClearColor, Color};
use bevy::window::{PresentMode, Window, WindowPlugin};
use bevy::DefaultPlugins;
use dotenv::dotenv;

use crate::camera::CameraPlugin;
#[cfg(feature = "debug")]
use crate::dev::DevelopmentPlugin;

mod camera;
#[cfg(feature = "debug")]
mod dev;

fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Starting Crossy Road");

    let mut app = App::new();
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            #[cfg(feature = "debug")]
            title: "Crossy Road (Development)".into(),
            #[cfg(not(feature = "debug"))]
            title: "Crossy Road".into(),
            resolution: (1280., 720.).into(),
            present_mode: PresentMode::Fifo,
            ..Default::default()
        }),
        ..Default::default()
    };

    // Default and other dependencies
    app.insert_resource(ClearColor(Color::AZURE))
        .add_plugins(DefaultPlugins.set(window_plugin));

    // Current crate
    app.add_plugins(CameraPlugin);

    // For development
    #[cfg(feature = "debug")]
    app.add_plugins(DevelopmentPlugin);

    app.run()
}
