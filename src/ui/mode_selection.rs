use macroquad::ui::{hash, root_ui, widgets};

use crate::init;
use crate::position::WindowContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeSelectionAction {
    None,
    PlayerVsPlayer,
    PlayerVsAI,
    Back,
}

pub fn draw_mode_selection(
    skin: &macroquad::ui::Skin,
    screen: &WindowContext,
) -> ModeSelectionAction {
    let mut action = ModeSelectionAction::None;

    root_ui().push_skin(skin);

    root_ui().window(
        hash!(),
        screen.pos_from_middle(0.5, 0.5, 0.5, 0.6),
        screen.gen_size(0.5, 0.6),
        |ui| {
            let window = WindowContext::new(0.5 * init::GAME_WIDTH, 0.6 * init::GAME_HEIGHT);

            if widgets::Button::new("Player vs Player")
                .position(window.pos_from_middle(0.5, 0.25, 0.75, 0.18))
                .ui(ui)
            {
                action = ModeSelectionAction::PlayerVsPlayer;
            }

            if widgets::Button::new("Player vs AI")
                .position(window.pos_from_middle(0.5, 0.50, 0.75, 0.18))
                .ui(ui)
            {
                action = ModeSelectionAction::PlayerVsAI;
            }

            if widgets::Button::new("Back")
                .position(window.pos_from_middle(0.5, 0.75, 0.55, 0.18))
                .ui(ui)
            {
                action = ModeSelectionAction::Back;
            }
        },
    );

    root_ui().pop_skin();

    action
}
