#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use egui;

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

  eframe::run_native(
    "Space Pix",
    native_options,
    Box::new(|cc| Ok(Box::new(spacepix::SpacePixUi::new(cc)))),
  )
}
