use std::io::Cursor;
#[cfg(feature = "debug")]
use std::time::Duration;

#[cfg(feature = "debug")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowTheme};
use bevy::winit::WinitWindows;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use winit::window::Icon;

#[cfg(feature = "debug")]
use crate::components::Coordinates;
use crate::plugins::BoardPlugin;
use crate::resources::BoardOptions;
use crate::systems::{make_window_visible_after_startup, setup_2d_camera, toggle_vsync};

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
            // This will spawn an invisible window
            // The window will be made visible in the make_visible() system after 3 frames.
            // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
            visible: false,
            ..default()
        }),
        ..default()
    }));

    #[cfg(feature = "debug")]
    add_debug_plugins(&mut app);

    #[cfg(feature = "debug")]
    register_custom_types_for_bevy_inspector_egui(&mut app);

    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        mine_count: 40,
        tile_padding: 3.0,
        ..default()
    });

    app.add_plugins(BoardPlugin);

    app.add_systems(Startup, set_window_icon);
    app.add_systems(Startup, setup_2d_camera);
    app.add_systems(Update, make_window_visible_after_startup);
    app.add_systems(Update, toggle_vsync);

    app.run();
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    info!("Setting the window icon");
    let primary_entity = primary_window.single();

    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };

    let icon_buf = Cursor::new(include_bytes!("../icons/Bomb-Icon-256.png"));

    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
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
