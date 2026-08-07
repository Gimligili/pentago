use macroquad::prelude::*;

use crate::game::{CellState, Game};

const TILE_SIZE: f32 = 220.0;
const TILE_GAP: f32 = 12.0;

const BOARD_SIZE: f32 = TILE_SIZE * 2.0 + TILE_GAP;
const BOARD_PADDING: f32 = 14.0;
const BOARD_CORNER_RADIUS: f32 = 24.0;
const BOARD_BACKGROUND: Color = Color::from_rgba(45, 30, 20, 255);

const CELL_SIZE: f32 = TILE_SIZE / 3.0;
const MARBLE_SIZE: f32 = CELL_SIZE * 0.65;

pub struct GameTextures {
    pub tile: Texture2D,
    pub white_marble: Texture2D,
    pub black_marble: Texture2D,
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

fn draw_marble(game: &Game, textures: &GameTextures, tile_row: usize, tile_column: usize, pos_x: f32, pos_y: f32) {
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

pub fn draw_game(game: &Game, textures: &GameTextures) {
    let board_x = (screen_width() - BOARD_SIZE) / 2.0;
    let board_y = (screen_height() - BOARD_SIZE) / 2.0;

    // Background/support behind the four tiles
    draw_rounded_rectangle(
        board_x - BOARD_PADDING,
        board_y - BOARD_PADDING,
        BOARD_SIZE + BOARD_PADDING * 2.0,
        BOARD_SIZE + BOARD_PADDING * 2.0,
        BOARD_CORNER_RADIUS,
        BOARD_BACKGROUND,
    );

    // Draw the four tiles
    for tile_row in 0..2 {
        for tile_column in 0..2 {
            let pos_x = board_x + tile_column as f32 * (TILE_SIZE + TILE_GAP);
            let pos_y = board_y + tile_row as f32 * (TILE_SIZE + TILE_GAP);

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
}
