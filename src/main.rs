use std::time::Duration;

#[cfg(feature = "debug")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::components::Coordinates;
use crate::plugins::BoardPlugin;
use crate::systems::setup_2d_camera;

mod components;
mod plugins;
mod resources;
mod systems;

const WINDOW_TITLE: &str = "Rust Minesweeper";
const INITIAL_RESOLUTION_X: u16 = 800;
const INITIAL_RESOLUTION_Y: u16 = 600;

fn main() {
    let mut app = App::new();

    // Set window properties
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: WINDOW_TITLE.into(),
            resolution: (INITIAL_RESOLUTION_X, INITIAL_RESOLUTION_Y).into(),
            present_mode: PresentMode::AutoVsync,
            window_theme: Some(WindowTheme::Dark),
            focused: true,
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: true,
                close: true,
                minimize: true,
            },
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }));

    app.add_plugins(BoardPlugin);

    #[cfg(feature = "debug")]
    add_debug_plugins(&mut app);

    #[cfg(feature = "debug")]
    register_custom_types_for_bevy_inspector_egui(&mut app);

    app.add_systems(Startup, setup_2d_camera);
    app.run();
}

#[cfg(feature = "debug")]
fn add_debug_plugins(app: &mut App) {
    info!("Debug mode plugins enabled");
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(LogDiagnosticsPlugin {
        wait_duration: Duration::from_secs(5),
        ..default()
    });
    app.add_plugins(WorldInspectorPlugin::new());
}

#[cfg(feature = "debug")]
fn register_custom_types_for_bevy_inspector_egui(app: &mut App) {
    info!("Registering custom types for Bevy Inspector EGUI");
    app.register_type::<Coordinates>();
}
