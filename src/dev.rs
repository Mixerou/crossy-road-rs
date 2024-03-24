use std::time::{Duration, Instant};

use bevy::app::{App, Update};
use bevy::diagnostic::{Diagnostic, DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::SystemState;
use bevy::math::Vec3;
use bevy::prelude::{Local, NextState, Plugin, Res, ResMut, State, Transform, With, World};
use bevy::window::{PresentMode, PrimaryWindow};
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::bevy_inspector::{
    ui_for_all_assets, ui_for_resources, ui_for_world_entities,
};
use bevy_inspector_egui::egui::{Button, CollapsingHeader, ComboBox, ScrollArea, Window};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_rapier3d::render::{DebugRenderContext, RapierDebugRenderPlugin};

use crate::player::Player;
use crate::states::CurrentBiome;
use crate::world::Map;

const GAP_BETWEEN_SECTIONS: f32 = 4.;

pub struct DevelopmentPlugin;

impl Plugin for DevelopmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            EguiPlugin,
            DefaultInspectorConfigPlugin,
            RapierDebugRenderPlugin::default().disabled(),
        ))
        .add_systems(Update, update_ui);
    }
}

struct UiContext {
    show_world_inspector: bool,
    fps: String,
    frame_time: String,
    frame_time_avg: String,
    last_diagnostics_read_at: Instant,
}

impl Default for UiContext {
    fn default() -> Self {
        Self {
            show_world_inspector: false,
            fps: "Measuring...".into(),
            frame_time: "Measuring...".into(),
            frame_time_avg: "Measuring...".into(),
            last_diagnostics_read_at: Instant::now(),
        }
    }
}

fn parse_diagnostic(
    diagnostic: Option<&Diagnostic>,
    is_average: bool,
    digits_after_dot: usize,
    postfix: String,
) -> String {
    match diagnostic {
        Some(diagnostic) => {
            let value = match is_average {
                true => diagnostic.average(),
                false => diagnostic.value(),
            };

            match value {
                Some(value) => format!("{value:.digits_after_dot$}{postfix}"),
                None => "Measuring...".into(),
            }
        }
        None => "Cannot retrieve".into(),
    }
}

fn update_ui(
    world: &mut World,
    mut context: Local<UiContext>,
    params: &mut SystemState<(Res<State<CurrentBiome>>, ResMut<NextState<CurrentBiome>>)>,
) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    if context.last_diagnostics_read_at + Duration::from_millis(100) < Instant::now() {
        let diagnostics = world.resource::<DiagnosticsStore>();
        let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
        let frame_time = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME);

        context.fps = parse_diagnostic(fps, false, 0, "".into());
        context.frame_time = parse_diagnostic(frame_time, false, 1, "ms".into());
        context.frame_time_avg = parse_diagnostic(frame_time, true, 1, "ms".into());

        context.last_diagnostics_read_at = Instant::now();
    }

    Window::new("Development Menu")
        .title_bar(false)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            ui.label(format!("FPS: {}", context.fps));
            ui.label(format!(
                "Frame Time: {} (avg {})",
                context.frame_time, context.frame_time_avg
            ));

            let player_translation = match world
                .query_filtered::<&Transform, With<Player>>()
                .get_single(world)
            {
                Ok(transform) => transform.translation,
                Err(_) => Vec3::NAN,
            };
            ui.label(format!(
                "Player XYZ: {:.0} {:.0} {:.0}",
                player_translation.x, player_translation.y, player_translation.z,
            ));

            ui.add_space(GAP_BETWEEN_SECTIONS);

            ui.collapsing("World", |ui| {
                let chunks = match world.get_resource_ref::<Map>() {
                    Some(map) => map.chunks.len().to_string(),
                    None => "no map".into(),
                };
                ui.label(format!("Chunks Spawned: {}", chunks));

                ui.horizontal(|ui| {
                    ui.label("Current Biome");

                    let (current_biome, mut current_biome_setter) = params.get_mut(world);
                    ComboBox::from_label("")
                        .selected_text(current_biome.to_string())
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);

                            for (biome, biome_name) in CurrentBiome::all_variant_names() {
                                let label =
                                    ui.selectable_label(&biome == current_biome.get(), biome_name);

                                if label.clicked() && &biome != current_biome.get() {
                                    current_biome_setter.set(biome);
                                }
                            }
                        });
                });
            });

            ui.add_space(GAP_BETWEEN_SECTIONS);

            let Ok(mut window) = world
                .query_filtered::<&mut bevy::prelude::Window, With<PrimaryWindow>>()
                .get_single_mut(world)
            else {
                return;
            };
            let vsync_button = ui.add(Button::new(
                match window.present_mode == PresentMode::Fifo {
                    true => "Disable VSync",
                    false => "Enable VSync",
                },
            ));
            if vsync_button.clicked() {
                window.present_mode = match window.present_mode == PresentMode::Fifo {
                    true => PresentMode::Immediate,
                    false => PresentMode::Fifo,
                };
            }

            let mut rapier_debug_context = world.resource_mut::<DebugRenderContext>();
            let world_inspector_button = ui.add(Button::new(match context.show_world_inspector {
                true => "Hide World Inspector",
                false => "Show World Inspector",
            }));
            if world_inspector_button.clicked() {
                context.show_world_inspector = !context.show_world_inspector;
            }

            let rapier_debug_button = ui.add(Button::new(match rapier_debug_context.enabled {
                true => "Disable Rapier Debugger",
                false => "Enable Rapier Debugger",
            }));
            if rapier_debug_button.clicked() {
                rapier_debug_context.enabled = !rapier_debug_context.enabled;
            }
        });

    if context.show_world_inspector {
        Window::new("World Inspector")
            .title_bar(false)
            .show(egui_context.get_mut(), |ui| {
                ScrollArea::both().show(ui, |ui| {
                    CollapsingHeader::new("Entities").show(ui, |ui| {
                        ui_for_world_entities(world, ui);
                    });
                    CollapsingHeader::new("Assets").show(ui, |ui| {
                        ui_for_all_assets(world, ui);
                    });
                    CollapsingHeader::new("Resources").show(ui, |ui| {
                        ui_for_resources(world, ui);
                    });
                });
            });
    }
}
