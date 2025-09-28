use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
mod icon_loader;
mod custom_colors;
mod style;
mod position;


fn window_conf() -> Conf {

    let icon = icon_loader::load_icon("ui_assets/pentago.ico").expect("failed to load icon.ico");
    Conf { 
        window_title: "Pentago".to_owned(), 
        window_width: position::WIDTH,
        window_height: position::HEIGHT,
        fullscreen: false, 
        window_resizable: false, 
        icon: Some(icon), 
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let skin = style::gen_skin().await;

    loop {
        clear_background(custom_colors::BLEU_NUIT);


        root_ui().push_skin(&skin);

        root_ui().window(hash!(), position::middle_pos(0.5, 0.5, 300., 300.), vec2(300., 300.), |ui| {
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
