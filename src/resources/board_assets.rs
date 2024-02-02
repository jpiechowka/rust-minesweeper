use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

#[derive(Debug, Clone, Resource)]
pub struct BoardAssets {
    pub label: String,
    pub board_material: SpriteMaterial,
    pub tile_material: SpriteMaterial,
    pub covered_tile_material: SpriteMaterial,
    pub mine_counter_font: Handle<Font>,
    pub mine_counter_colors: Vec<Color>,
    pub flag_material: SpriteMaterial,
    pub mine_material: SpriteMaterial,
}

impl BoardAssets {
    pub fn default_colors() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::GREEN,
            Color::YELLOW,
            Color::ORANGE,
            Color::RED,
            Color::PURPLE,
        ]
    }

    pub fn mine_counter_color(&self, counter: u8) -> Color {
        let color_idx = counter.saturating_sub(1) as usize;
        match self.mine_counter_colors.get(color_idx) {
            Some(color) => *color,
            None => match self.mine_counter_colors.last() {
                Some(color) => *color,
                None => Color::WHITE,
            },
        }
    }
}
