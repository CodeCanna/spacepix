use crate::apis::ApiKey;
use crate::errors::{FailedToGetDataApod, FailedToGetDataNeows, SetAPIKeyFailed};
use crate::{json_objects::NearEarthObject, Apod, Urls, NEOWS};
use chrono::NaiveDate;
use eframe::egui::{FontId, RichText};
use egui::Image;
use std::io::{Read, Write};
use std::{fs, path};
use std::{path::Path, vec};
use json::object;

const SAUCE_PATH: &str = "secret.json";

// This is the object that the view port will represent
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct SpacePixUi {
    apod: Apod,
    neows: NEOWS,
    api_key: ApiKey,
    apod_cache: Option<(String, String)>,
    neows_cache: Option<String>,
    about_window_visible: bool,
    api_key_input_visible: bool,
    apod_full_window_visible: bool,
    neows_invalid_input_window_visible: bool,
}

impl Default for SpacePixUi {
    fn default() -> Self {
        Self {
            apod: Apod::default(),
            neows: NEOWS::default(),
            api_key: ApiKey::default(),
            apod_cache: None,
            neows_cache: None,
            about_window_visible: false,
            api_key_input_visible: false,
            apod_full_window_visible: false,
            neows_invalid_input_window_visible: false,
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

    pub fn get_apod_data_blocking(&mut self) -> Result<(String, String), FailedToGetDataApod> {
        match &self.apod.cache {
            Some(cache) => Ok(cache.clone()),
            None => {
                //let sauce = Urls::get_secret_sauce().expect("Failed to get secret.");
                let sauce = match Urls::get_secret_sauce() {
                    Ok(sus) => sus,
                    Err(_) => return Err(FailedToGetDataApod {}),
                };
                let url = Urls::make_secret_sauce(&sauce).unwrap().apod;
                //let data = reqwest::blocking::get(&url)?.text()?;

                let data = match reqwest::blocking::get(&url) {
                    Ok(r) => r.text().unwrap(),
                    Err(_) => return Err(FailedToGetDataApod {}),
                };

                let json_object = json::parse(&data).unwrap(); //.expect("Failed to parse image data...");
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
        let naive_start_date = match NaiveDate::parse_from_str(&dates.0, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return Err(FailedToGetDataNeows {}),
        };

        let naive_end_date = match NaiveDate::parse_from_str(&dates.1, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return Err(FailedToGetDataNeows {}),
        };

        let url =
            Urls::build_url_neows(naive_start_date, naive_end_date).unwrap_or("fail".to_string());

        let data = match reqwest::blocking::get(url) {
            Ok(d) => d.text(),
            Err(_) => return Err(FailedToGetDataNeows {}),
        };
        // let data = reqwest::blocking::get(url)
        //     .unwrap()
        //     .text()
        //     .or(Err(FailedToGetDataNeows {}));
        //let json_data = json::parse(&data.unwrap());

        let json_data = match json::parse(&data.unwrap()) {
            Ok(jv) => jv,
            Err(_) => return Err(FailedToGetDataNeows {}),
        };

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
        Ok(objects_vec)
    }

    fn set_api_key(
        &mut self,
        secret_path: &Path,
        key: String,
    ) -> Result<(), SetAPIKeyFailed> {
        match fs::File::create(secret_path) {
            Ok(mut f) => {
                let json_buff = object! {key: key};
                let _ = f.write(json_buff.to_string().as_bytes());
                Ok(())
            },
            Err(_) => {return Err(SetAPIKeyFailed{})}
        }
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

    fn show_neows_invlid_input_win(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("invalid_neows_input_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Invalid Input")
                .with_inner_size([300.0, 200.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("NeoWs Invalid Input!");
                    ui.label("Both inputs must be valid dates within 7 days of eachother.  Also both vields must be filled out.");
                    ui.label("Input Error!");
                    if ui.button("Ok").clicked() {
                        self.neows_invalid_input_window_visible = false;
                    }
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.neows_invalid_input_window_visible = false;
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

    fn show_api_input(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("show_api_input_viewport"),
            egui::ViewportBuilder::default()
                .with_title("API Key")
                .with_inner_size([300.0, 200.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("API Key not Found Enter it Below:");
                    ui.text_edit_singleline(&mut self.api_key.key);
                    if ui.button("Submit").clicked() {
                        match self
                            .set_api_key(&path::Path::new("secret.json"), self.api_key.key.clone())
                        {
                            Ok(_) => {
                                ui.label("Api key Set!");
                            }
                            Err(e) => {
                                ui.label(&e.to_string());
                            }
                        }
                    }
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.api_key_input_visible = false;
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
                    if ui.button("Set API Key").clicked() {
                        self.api_key_input_visible = true;
                    }

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
                        let image_data = self.get_apod_data_blocking();
                        match image_data {
                            Ok(data) => {
                                if ui
                                    .add(egui::widgets::ImageButton::new(egui::Image::from_uri(
                                        &data.0,
                                    )))
                                    .clicked()
                                {
                                    self.apod_full_window_visible = true;
                                }
                                ui.heading(
                                    RichText::new("Description:").font(FontId::monospace(30.0)),
                                );
                                ui.separator();
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.label(RichText::new(&data.1).font(FontId::monospace(17.0)));
                                });
                                if self.apod_full_window_visible {
                                    self.show_apod_full(&egui::Image::from_uri(&data.0), &ctx);
                                }
                            }
                            Err(_) => {
                                ui.label("Network connection error!");
                            }
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

                    // let mut neows_ui_data = Vec::default();
                    if ui.button("Search").clicked() {
                        match self.get_neows_data_blocking((
                            self.neows.start_date.clone(),
                            self.neows.end_date.clone(),
                        )) {
                            Ok(data) => {
                                for object in data {
                                    println!("{}", object.asteroid_id);
                                    ui.add(egui::Label::new(object.asteroid_id));
                                }
                            }
                            Err(_) => {
                                self.neows_invalid_input_window_visible = true;
                            }
                        }
                    }
                });
            }); // NEOWS //
        });
        if self.api_key_input_visible {
            self.show_api_input(&ctx.clone());
        }

        if self.neows_invalid_input_window_visible {
            self.show_neows_invlid_input_win(&ctx.clone());
        }

        if self.about_window_visible {
            self.show_about(&ctx);
        }
    }
}
