use macroquad::prelude::*;

use super::game_screen_state::{GameScreenState, RotationAnimation};
use crate::display::DisplayContext;
use pentago_engine::game::{CellState, Game, GameMode, Placement, TileRotation, TurnState};

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
pub const ROTATION_ANIMATION_DURATION: f32 = 0.30;

pub struct GameTextures {
    pub tile: Texture2D,
    pub white_marble: Texture2D,
    pub black_marble: Texture2D,
    pub rotation_arrow: Texture2D,
    pub tile_render_target: RenderTarget,
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
        let tile_render_target = render_target(TILE_SIZE_REF as u32, TILE_SIZE_REF as u32);
        tile_render_target.texture.set_filter(FilterMode::Linear);

        Self {
            tile: load_texture_with_transparency("ui_assets/tile.png").await,
            white_marble: load_texture_with_transparency("ui_assets/sphere_white.png").await,
            black_marble: load_texture_with_transparency("ui_assets/sphere_black.png").await,
            rotation_arrow: load_texture_with_transparency("ui_assets/arrow.png").await,
            tile_render_target,
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

fn cell_position(placement: &Placement, display: &DisplayContext) -> Vec2 {
    let board = board_origin(display);

    let tile_size = tile_size(display);
    let tile_gap = tile_gap(display);
    let cell_size = tile_size / 3.0;

    vec2(
        board.x
            + placement.tile_column as f32 * (tile_size + tile_gap)
            + placement.column as f32 * cell_size,
        board.y
            + placement.tile_row as f32 * (tile_size + tile_gap)
            + placement.row as f32 * cell_size,
    )
}

fn draw_invalid_placement_cross(cell_pos: Vec2, cell_size: f32, display: &DisplayContext) {
    let margin = cell_size * 0.28;
    let thickness = 5.0 * display.scale;

    draw_line(
        cell_pos.x + margin,
        cell_pos.y + margin,
        cell_pos.x + cell_size - margin,
        cell_pos.y + cell_size - margin,
        thickness,
        RED,
    );

    draw_line(
        cell_pos.x + cell_size - margin,
        cell_pos.y + margin,
        cell_pos.x + margin,
        cell_pos.y + cell_size - margin,
        thickness,
        RED,
    );
}

fn draw_placement_preview(
    game: &Game,
    textures: &GameTextures,
    placement: &Placement,
    display: &DisplayContext,
) {
    let tile_size = tile_size(display);
    let cell_size = tile_size / 3.0;
    let marble_size = cell_size * MARBLE_RATIO;

    let cell_pos = cell_position(placement, display);

    let marble_x = cell_pos.x + (cell_size - marble_size) / 2.0;

    let marble_y = cell_pos.y + (cell_size - marble_size) / 2.0;

    let cell = game.board.tiles[placement.tile_row][placement.tile_column].cells[placement.row]
        [placement.column];

    if cell.state == CellState::Empty {
        let texture = match game.current_player {
            CellState::White => &textures.white_marble,
            CellState::Black => &textures.black_marble,
            CellState::Empty => return,
        };

        draw_texture_ex(
            texture,
            marble_x,
            marble_y,
            Color::new(0.65, 0.65, 0.65, 0.55),
            DrawTextureParams {
                dest_size: Some(vec2(marble_size, marble_size)),
                ..Default::default()
            },
        );
    } else {
        draw_invalid_placement_cross(cell_pos, cell_size, display);
    }
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

fn draw_tile_highlight(
    tile_row: usize,
    tile_column: usize,
    board: Vec2,
    display: &DisplayContext,
    color: Color,
    thickness: f32,
) {
    let tile_size = tile_size(display);
    let tile_gap = tile_gap(display);

    let x = board.x + tile_column as f32 * (tile_size + tile_gap);

    let y = board.y + tile_row as f32 * (tile_size + tile_gap);

    draw_rectangle_lines(
        x - 2.0 * thickness,
        y - 2.0 * thickness,
        tile_size + thickness * 4.0,
        tile_size + thickness * 4.0,
        thickness,
        color,
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

fn animation_smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn draw_tile_marbles_to_target(
    game: &Game,
    textures: &GameTextures,
    tile_row: usize,
    tile_column: usize,
) {
    let cell_size = TILE_SIZE_REF / 3.0;

    let marble_size = cell_size * MARBLE_RATIO;

    for row in 0..3 {
        for column in 0..3 {
            let state = game.board.tiles[tile_row][tile_column].cells[row][column].state;

            let texture = match state {
                CellState::Empty => continue,
                CellState::White => &textures.white_marble,
                CellState::Black => &textures.black_marble,
            };

            let x = column as f32 * cell_size + (cell_size - marble_size) / 2.0;

            let y = row as f32 * cell_size + (cell_size - marble_size) / 2.0;

            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(marble_size, marble_size)),
                    ..Default::default()
                },
            );
        }
    }
}

fn draw_rotating_tile(
    game: &Game,
    textures: &GameTextures,
    animation: &RotationAnimation,
    display: &DisplayContext,
) {
    let target_size = TILE_SIZE_REF;

    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, target_size, target_size));

