use crate::apis::ApiKey;
use crate::errors::ApiKeyError;
use crate::{Apod, NEOFeed, NearEarthObject, Urls};
use chrono::NaiveDate;
use eframe::egui::{FontId, RichText};
use egui::Image;
use json::object;
use std::io::Write;
use std::{fs, path};
use std::{path::Path, vec};

// This is the object that the view port will represent
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct SpacePixUi {
    apod: Option<Apod>,
    neows: Option<NearEarthObject>,
    api_key: ApiKey,
    about_window_visible: bool,
    api_key_input_visible: bool,
    apod_full_window_visible: bool,
    neows_invalid_input_window_visible: bool,
}

impl Default for SpacePixUi {
    fn default() -> Self {
        Self {
            apod: None,
            neows: None,
            api_key: ApiKey::default(),
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

    fn set_api_key(&mut self, secret_path: &Path, key: String) -> Result<(), ApiKeyError> {
        match fs::File::create(secret_path) {
            Ok(mut f) => {
                let json_buff = object! {key: key};
                let _ = f.write(json_buff.to_string().as_bytes());
                Ok(())
            }
            Err(e) => Err(ApiKeyError::KeyStore(e)),
        }
    }

    fn about_window(&mut self, ctx: &egui::Context) {
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

    fn show_about(&mut self, state: bool) {
        self.about_window_visible = state;
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

    fn apod_full_window(
        &mut self,
        img: &Image,
        image_name: &String,
        image_credit: &String,
        ctx: &egui::Context,
    ) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("apod_viewport"),
            egui::ViewportBuilder::default()
                .with_title(format!(
                    "{} (By {})",
                    &image_name,
                    &image_credit.replace("\n", "")
                ))
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

    fn show_apod_full(&mut self, state: bool) {
        self.apod_full_window_visible = state;
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
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.heading("Enter your NASA API Key below.");
                            ui.label("NOTE: Type DEMO_KEY to use the demo key, with limitations.");
                            ui.text_edit_singleline(&mut self.api_key.key);
                            if ui.button("Submit").clicked() {
                                match self.set_api_key(
                                    &path::Path::new("secret.json"),
                                    self.api_key.key.clone(),
                                ) {
                                    Ok(_) => {
                                        ui.label("Api key Set!");
                                    }
                                    Err(e) => {
                                        ui.label(&e.to_string());
                                    }
                                }
                            }

                            if ui.link("Don't have a NASA API Key?").clicked() {
                                match open::that("https://api.nasa.gov/") {
                                    Ok(_) => {}
                                    Err(_) => {
                                        ui.label("Failed to open web browser.");
                                    }
                                }
                            }
                        },
                    );
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
                        ui.close_menu();
                    }

                    if ui.button("APOD").clicked() {
                        println!("APOD Settings");
                        ui.close_menu();
                    }

                    if ui.button("Asteroids - NeoWs").clicked() {
                        println!("NeoWs Settings");
                        ui.close_menu();
                    }

                    if ui.button("DONKI").clicked() {
                        println!("DONKI Settings");
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        ui.close_menu();
                    }
                });

