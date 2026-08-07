use macroquad::prelude::*;

use crate::display::DisplayContext;
use crate::game::{CellState, Game, TurnState};

const BOARD_BACKGROUND: Color = Color::from_rgba(45, 30, 20, 255);

const TOP_UI_HEIGHT_REF: f32 = 105.0;

const BOARD_PADDING_REF: f32 = 14.0;
const BOARD_CORNER_RADIUS_REF: f32 = 24.0;

pub const TILE_SIZE_REF: f32 = 230.0;
pub const TILE_GAP_REF: f32 = 12.0;

const MARBLE_RATIO: f32 = 0.65;

const ROTATION_BUTTON_WIDTH_REF: f32 = 60.0;
const ROTATION_BUTTON_HEIGHT_REF: f32 = 25.0;
const ROTATION_BUTTON_MARGIN_REF: f32 = 24.0;
const ROTATION_BUTTON_GAP_REF: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameViewState {
    pub selected_tile: Option<(usize, usize)>,
}

impl Default for GameViewState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameViewState {
    pub fn new() -> Self {
        Self {
            selected_tile: None,
        }
    }
}

pub struct GameTextures {
    pub tile: Texture2D,
    pub white_marble: Texture2D,
    pub black_marble: Texture2D,
    pub rotation_arrow: Texture2D,
}

async fn load_texture_with_transparency(path: &str) -> Texture2D {
    let mut image = load_image(path).await.expect("Failed to load texture");

    for pixel in image.bytes.chunks_exact_mut(4) {
        let red = pixel[0];
        let green = pixel[1];
        let blue = pixel[2];

        if red == 255 && green == 0 && blue == 0 {
            pixel[3] = 0;
        }
    }

    Texture2D::from_image(&image)
}

impl GameTextures {
    pub async fn load() -> Self {
        Self {
            tile: load_texture_with_transparency("ui_assets/tile.png").await,
            white_marble: load_texture_with_transparency("ui_assets/sphere_white.png").await,
            black_marble: load_texture_with_transparency("ui_assets/sphere_black.png").await,
            rotation_arrow: load_texture_with_transparency("ui_assets/arrow.png").await,
        }
    }
}

fn draw_rounded_rectangle(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
    // horizontal
    draw_rectangle(x + radius, y, width - 2.0 * radius, height, color);

    // vertical
    draw_rectangle(x, y + radius, width, height - 2.0 * radius, color);

    // Corners
    draw_circle(x + radius, y + radius, radius, color);
    draw_circle(x + width - radius, y + radius, radius, color);
    draw_circle(x + radius, y + height - radius, radius, color);
    draw_circle(x + width - radius, y + height - radius, radius, color);
}

pub fn tile_size(display: &DisplayContext) -> f32 {
    TILE_SIZE_REF * display.scale
}

pub fn tile_gap(display: &DisplayContext) -> f32 {
    TILE_GAP_REF * display.scale
}

pub fn board_size(display: &DisplayContext) -> f32 {
    tile_size(display) * 2.0 + tile_gap(display)
}

pub fn board_origin(display: &DisplayContext) -> Vec2 {
    vec2(
        (display.width - board_size(display)) / 2.0,
        display.y(TOP_UI_HEIGHT_REF),
    )
}

