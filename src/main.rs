#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use egui;
use spacepix::Parser;
use std::{fs, io::Read};

const SECRET: &str = "secret.json";

fn load_icon(path: &str) -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(load_icon("spacepix.png")),
        ..Default::default()
    };

    match fs::File::open(&SECRET) {
        Ok(mut f) => {
            let mut key = String::default();
            f.read_to_string(&mut key).unwrap();
            let key_json: serde_json::Value = serde_json::from_str(&key.as_str()).unwrap();//json::from(key);
            let parser = Parser::new(key_json["key"].to_string());
            Ok(eframe::run_native(
            "Space Pix",
            native_options,
            Box::new(|cc| Ok(Box::new(spacepix::SpacePixUi::new(cc, parser)))),
            )?)
        },
        Err(_) => panic!("Missing API key file..."),
    }
}
