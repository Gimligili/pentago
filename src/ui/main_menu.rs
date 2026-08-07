use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

use crate::init;
use crate::position::WindowContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuAction {
    None,
    Play,
    Options,
    Quit,
}

pub fn draw_main_menu(skin: &macroquad::ui::Skin, screen: &WindowContext) -> MainMenuAction {
    let mut action = MainMenuAction::None;

    draw_text_ex(
        "Pentago",
        init::GAME_WIDTH * 0.28,
        init::GAME_HEIGHT * 0.2,
        TextParams {
            font: None,
            font_size: (init::GAME_HEIGHT * 0.2) as u16,
            font_scale: get_time().sin() as f32 / 2.0 + 1.0,
            color: WHITE,
            rotation: 0.0,
            font_scale_aspect: 1.0,
        },
    );

    root_ui().push_skin(skin);

    root_ui().window(
        hash!(),
        screen.pos_from_middle(0.5, 0.6, 0.4, 0.5),
        screen.gen_size(0.4, 0.5),
        |ui| {
            let window = WindowContext::new(0.4 * init::GAME_WIDTH, 0.5 * init::GAME_HEIGHT);

            if widgets::Button::new("Play")
                .position(window.pos_from_middle(0.5, 0.25, 0.6, 0.2))
                .ui(ui)
            {
                action = MainMenuAction::Play;
            }

            if widgets::Button::new("Options")
                .position(window.pos_from_middle(0.5, 0.5, 0.7, 0.2))
                .ui(ui)
            {
                action = MainMenuAction::Options;
            }

            if widgets::Button::new("Quit")
                .position(window.pos_from_middle(0.5, 0.75, 0.6, 0.2))
                .ui(ui)
            {
                action = MainMenuAction::Quit;
            }
        },
    );

    root_ui().pop_skin();

    action
}
