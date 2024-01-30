use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::components::Coordinates;
use crate::plugins::Bounds2;
use crate::resources::TileMap;

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub entity: Entity,
}

impl Board {
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2f32;

        if !self.bounds.is_in_bounds(position) {
            return None;
        }

        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            // https://bevyengine.org/learn/migration-guides/0.10-0.11/#consistent-screen-space-coordinates
            y: self.tile_map.height() - 1 - (coordinates.y / self.tile_size) as u16,
        })
    }

    pub fn tile_to_uncover(&self, coordinates: &Coordinates) -> Option<&Entity> {
        self.covered_tiles.get(coordinates)
    }

    pub fn try_uncover_tile(&mut self, coordinates: &Coordinates) -> Option<Entity> {
        self.covered_tiles.remove(coordinates)
    }

    pub fn adjacent_covered_tiles(&self, coordinates: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coordinates)
            .filter_map(|coord| self.covered_tiles.get(&coord))
            .copied()
            .collect()
    }
}
