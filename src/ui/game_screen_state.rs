use pentago_engine::game::{Rotation, TileRotation};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotationAnimation {
    pub tile_row: usize,
    pub tile_column: usize,
    pub orientation: TileRotation,
    pub progress: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AiTurnState {
    Idle,

    Thinking { timer: f32 },

    PlacementShown { timer: f32, rotation: Rotation },

    WaitingForRotationAnimation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameScreenState {
    pub selected_tile: Option<(usize, usize)>,
    pub rotation_animation: Option<RotationAnimation>,
    pub ai_turn_state: AiTurnState,
}

impl GameScreenState {
    pub fn new() -> Self {
        Self {
            selected_tile: None,
            rotation_animation: None,
            ai_turn_state: AiTurnState::Idle,
        }
    }
}

impl Default for GameScreenState {
    fn default() -> Self {
        Self::new()
    }
}
