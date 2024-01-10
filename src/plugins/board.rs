use bevy::prelude::*;

use crate::resources::TileMap;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);
        info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board() {
        let mut tile_map = TileMap::new_empty(20, 20);
        tile_map.set_mines(40);

        #[cfg(feature = "debug")]
        info!("{}", tile_map.console_output());
    }
}
