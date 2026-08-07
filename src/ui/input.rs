use macroquad::prelude::*;

use crate::game::{Placement, TileRotation};

use super::game_view::{BOARD_SIZE, TILE_GAP, TILE_SIZE, board_origin, rotation_buttons_rect};

pub fn clicked_placement() -> Option<Placement> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let origin = board_origin();

    let local_x = mouse_x - origin.x;
    let local_y = mouse_y - origin.y;

    if local_x < 0.0 || local_y < 0.0 || local_x >= BOARD_SIZE || local_y >= BOARD_SIZE {
        return None;
    }

    let tile_stride = TILE_SIZE + TILE_GAP;

    let tile_column = (local_x / tile_stride) as usize;
    let tile_row = (local_y / tile_stride) as usize;

    if tile_column >= 2 || tile_row >= 2 {
        return None;
    }

    let x_in_tile = local_x - tile_column as f32 * tile_stride;
    let y_in_tile = local_y - tile_row as f32 * tile_stride;

    // Ignore the gap between tiles
    if x_in_tile >= TILE_SIZE || y_in_tile >= TILE_SIZE {
        return None;
    }

    let cell_size = TILE_SIZE / 3.0;

    let column = (x_in_tile / cell_size) as usize;
    let row = (y_in_tile / cell_size) as usize;

    Some(Placement {
        tile_row,
        tile_column,
        row,
        column,
    })
}

pub fn clicked_tile() -> Option<(usize, usize)> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let origin = board_origin();

    let local_x = mouse_x - origin.x;
    let local_y = mouse_y - origin.y;

    if local_x < 0.0 || local_y < 0.0 || local_x >= BOARD_SIZE || local_y >= BOARD_SIZE {
        return None;
    }

    let tile_stride = TILE_SIZE + TILE_GAP;

    let tile_column = (local_x / tile_stride) as usize;
    let tile_row = (local_y / tile_stride) as usize;

    if tile_column >= 2 || tile_row >= 2 {
        return None;
    }

    let x_in_tile = local_x - tile_column as f32 * tile_stride;
    let y_in_tile = local_y - tile_row as f32 * tile_stride;

    // Ignore the gap between tiles
    if x_in_tile >= TILE_SIZE || y_in_tile >= TILE_SIZE {
        return None;
    }

    Some((tile_row, tile_column))
}

pub fn clicked_rotation() -> Option<TileRotation> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let mouse = vec2(mouse_x, mouse_y);

    let (counter_clockwise, clockwise) = rotation_buttons_rect();

    if counter_clockwise.contains(mouse) {
        Some(TileRotation::CounterClockwise)
    } else if clockwise.contains(mouse) {
        Some(TileRotation::Clockwise)
    } else {
        None
    }
}
