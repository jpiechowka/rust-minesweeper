use bevy::prelude::*;

use crate::components::{Coordinates, Mine, MineNeighbor, Uncover};
use crate::plugins::{BoardCompletedEvent, MineExplosionEvent, TileTriggerEvent};
use crate::resources::Board;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_event_reader: EventReader<TileTriggerEvent>,
) {
    // adopted
    for trigger_event in tile_trigger_event_reader.read() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.coordinates) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Mine>, Option<&MineNeighbor>)>,
    mut board_completed_event_writer: EventWriter<BoardCompletedEvent>,
    mut mine_explosion_event_writer: EventWriter<MineExplosionEvent>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn_recursive();

        let (coordinates, mine, mine_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };

        match board.try_uncover_tile(coordinates) {
            None => info!("Tried to uncover an already uncovered tile"),
            Some(e) => info!("Uncovered tile {} (entity: {:?})", coordinates, e),
        }

        if board.is_completed() {
            info!("Board completed!");
            board_completed_event_writer.send(BoardCompletedEvent);
        }

        if mine.is_some() {
            info!("Boom!");
            mine_explosion_event_writer.send(MineExplosionEvent);
        } else if mine_counter.is_none() {
            for entity in board.adjacent_covered_tiles(*coordinates) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}
