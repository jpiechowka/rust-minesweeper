use std::io::Cursor;
#[cfg(feature = "debug")]
use std::time::Duration;

#[cfg(feature = "debug")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
#[cfg(feature = "debug")]
use bevy::prelude::Reflect;
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowTheme};
use bevy::winit::WinitWindows;
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use resources::{BoardAssets, SpriteMaterial};
use winit::window::Icon;

#[cfg(feature = "debug")]
use crate::components::{Coordinates, Mine, MineNeighbor, Uncover};
use crate::plugins::BoardPlugin;
use crate::resources::BoardOptions;
use crate::systems::{make_window_visible_after_startup, setup_2d_camera, toggle_vsync};

mod components;
mod plugins;
mod resources;
mod systems;

const WINDOW_TITLE: &str = "Rust Minesweeper";
const INITIAL_RESOLUTION_X: u16 = 800;
const INITIAL_RESOLUTION_Y: u16 = 800;

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    Out,
}

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

    app.add_state::<AppState>();
    app.add_plugins(BoardPlugin {
        running_state: AppState::InGame,
    });

    app.add_systems(Startup, (set_window_icon, setup_2d_camera, setup_board));
    app.add_systems(
        Update,
        (
            make_window_visible_after_startup,
            toggle_vsync,
            state_handler,
        ),
    );

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
    app.register_type::<Mine>();
    app.register_type::<MineNeighbor>();
    app.register_type::<Uncover>();
}

fn state_handler(
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::C) {
        info!("[C] key pressed. Attempting to clear the board");
        if current_state.get() == &AppState::InGame {
            info!("Clearing the board");
            next_state.set(AppState::Out);
        } else {
            warn!("Wrong state detected. Game was already cleared before. Press 'R' to regenerate the board")
        }
    }

    if keys.just_pressed(KeyCode::R) {
        info!("[R] key pressed. Attempting to regenerate the board");
        if current_state.get() == &AppState::Out {
            info!("Regenerating the game board");
            next_state.set(AppState::InGame);
        } else {
            warn!("Wrong state detected. Clear the board with 'C' before regenerating the board")
        }
    }
}

fn setup_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("Setting up the board");

    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        mine_count: 60,
        tile_padding: 3.0,
        safe_start_enabled: true,
        ..default()
    });

    commands.insert_resource(BoardAssets {
        label: "Default".into(),
        board_material: SpriteMaterial {
            color: Color::BLACK,
            ..default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..default()
        },
        mine_counter_colors: BoardAssets::default_colors(),
        mine_counter_font: asset_server.load("fonts/symtext/Symtext.ttf"),
        flag_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/Flag.png"),
        },
        mine_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/Mine.png"),
        },
    });

    info!("Board has been configured");
}