fn draw_marble(
    game: &Game,
    textures: &GameTextures,
    tile_row: usize,
    tile_column: usize,
    pos_x: f32,
    pos_y: f32,
    display: &DisplayContext,
) {
    let tile_size = tile_size(display);
    let cell_size = tile_size / 3.0;
    let marble_size = cell_size * MARBLE_RATIO;

    for row in 0..3 {
        for column in 0..3 {
            let cell = game.board.tiles[tile_row][tile_column].cells[row][column];

            let texture = match cell.state {
                CellState::Empty => None,
                CellState::White => Some(&textures.white_marble),
                CellState::Black => Some(&textures.black_marble),
            };

            if let Some(texture) = texture {
                let marble_x = pos_x + column as f32 * cell_size + (cell_size - marble_size) / 2.0;
                let marble_y = pos_y + row as f32 * cell_size + (cell_size - marble_size) / 2.0;

                draw_texture_ex(
                    texture,
                    marble_x,
                    marble_y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(marble_size, marble_size)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

fn draw_selected_tile(tile_row: usize, tile_column: usize, board: Vec2, display: &DisplayContext) {
    let tile_size = tile_size(display);
    let tile_gap = tile_gap(display);

    let outline = 4.0 * display.scale;

    let x = board.x + tile_column as f32 * (tile_size + tile_gap);
    let y = board.y + tile_row as f32 * (tile_size + tile_gap);

    draw_rectangle_lines(
        x - outline,
        y - outline,
        tile_size + outline * 2.0,
        tile_size + outline * 2.0,
        outline,
        GOLD,
    );
}

pub fn rotation_buttons_rect(display: &DisplayContext) -> (Rect, Rect) {
    let board = board_origin(display);
    let board_size = board_size(display);

    let width = ROTATION_BUTTON_WIDTH_REF * display.scale;
    let height = ROTATION_BUTTON_HEIGHT_REF * display.scale;

    let margin = ROTATION_BUTTON_MARGIN_REF * display.scale;
    let gap = ROTATION_BUTTON_GAP_REF * display.scale;

    let total_height = height * 2.0 + gap;
    let x = board.x + board_size + margin;
    let start_y = board.y + (board_size - total_height) / 2.0;

    let counter_clockwise = Rect::new(x, start_y, width, height);
    let clockwise = Rect::new(x, start_y + height + gap, width, height);

    (counter_clockwise, clockwise)
}

fn draw_rotation_buttons(textures: &GameTextures, display: &DisplayContext) {
    let (left_button, right_button) = rotation_buttons_rect(display);

    draw_texture_ex(
        &textures.rotation_arrow,
        left_button.x,
        left_button.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(left_button.w, left_button.h)),
            flip_x: true,
            ..Default::default()
        },
    );

    draw_texture_ex(
        &textures.rotation_arrow,
        right_button.x,
        right_button.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(right_button.w, right_button.h)),
            ..Default::default()
        },
    );
}

fn draw_game_status(game: &Game, view_state: &GameViewState, display: &DisplayContext) {
    let player_text = match game.current_player {
        CellState::White => "White player's turn",
        CellState::Black => "Black player's turn",
        CellState::Empty => "",
    };

    let action_text = match game.state {
        TurnState::WaitingForPlacement => "Place a marble",
        TurnState::PlacementDone => "Press 'Enter' to confirm or click 'right-click' to cancel",
        TurnState::WaitingForRotation => {
            if view_state.selected_tile.is_some() {
                "Choose selected tile rotation direction"
            } else {
                "Select a tile to rotate"
            }
        }
        TurnState::RotationDone => "Press 'Enter' to confirm or click 'right-click' to cancel",
    };

    draw_text(
        player_text,
        display.x(30.0),
        display.y(35.0),
        40.0 * display.scale,
        WHITE,
    );

    draw_text(
        action_text,
        display.x(30.0),
        display.y(70.0),
        25.0 * display.scale,
        GRAY,
    );
}

pub fn draw_game(
    game: &Game,
    textures: &GameTextures,
    view_state: &GameViewState,
    display: &DisplayContext,
) {
    let board = board_origin(display);

    let tile_size = tile_size(display);
    let tile_gap = tile_gap(display);
    let board_size = board_size(display);

    let board_padding = BOARD_PADDING_REF * display.scale;

    let board_corner_radius = BOARD_CORNER_RADIUS_REF * display.scale;

    draw_game_status(game, view_state, display);

    // Background/support behind the four tiles
    draw_rounded_rectangle(
        board.x - board_padding,
        board.y - board_padding,
        board_size + board_padding * 2.0,
        board_size + board_padding * 2.0,
        board_corner_radius,
        BOARD_BACKGROUND,
    );

    // Draw the four tiles
    for tile_row in 0..2 {
        for tile_column in 0..2 {
            let pos_x = board.x + tile_column as f32 * (tile_size + tile_gap);

            let pos_y = board.y + tile_row as f32 * (tile_size + tile_gap);

            draw_texture_ex(
                &textures.tile,
                pos_x,
                pos_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tile_size, tile_size)),
                    ..Default::default()
                },
            );

            draw_marble(game, textures, tile_row, tile_column, pos_x, pos_y, display);
        }
    }
    if let Some((tile_row, tile_column)) = view_state.selected_tile {
        draw_selected_tile(tile_row, tile_column, board, display);
        draw_rotation_buttons(textures, display);
    }
}
