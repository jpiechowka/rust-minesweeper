use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::resources::Board;

pub fn handle_mouse_input(
    window_query: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mut button_event_reader: EventReader<MouseButtonInput>,
) {
    let window = window_query.single();

    for event in button_event_reader.read() {
        if let ButtonState::Pressed = event.state {
            if let Some(click_position) = window.cursor_position() {
                if let Some(tile_coordinates) = board.mouse_position(window, click_position) {
                    match event.button {
                        MouseButton::Left => {
                            info!(
                                "LMB clicked, trying to uncover tile on {}",
                                tile_coordinates
                            );
                        }
                        MouseButton::Right => {
                            info!("RMB clicked, trying to mark tile on {}", tile_coordinates);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