    camera.render_target = Some(textures.tile_render_target.clone());
    set_camera(&camera);

    // Transparent render target background
    clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

    // Tile
    draw_texture_ex(
        &textures.tile,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(target_size, target_size)),
            ..Default::default()
        },
    );

    draw_tile_marbles_to_target(game, textures, animation.tile_row, animation.tile_column);

    set_default_camera();

    let board = board_origin(display);
    let tile_size = tile_size(display);
    let tile_gap = tile_gap(display);

    let x = board.x + animation.tile_column as f32 * (tile_size + tile_gap);
    let y = board.y + animation.tile_row as f32 * (tile_size + tile_gap);

    let eased = animation_smoothstep(animation.progress);
    let angle = match animation.orientation {
        TileRotation::Clockwise => std::f32::consts::FRAC_PI_2 * eased,

        TileRotation::CounterClockwise => -std::f32::consts::FRAC_PI_2 * eased,
    };

    draw_texture_ex(
        &textures.tile_render_target.texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(tile_size, tile_size)),
            rotation: angle,
            flip_y: true,
            ..Default::default()
        },
    );
}

fn draw_game_status(
    game: &Game,
    game_screen_state: &GameScreenState,
    display: &DisplayContext,
    font: &Font,
) {
    let player_text = match game.current_player {
        CellState::White => "White player's turn",
        CellState::Black => "Black player's turn",
        CellState::Empty => "",
    };

    let action_text = match game.state {
        TurnState::WaitingForPlacement => "Place a marble",
        TurnState::PlacementDone => "Press 'Enter' to confirm or click 'right-click' to cancel",
        TurnState::WaitingForRotation => {
            if game_screen_state.selected_tile.is_some() {
                "Choose selected tile rotation direction"
            } else {
                "Select a tile to rotate"
            }
        }
        TurnState::RotationDone => "Press 'Enter' to confirm or click 'right-click' to cancel",
    };

    draw_text_ex(
        player_text,
        display.x(30.0),
        display.y(35.0),
        TextParams {
            font: Some(font),
            font_size: (40.0 * display.scale) as u16,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(
        action_text,
        display.x(30.0),
        display.y(70.0),
        TextParams {
            font: Some(font),
            font_size: (25.0 * display.scale) as u16,
            color: GRAY,
            ..Default::default()
        },
    );
}

pub fn draw_game(
    game: &Game,
    textures: &GameTextures,
    game_screen_state: &GameScreenState,
    display: &DisplayContext,
    font: &Font,
) {
    let board = board_origin(display);
    let human_turn =
        game.game_mode == GameMode::PlayerVsPlayer || game.current_player == CellState::White;

    let tile_size = tile_size(display);
    let tile_gap = tile_gap(display);
    let board_size = board_size(display);
    let board_padding = BOARD_PADDING_REF * display.scale;
    let board_corner_radius = BOARD_CORNER_RADIUS_REF * display.scale;

    draw_game_status(game, game_screen_state, display, font);

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

            let is_animated =
                game_screen_state
                    .rotation_animation
                    .as_ref()
                    .is_some_and(|animation| {
                        animation.tile_row == tile_row && animation.tile_column == tile_column
                    });

            if is_animated {
                continue;
            }

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

    // Handle rotation animation if needed
    if let Some(animation) = &game_screen_state.rotation_animation {
        draw_rotating_tile(game, textures, animation, display);
    }

    if human_turn {
        if game.state == TurnState::WaitingForRotation {
            if let Some((tile_row, tile_column)) = game_screen_state.selected_tile {
                draw_tile_highlight(
                    tile_row,
                    tile_column,
                    board,
                    display,
                    GOLD,
                    4.0 * display.scale,
                );

                draw_rotation_buttons(textures, display);
            } else if let Some((tile_row, tile_column)) = crate::ui::input::hovered_tile(display) {
                draw_tile_highlight(
                    tile_row,
                    tile_column,
                    board,
                    display,
                    Color::from_rgba(180, 180, 180, 160),
                    3.0 * display.scale,
                );
            }
        } else if game.state == TurnState::WaitingForPlacement
            && let Some(placement) = crate::ui::input::hovered_placement(display)
        {
            draw_placement_preview(game, textures, &placement, display);
        }
    }
}
