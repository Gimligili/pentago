use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
mod init;
mod graphics;
mod style;
mod position;
mod game;

use init::window_conf;

use crate::position::WindowContext;

#[macroquad::main(window_conf())]
async fn main() {

    let skin = style::gen_skin().await;

    let screen = WindowContext::new(init::GAME_WIDTH, init::GAME_HEIGHT);

    loop {
        clear_background(graphics::BLEU_NUIT);


        root_ui().push_skin(&skin);

        root_ui().window(hash!(), screen.pos_from_middle(0.5, 0.5, 0.4, 0.5), screen.gen_size(0.4, 0.5), |ui| {
            let window = WindowContext::new(0.4 * init::GAME_WIDTH, 0.5 * init::GAME_HEIGHT);
            widgets::Button::new("Play")
                .position(window.pos_from_middle(0.5, 0.25, 0.6, 0.2))
                .ui(ui);
            widgets::Button::new("Options")
                .position(window.pos_from_middle(0.5, 0.5, 0.7, 0.2))
                .ui(ui);

            widgets::Button::new("Quit")
                .position(window.pos_from_middle(0.5, 0.75, 0.6, 0.2))
                .ui(ui);
        });
        root_ui().pop_skin();


        next_frame().await;
    }
}
