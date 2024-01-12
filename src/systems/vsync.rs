use bevy::prelude::*;
use bevy::window::PresentMode;

/// This system toggles the vsync mode when pressing the button V.
/// You'll see FPS increase displayed in the console (if debug feature is enabled)
pub fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::V) {
        let mut window = windows.single_mut();

        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };

        info!(
            "[V] key pressed. Changing VSync mode to: {:?}",
            window.present_mode
        );
    }
}
