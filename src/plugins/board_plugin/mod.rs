pub use board_plugin::BoardPlugin;
pub use bounds::Bounds2;
pub use events::BoardCompletedEvent;
pub use events::MineExplosionEvent;
pub use events::TileMarkEvent;
pub use events::TileTriggerEvent;

mod board_plugin;
mod bounds;
mod events;
