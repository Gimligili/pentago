use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
mod init;
mod custom_colors;
mod style;
mod position;
mod game;

use init::window_conf;

use crate::position::{pos_from_middle, gen_size};


#[macroquad::main(window_conf)]
async fn main() {

    let skin = style::gen_skin().await;

    loop {
        clear_background(custom_colors::BLEU_NUIT);


        root_ui().push_skin(&skin);

        root_ui().window(hash!(), pos_from_middle(0.5, 0.5, 0.4, 0.5), gen_size(0.4, 0.5), |ui| {
            widgets::Button::new("Play")
                .position(vec2(65.0, 15.0))
                .ui(ui);
            widgets::Button::new("Options")
                .position(vec2(40.0, 75.0))
                .ui(ui);

            widgets::Button::new("Quit")
                .position(vec2(65.0, 195.0))
                .ui(ui);
        });
        root_ui().pop_skin();


        next_frame().await;
    }
}