                ui.menu_button("Settings", |ui| {
                    if ui.button("Set API Key").clicked() {
                        self.api_key_input_visible = true;
                        ui.close_menu();
                    }

                    if ui.button("Theme").clicked() {
                        println!("Theme button clicked!");
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.about_window_visible = true; // Set about_window_visible to true so on next update() it will come up.
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // APOD //
            egui::Window::new("APOD (Astronomy Pic Of the Day)")
                .max_height(1000.0)
                //.open(&mut self.apod_window_visible)
                .show(ctx, |ui| {
                    // APOD Window //
                    egui::Frame::default().show(ui, |ui| {
                        match &self.apod {
                            Some(data) => {
                                ui.heading(
                                    RichText::new(data.title.clone()).font(FontId::monospace(20.0)),
                                );
                                if ui
                                    .add(egui::widgets::ImageButton::new(egui::Image::from_uri(
                                        data.url.clone(),
                                    )))
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                                {
                                    self.apod_full_window_visible = true;
                                    //self.show_apod_full(true);
                                }
                                ui.label(format!(
                                    "Copyright: {}",
                                    data.copyright.clone().replace("\n", "")
                                ));
                                ui.heading(
                                    RichText::new("Description:").font(FontId::monospace(30.0)),
                                );
                                ui.separator();
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.label(
                                        RichText::new(data.explanation.clone())
                                            .font(FontId::monospace(17.0)),
                                    );
                                });

                                if self.apod_full_window_visible {
                                    self.apod_full_window(
                                        &egui::Image::from_uri(data.hdurl.clone()),
                                        &data.title.clone(),
                                        &data.copyright.clone(),
                                        &ctx,
                                    );
                                }
                            }
                            None => self.apod = Some(Apod::get_apod_data_blocking().unwrap()),
                        }
                    });
                }); // APOD //

            egui::Window::new("Asteroids - NeoWs").show(ctx, |ui| {
                // NEOWS //
                egui::Frame::default().show(ui, |ui| {
                    let neows = NEOFeed::get_neows_feed_blocking("2024-10-01");
                    // ui.label("Start and End dates must be within 7 days of eachother.");
                    // ui.label("If you only want to search for one day, only enter a start date, and leave end date empty.");
                    // ui.label("Date format: YYYY-MM-DD");

                    // ui.label("Start Date:");
                    // ui.text_edit_singleline(&mut self.neows.start_date);

                    // ui.label("End Date:");
                    // ui.text_edit_singleline(&mut self.neows.end_date);

                    // // let mut neows_ui_data = Vec::default();
                    // if ui.button("Search").clicked() {
                    //     match self.get_neows_data_blocking((
                    //         self.neows.start_date.clone(),
                    //         self.neows.end_date.clone(),
                    //     )) {
                    //         Ok(data) => {
                    //             self.neows.neows.clear(); // Clear old data
                    //             for object in data {
                    //                 self.neows.neows.push(object);
                    //             }
                    //             dbg!(&self.neows.neows);
                    //         }
                    //         Err(_) => {
                    //             self.neows_invalid_input_window_visible = true;
                    //         }
                    //     }
                    // }

                    // ui.separator();

                    // // Display any NeoWs
                    // egui::ScrollArea::vertical().show(ui, |ui| {
                    //     for object in &self.neows.neows {
                    //         if ui.link(&object.name.replace("(", "").replace(")", "")).clicked() {
                    //             match open::that(format!("https://eyes.nasa.gov/apps/asteroids/#/{}", &object.name.replace(" ", "_").replace("(", "").replace(")", "").to_lowercase())) {
                    //                 Ok(_) => {},
                    //                 Err(e) => { ui.label(&e.to_string()); }
                    //             }
                    //         }

                    //         ui.add(egui::Label::new(format!("Asteroid Id: {}", &object.asteroid_id)));
                    //         ui.label(format!("Near Miss Date: {}", &object.close_approach_time));
                    //         ui.label(format!("Distance Min: {} miles from Earth\nDistance Max: {} miles from Earth", &object.estimated_diameter.0, &object.estimated_diameter.1));
                    //         ui.label(format!("Relative Velocity: {} miles per hour", object.relative_velocity));
                    //         ui.label(format!("Estimated Diameter: (min {} feet\nmax {} feet", object.estimated_diameter.0, object.estimated_diameter.1));
                    //         ui.label(format!("Deemed hazardous by NASA: {}", object.is_potentially_hazardous_asteroid));
                    //         ui.separator();
                    //     }
                    // });
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
            self.about_window(&ctx);
        }
    }
}
// beans
#[cfg(test)]
mod tests {
    #[test]
    fn test_get_neows_data_blocking() {
        todo!()
    }

    #[test]
    fn test_get_apod_data_blocking() {
        todo!()
    }

    #[test]
    fn test_set_api_key() {
        todo!()
    }
}
