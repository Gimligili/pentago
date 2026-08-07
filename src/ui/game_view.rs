use macroquad::prelude::*;

use crate::game::{CellState, Game, TurnState};

const TOP_UI_HEIGHT: f32 = 105.0;

pub const TILE_SIZE: f32 = 230.0;
pub const TILE_GAP: f32 = 12.0;

pub const BOARD_SIZE: f32 = TILE_SIZE * 2.0 + TILE_GAP;
const BOARD_PADDING: f32 = 14.0;
const BOARD_CORNER_RADIUS: f32 = 24.0;
const BOARD_BACKGROUND: Color = Color::from_rgba(45, 30, 20, 255);

const CELL_SIZE: f32 = TILE_SIZE / 3.0;
const MARBLE_SIZE: f32 = CELL_SIZE * 0.65;

const ROTATION_BUTTON_WIDTH: f32 = 60.0;
const ROTATION_BUTTON_HEIGHT: f32 = 25.0;
const ROTATION_BUTTON_MARGIN: f32 = 24.0;
const ROTATION_BUTTON_GAP: f32 = 16.0;

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

fn draw_marble(
    game: &Game,
    textures: &GameTextures,
    tile_row: usize,
    tile_column: usize,
    pos_x: f32,
    pos_y: f32,
) {
    for row in 0..3 {
        for column in 0..3 {
            let cell = game.board.tiles[tile_row][tile_column].cells[row][column];

            let texture = match cell.state {
                CellState::Empty => None,
                CellState::White => Some(&textures.white_marble),
                CellState::Black => Some(&textures.black_marble),
            };

            if let Some(texture) = texture {
                let marble_x = pos_x + column as f32 * CELL_SIZE + (CELL_SIZE - MARBLE_SIZE) / 2.0;

                let marble_y = pos_y + row as f32 * CELL_SIZE + (CELL_SIZE - MARBLE_SIZE) / 2.0;

                draw_texture_ex(
                    texture,
                    marble_x,
                    marble_y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(MARBLE_SIZE, MARBLE_SIZE)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

fn draw_selected_tile(tile_row: usize, tile_column: usize, board: Vec2) {
    let x = board.x + tile_column as f32 * (TILE_SIZE + TILE_GAP);
    let y = board.y + tile_row as f32 * (TILE_SIZE + TILE_GAP);

    draw_rectangle_lines(
        x - 4.0,
        y - 4.0,
        TILE_SIZE + 8.0,
        TILE_SIZE + 8.0,
        4.0,
        GOLD,
    );
}

pub fn rotation_buttons_rect() -> (Rect, Rect) {
    let board_x = (screen_width() - BOARD_SIZE) / 2.0;
    let board_y = (screen_height() - BOARD_SIZE) / 2.0;

    let x = board_x + BOARD_SIZE + ROTATION_BUTTON_MARGIN;

    let total_height = ROTATION_BUTTON_HEIGHT * 2.0 + ROTATION_BUTTON_GAP;

    let start_y = board_y + (BOARD_SIZE - total_height) / 2.0;

    let counter_clockwise = Rect::new(x, start_y, ROTATION_BUTTON_WIDTH, ROTATION_BUTTON_HEIGHT);

    let clockwise = Rect::new(
        x,
        start_y + ROTATION_BUTTON_HEIGHT + ROTATION_BUTTON_GAP,
        ROTATION_BUTTON_WIDTH,
        ROTATION_BUTTON_HEIGHT,
    );

    (counter_clockwise, clockwise)
}

fn draw_rotation_buttons(textures: &GameTextures) {
    let (left_button, right_button) = rotation_buttons_rect();

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

fn draw_game_status(game: &Game, view_state: &GameViewState) {
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


    draw_text(player_text, 30.0, 35.0, 40.0, WHITE);
    draw_text(action_text, 30.0, 70.0, 25.0, GRAY);
}

pub fn board_origin() -> Vec2 {
    vec2(
        (screen_width() - BOARD_SIZE) / 2.0,
        TOP_UI_HEIGHT,
    )
}

pub fn draw_game(game: &Game, textures: &GameTextures, view_state: &GameViewState) {
    let board = board_origin();

    draw_game_status(game, view_state);

    // Background/support behind the four tiles
    draw_rounded_rectangle(
        board.x - BOARD_PADDING,
        board.y - BOARD_PADDING,
        BOARD_SIZE + BOARD_PADDING * 2.0,
        BOARD_SIZE + BOARD_PADDING * 2.0,
        BOARD_CORNER_RADIUS,
        BOARD_BACKGROUND,
    );

    // Draw the four tiles
    for tile_row in 0..2 {
        for tile_column in 0..2 {
            let pos_x = board.x + tile_column as f32 * (TILE_SIZE + TILE_GAP);
            let pos_y = board.y + tile_row as f32 * (TILE_SIZE + TILE_GAP);

            draw_texture_ex(
                &textures.tile,
                pos_x,
                pos_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
            );

            draw_marble(game, textures, tile_row, tile_column, pos_x, pos_y);
        }
    }
    if let Some((tile_row, tile_column)) = view_state.selected_tile {
        draw_selected_tile(tile_row, tile_column, board);
        draw_rotation_buttons(textures);
    }
}
