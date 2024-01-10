use std::ops::{Deref, DerefMut};

use rand::{thread_rng, Rng};

use crate::components::Coordinates;
use crate::resources::tile::Tile;

/// An array of tuples representing the coordinates of the neighbors of a cell in a grid.
/// The array contains the coordinates of the neighbors in the following order:
/// 1. Bottom left
/// 2. Bottom
/// 3. Bottom right
/// 4. Left
/// 5. Right
/// 6. Top left
/// 7. Top
/// 8. Top right
///
/// The coordinates are represented as `(i8, i8)` tuples, where the first element is the x-coordinate
/// and the second element is the y-coordinate.
///
/// Each tuple represents the relative coordinates of the neighbor with respect to the current cell.
/// For example, the neighbor at index 0, `(-1, -1)`, has a relative x-coordinate of -1 (one step to the left)
/// and a relative y-coordinate of -1 (one step down).
const NEIGHBOR_COORDINATES: [(i8, i8); 8] = [
    (-1, -1), // Bottom left
    (0, -1),  // Bottom
    (1, -1),  // Bottom right
    (-1, 0),  // Left
    (1, 0),   // Right
    (-1, 1),  // Top left
    (0, 1),   // Top
    (1, 1),   // Top right
];

#[derive(Debug, Clone)]
pub struct TileMap {
    mine_count: u16,
    width: u16,
    height: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn new_empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .map(|_| (0..width).map(|_| Tile::Empty).collect())
            .collect();

        Self {
            mine_count: 0,
            width,
            height,
            map,
        }
    }

    pub fn set_mines(&mut self, mine_count: u16) {
        self.mine_count = mine_count;
        let mut remaining_mines = mine_count;
        let mut rng = thread_rng();

        while remaining_mines > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );

            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Mine;
                remaining_mines -= 1;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coordinates { x, y };
                if self.is_mine_at(coords) {
                    continue;
                }

                let count = self.mine_count_at(coords);
                if count == 0 {
                    continue;
                }

                let tile = &mut self[y as usize][x as usize];
                *tile = Tile::MineNeighbor(count)
            }
        }
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        NEIGHBOR_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    pub fn is_mine_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        }

        self.map[coordinates.y as usize][coordinates.x as usize].is_a_mine()
    }

    pub fn mine_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_mine_at(coordinates) {
            return 0;
        }

        self.safe_square_at(coordinates)
            .filter(|coord| self.is_mine_at(*coord))
            .count() as u8
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn mine_count(&self) -> u16 {
        self.mine_count
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer: String = format!(
            "Map ({}, {}) with {} mines:\n\n",
            self.width, self.height, self.mine_count
        );

        let line: String = (0..=(self.width + 1)).map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);

        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }

        format!("{}{}", buffer, line)
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
