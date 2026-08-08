use macroquad::prelude::*;

use crate::display::DisplayContext;
use pentago_engine::game::{Placement, TileRotation};

use super::game_view::{board_origin, board_size, rotation_buttons_rect, tile_gap, tile_size};

pub fn hovered_placement(display: &DisplayContext) -> Option<Placement> {
    let (mouse_x, mouse_y) = mouse_position();

    let board = board_origin(display);
    let board_size = board_size(display);

    let local_x = mouse_x - board.x;
    let local_y = mouse_y - board.y;

    if local_x < 0.0 || local_y < 0.0 || local_x >= board_size || local_y >= board_size {
        return None;
    }

    let tile_size = tile_size(display);
    let tile_gap = tile_gap(display);
    let tile_stride = tile_size + tile_gap;

    let tile_column = (local_x / tile_stride) as usize;
    let tile_row = (local_y / tile_stride) as usize;

    if tile_column >= 2 || tile_row >= 2 {
        return None;
    }

    let x_in_tile = local_x - tile_column as f32 * tile_stride;

    let y_in_tile = local_y - tile_row as f32 * tile_stride;

    // Mouse is in the gap between tiles
    if x_in_tile >= tile_size || y_in_tile >= tile_size {
        return None;
    }

    let cell_size = tile_size / 3.0;

    let column = (x_in_tile / cell_size) as usize;
    let row = (y_in_tile / cell_size) as usize;

    Some(Placement {
        tile_row,
        tile_column,
        row,
        column,
    })
}

pub fn clicked_placement(display: &DisplayContext) -> Option<Placement> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    hovered_placement(display)
}

pub fn hovered_tile(display: &DisplayContext) -> Option<(usize, usize)> {
    let (mouse_x, mouse_y) = mouse_position();

    let board = board_origin(display);
    let board_size = board_size(display);

    let local_x = mouse_x - board.x;
    let local_y = mouse_y - board.y;

    if local_x < 0.0 || local_y < 0.0 || local_x >= board_size || local_y >= board_size {
        return None;
    }

    let tile_size = tile_size(display);
    let tile_gap = tile_gap(display);
    let tile_stride = tile_size + tile_gap;

    let tile_column = (local_x / tile_stride) as usize;
    let tile_row = (local_y / tile_stride) as usize;

    if tile_column >= 2 || tile_row >= 2 {
        return None;
    }

    let x_in_tile = local_x - tile_column as f32 * tile_stride;

    let y_in_tile = local_y - tile_row as f32 * tile_stride;

    // Ignore the gap between tiles
    if x_in_tile >= tile_size || y_in_tile >= tile_size {
        return None;
    }

    Some((tile_row, tile_column))
}

pub fn clicked_tile(display: &DisplayContext) -> Option<(usize, usize)> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    hovered_tile(display)
}

pub fn clicked_rotation(display: &DisplayContext) -> Option<TileRotation> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let mouse = vec2(mouse_x, mouse_y);

    let (counter_clockwise, clockwise) = rotation_buttons_rect(display);

    if counter_clockwise.contains(mouse) {
        Some(TileRotation::CounterClockwise)
    } else if clockwise.contains(mouse) {
        Some(TileRotation::Clockwise)
    } else {
        None
    }
}
