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
    pub marked_tiles: Vec<Coordinates>,
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
        if self.marked_tiles.contains(coordinates) {
            None
        } else {
            self.covered_tiles.get(coordinates)
        }
    }

    pub fn try_uncover_tile(&mut self, coordinates: &Coordinates) -> Option<Entity> {
        if self.marked_tiles.contains(coordinates) {
            self.unmark_tile(coordinates)?;
        }

        self.covered_tiles.remove(coordinates)
    }

    pub fn adjacent_covered_tiles(&self, coordinates: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coordinates)
            .filter_map(|coord| self.covered_tiles.get(&coord))
            .copied()
            .collect()
    }

    fn unmark_tile(&mut self, coords: &Coordinates) -> Option<Coordinates> {
        let pos = match self.marked_tiles.iter().position(|a| a == coords) {
            None => {
                error!("Failed to unmark tile at {}", coords);
                return None;
            }
            Some(p) => p,
        };
        Some(self.marked_tiles.remove(pos))
    }

    pub fn is_completed(&self) -> bool {
        self.tile_map.mine_count() as usize == self.covered_tiles.len()
    }

    pub fn try_toggle_mark(&mut self, coords: &Coordinates) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(coords)?;
        let mark = if self.marked_tiles.contains(coords) {
            self.unmark_tile(coords)?;
            false
        } else {
            self.marked_tiles.push(*coords);
            true
        };
        Some((entity, mark))
    }
}
