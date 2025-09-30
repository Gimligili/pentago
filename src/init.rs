use std::fs::File;
use std::io::BufReader;
use std::convert::TryInto;
use macroquad::prelude::*;
use crate::miniquad::conf::Icon;
use ico::IconDir;
use image::imageops::FilterType;

pub const GAME_WIDTH: f32 = 800.;
pub const GAME_HEIGHT: f32 = 600.;

pub fn window_conf() -> Conf {

    let icon = load_icon("ui_assets/pentago.ico").expect("failed to load icon.ico");
    Conf { 
        window_title: "Pentago".to_owned(), 
        window_width: GAME_WIDTH as i32,
        window_height: GAME_HEIGHT as i32,
        fullscreen: false, 
        window_resizable: false, 
        icon: Some(icon), 
        ..Default::default()
    }
}

pub fn load_icon(path: &str) -> Result<Icon, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let icon_dir = IconDir::read(&mut reader)?; // decode .ico

    // pick best entry and resize if needed
    let pick_and_resize = |size: u32| -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // choose entry with closest dimensions to `size`
        let best = icon_dir.entries().iter()
            .min_by_key(|e| {
                let dw = (e.width() as i32 - size as i32).abs();
                let dh = (e.height() as i32 - size as i32).abs();
                (dw + dh) as u32
            })
            .ok_or("no images in .ico")?;

        let decoded = best.decode()?;
        let w = decoded.width();
        let h = decoded.height();
        let rgba = decoded.rgba_data().to_vec();

        if w != size || h != size {
            let img = image::RgbaImage::from_raw(w, h, rgba)
                .ok_or("failed to build image buffer")?;
            let resized = image::imageops::resize(&img, size, size, FilterType::Lanczos3);
            Ok(resized.into_raw())
        } else {
            Ok(rgba)
        }
    };

    let small = pick_and_resize(16)?;
    let medium = pick_and_resize(32)?;
    let big = pick_and_resize(64)?;

    let small_arr: [u8; 16*16*4]   = small.try_into().map_err(|_| "16x16 icon wrong len")?;
    let medium_arr: [u8; 32*32*4]  = medium.try_into().map_err(|_| "32x32 icon wrong len")?;
    let big_arr: [u8; 64*64*4]     = big.try_into().map_err(|_| "64x64 icon wrong len")?;

    Ok(Icon { small: small_arr, medium: medium_arr, big: big_arr })
}