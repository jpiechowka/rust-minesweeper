#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Mine,
    MineNeighbor(u8),
    Empty,
}

impl Tile {
    pub const fn is_a_mine(&self) -> bool {
        matches!(self, Self::Mine)
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Self::Mine => "*".bright_red(),
                Self::MineNeighbor(v) => match v {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => v.to_string().red(),
                },
                Self::Empty => " ".normal(),
            }
        )
    }
}
