use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TileSize {
    Fixed(f32),
    WindowAdaptive { min: f32, max: f32 },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    Centered { offset: Vec3 },
    CustomPosition(Vec3),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Resource)]
pub struct BoardOptions {
    pub map_size: (u16, u16),
    pub mine_count: u16,
    pub position: BoardPosition,
    pub tile_size: TileSize,
    pub tile_padding: f32,
    pub safe_start_enabled: bool,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::WindowAdaptive {
            min: 10f32,
            max: 50f32,
        }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered { offset: Vec3::ZERO }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            mine_count: 30,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0f32,
            safe_start_enabled: false,
        }
    }
}
