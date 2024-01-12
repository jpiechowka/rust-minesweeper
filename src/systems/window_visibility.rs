use bevy::core::FrameCount;
use bevy::prelude::*;

const FRAME_COUNT_TO_MAKE_WINDOW_VISIBLE: u32 = 3;

pub fn make_window_visible_after_startup(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == FRAME_COUNT_TO_MAKE_WINDOW_VISIBLE {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        info!(
            "Making window visible after {} frames",
            FRAME_COUNT_TO_MAKE_WINDOW_VISIBLE
        );
        window.single_mut().visible = true;
    }
}
