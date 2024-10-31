use std::{fmt::Debug, vec};

use crate::{errors::FailedToGetDataNeows, json_objects::NearEarthObject, Apod, Urls, NEOWS};
use chrono::{NaiveDate, ParseError};
use eframe::egui::{FontId, RichText};
use egui::Image;
use json::JsonValue;

// This is the object that the view port will represent
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct SpacePixUi {
    apod: Apod,
    neows: NEOWS,
    apod_cache: Option<(String, String)>,
    neows_cache: Option<String>,
    about_window_visible: bool,
    apod_full_window_visible: bool,
}

impl Default for SpacePixUi {
    fn default() -> Self {
        Self {
            apod: Apod::default(),
            neows: NEOWS::default(),
            apod_cache: None,
            neows_cache: None,
            about_window_visible: false,
            apod_full_window_visible: false,
        }
    }
}

impl SpacePixUi {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    #[allow(dead_code)]
    pub async fn get_pic_data() -> Result<(String, String), reqwest::Error> {
        let data = reqwest::get("https://api.nasa.gov/planetary/apod?api_key=")
            .await?
            .text()
            .await?;

        let json_object = json::parse(&data).expect("Coultn't parse json.");
        let image_data: (String, String) = (
            json_object["hdurl"].to_string(),
            json_object["explanation"].to_string(),
        );

        Ok(image_data)
    }

    pub fn get_apod_data_blocking(&mut self) -> Result<(String, String), reqwest::Error> {
        match &self.apod.cache {
            Some(cache) => Ok(cache.clone()),
            None => {
                let sauce = Urls::get_secret_sauce().expect("Failed to get secret.");
                let url = Urls::make_secret_sauce(sauce.as_str()).unwrap().apod;
                let data = reqwest::blocking::get(&url)?
                    .text()
                    .expect("Failed to retrieve image from API...");

                let json_object = json::parse(&data).expect("Failed to parse image data...");
                let image_data: (String, String) = (
                    json_object["hdurl"].to_string(),
                    json_object["explanation"].to_string(),
                );

                self.apod.cache = Some(image_data.clone()); // Cache the image
                Ok(image_data)
            }
        }
    }

    pub fn get_neows_data_blocking(
        &mut self,
        dates: (String, String),
    ) -> Result<Vec<NearEarthObject>, FailedToGetDataNeows> {
        // Create NaiveDates from strings
        let naive_start_date =
            NaiveDate::parse_from_str(&dates.0, "%Y-%m-%d").or(Err(FailedToGetDataNeows {}));
        let naive_end_date =
            NaiveDate::parse_from_str(&dates.1, "%Y-%m-%d").or(Err(FailedToGetDataNeows {}));

        // if naive_start_date.is_err() || naive_end_date.is_err() {
        //     return Err(NeoWsInvalidDate{})
        // }

        let url = Urls::build_url_neows(naive_start_date.unwrap(), naive_end_date.unwrap())
            .unwrap_or("fail".to_string());
        let data = reqwest::blocking::get(url)
            .unwrap()
            .text()
            .or(Err(FailedToGetDataNeows {}));
        let json_data = json::parse(&data.unwrap()).unwrap();

        let objects = json_data["near_earth_objects"][&dates.0].members();
        let mut objects_vec: Vec<NearEarthObject> = vec![];

        for object in objects {
            println!(
                "Relative Velocity: {}",
                object["close_approach_data"][0]["relative_velocity"]["miles_per_hour"].to_string()
            );
            let o = NearEarthObject::new(
                object["id"].to_string(),
                object["name"].to_string(),
                (
                    object["estimated_diameter"]["feet"]["estimated_diameter_min"].to_string(),
                    object["estimated_diameter"]["feet"]["estimated_diameter_max"].to_string(),
                ),
                object["is_potentially_hazardous_asteroid"]
                    .as_bool()
                    .unwrap(),
                object["close_approach_data"][0]["close_approach_date"].to_string(),
                object["close_approach_data"][0]["close_approach_date_full"].to_string(),
                object["close_approach_data"][0]["relative_velocity"]["miles_per_hour"].to_string(),
                object["close_approach_data"][0]["miss_distance"]["miles"].to_string(),
                object["close_approach_data"][0]["orbiting_body"].to_string(),
            );
            objects_vec.push(o);
        }

        // println!("{}", json_data["near_earth_objects"][dates.0][0].pretty(4));
        Ok(objects_vec)
    }

    fn show_about(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("about_viewport"),
            egui::ViewportBuilder::default()
                .with_title("About Spacepix")
                .with_inner_size([300.0, 200.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Spacepix");
                    ui.label("Creator & Maintainer: Mark A Waid Jr - mark.waid94@gmail.com");
                    ui.label("License: GNU");
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.about_window_visible = false;
                }
            },
        );
    }

    fn show_apod_full(&mut self, img: &Image, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("apod_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Astronomy Picture of the Day Full Size")
                .with_maximized(true),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.image(img.source(ctx));
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.apod_full_window_visible = false;
                }
            },
        );
    }
}

impl eframe::App for SpacePixUi {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(&ctx);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        println!("Save");
                    }

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Settings", |ui| {
                    if ui.button("APOD").clicked() {
                        println!("APOD Settings");
                    }

                    if ui.button("Asteroids - NeoWs").clicked() {
                        println!("NeoWs Settings");
                        // Urls::build_url_neows(
                        //     NaiveDate::from_ymd_opt(2020, 4, 7).unwrap(),
                        //       NaiveDate::from_ymd_opt(2020, 4, 1).unwrap()
                        // );
                    }

                    if ui.button("DONKI").clicked() {
                        println!("DONKI Settings");
                    }
                    ui.separator();
                    if ui.button("Theme").clicked() {
                        println!("Theme button clicked!");
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.about_window_visible = true; // Set about_window_visible to true so on next update() it will come up.
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // APOD //
            egui::Window::new("APOD (Astronomy Pic Of the Day)")
                .max_height(1000.0)
                .show(ctx, |ui| {
                    // APOD Window //
                    egui::Frame::default().show(ui, |ui| {
                        let image_data = self.get_apod_data_blocking().unwrap();
                        let image = egui::Image::from_uri(image_data.0)
                            .max_size(egui::Vec2::new(100.0, 100.0));
                        //ui.image(image.source(&ctx));
                        if ui
                            .add(egui::widgets::ImageButton::new(image.source(&ctx)))
                            .clicked()
                        {
                            self.apod_full_window_visible = true;
                        }
                        ui.heading(RichText::new("Description:").font(FontId::monospace(30.0)));
                        ui.separator();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label(RichText::new(&image_data.1).font(FontId::monospace(17.0)));
                        });
                        if self.apod_full_window_visible {
                            self.show_apod_full(&image, &ctx);
                        }
                    });
                }); // APOD //

            egui::Window::new("Asteroids - NeoWs").show(ctx, |ui| {
                // NEOWS //
                egui::Frame::default().show(ui, |ui| {
                    ui.label("NEOWS!!");

                    ui.label("Start Date:");
                    ui.text_edit_singleline(&mut self.neows.start_date);

                    ui.label("End Date:");
                    ui.text_edit_singleline(&mut self.neows.end_date);

                    let mut neows_ui_data = Vec::default();
                    if ui.button("Search").clicked() {
                        println!("Start Date: {}", &self.neows.start_date);
                        println!("End Date: {}", &self.neows.end_date);
                        neows_ui_data = self
                            .get_neows_data_blocking((
                                self.neows.start_date.clone(),
                                self.neows.end_date.clone(),
                            ))
                            .unwrap();
                    }

                    // ui.label(neows_ui_data[0].asteroid_id.clone());
                    dbg!(neows_ui_data);
                });
            }); // NEOWS //
        });
        if self.about_window_visible {
            self.show_about(&ctx);
        }
    }
}
