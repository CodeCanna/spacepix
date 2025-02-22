use crate::ui::{AboutWindow, ApiKeyWindow};
use crate::{Apod, ApodWindow, NEOFeed, NIVLWindow, NeowsWindow, Parser, NIVL};
use eframe::egui::{FontId, Image, RichText};
use egui::load::ImageLoader;
use egui::{vec2, ImageSource};
// use egui::ImageSource;
use std::path;

// This is the object that the view port will represent
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct SpacePixUi {
    apod: Option<Apod>,
    neows: Option<NEOFeed>,
    nivl: Option<NIVL>,
    apod_ui: ApodWindow,
    neows_ui: NeowsWindow,
    nivl_ui: NIVLWindow,
    about: AboutWindow,
    api: ApiKeyWindow,
    parser: Parser,
}

impl Default for SpacePixUi {
    fn default() -> Self {
        Self {
            apod: None,
            neows: None,
            nivl: None,
            apod_ui: ApodWindow::default(),
            neows_ui: NeowsWindow::default(),
            nivl_ui: NIVLWindow::default(),
            about: AboutWindow::default(),
            api: ApiKeyWindow::default(),
            parser: Parser::default(),
        }
    }
}

impl SpacePixUi {
    pub fn new(cc: &eframe::CreationContext<'_>, parser: Parser) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // cc.egui_ctx.set_visuals();

        Self {
            parser,
            ..Default::default()
        }
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
                    ui.add(
                        egui::Image::new(egui::include_image!("../spacepix.png"))
                            .max_size(vec2(100.0, 100.0)),
                    );
                    ui.heading("Spacepix");
                    if ui.link("Github").clicked() {
                        match open::that("https://github.com/CodeCanna/spacepix") {
                            Ok(_) => {}
                            Err(_) => {
                                ui.label("Failed to open web browser.");
                            }
                        }
                    }
                    ui.label("Creator & Maintainer: Mark A Waid Jr - ");
                    if ui.link("mark.waid94@gmail.com").clicked() {
                        match open::that("mailto:mark.waid94@gmail.com") {
                            Ok(_) => {}
                            Err(_) => {
                                ui.label("Failed to open browser");
                            }
                        }
                    }
                    ui.label("License: GNU");
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    self.about.about_window_visible = false;
                }
            },
        );
    }

    #[allow(dead_code)]
    fn show_about(&mut self, state: bool) {
        self.about.about_window_visible = state;
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
                                            self.neows_ui.neows_invalid_input_window_visible = false;
                                        }
                                    });

                                    if ctx.input(|i| i.viewport().close_requested()) {
                                        // Tell parent viewport that we should not show next frame:
                                        self.neows_ui.neows_invalid_input_window_visible = false;
                                    }
                                },
    );
    }

    #[allow(dead_code)]
    fn show_apod_full(&mut self, state: bool) {
        self.apod_ui.apod_full_window_visible = state;
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
                            ui.text_edit_singleline(&mut self.api.key);
                            if ui.button("Submit").clicked() {
                                match self.parser.set_api_key(
                                    &path::Path::new("secret.json"),
                                    self.api.key.clone(),
                                ) {
                                    Ok(_) => {
                                        ui.label("Api key Set!");
                                    }
                                    Err(_) => {
                                        ui.label(self.api.key_set_label.clone());
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
                    self.api.api_key_window_visible = false;
                }
            },
        );
    }
}

impl eframe::App for SpacePixUi {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        println!("Save");
                        ui.close_menu();
                    }

                    if ui.button("APOD").clicked() {
                        self.apod_ui.apod_window_visible = true; // Open APOD window
                        ui.close_menu();
                    }

                    if ui.button("Asteroids - NeoWs").clicked() {
                        self.neows_ui.neows_window_visible = true; // Open NEOWs window
                        ui.close_menu();
                    }

                    if ui.button("NASA Image and Video Library").clicked() {
                        self.nivl_ui.nivl_window_visible = true;
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
                        self.api.api_key_window_visible = true;
                        ui.close_menu();
                    }

                    ui.menu_button("Theme", |ui| {
                        if ui.button("Dark").clicked() {
                            println!("Set Dark Theme");
                            // egui::style::Visuals::dark();
                            ui.visuals_mut().dark_mode = true;
                        }

                        if ui.button("Light").clicked() {
                            println!("Set Light Theme");
                            // egui::style::Visuals::light();
                            ui.visuals_mut().dark_mode = false;
                        }
                    });
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.about.about_window_visible = true; // Set about_window_visible to true so on next update() it will come up.
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut apod_window_visible = self.apod_ui.apod_window_visible; // Set a local variable to I don't have to use self with .open() below
                                                                            // APOD //
            egui::Window::new("APOD (Astronomy Pic Of the Day)")
                .max_height(1000.0)
                .open(&mut apod_window_visible) // This doesn't need to be &mut self.apod_ui_apod_window_visible
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
                                    self.apod_ui.apod_full_window_visible = true;
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

                                if self.apod_ui.apod_full_window_visible {
                                    self.apod_ui.apod_full_window(
                                        &egui::Image::from_uri(data.hdurl.clone()),
                                        &data.title.clone(),
                                        &data.copyright.clone(),
                                        &ctx,
                                    );
                                }
                            }
                            None => match Apod::get_apod_data_blocking() {
                                Ok(apod) => self.apod = Some(apod),
                                Err(e) => {
                                    ui.label("Network Error");
                                }
                            },
                        }
                    });
                }); // APOD //
            self.apod_ui.apod_window_visible = apod_window_visible;

            egui::Window::new("Asteroids - NeoWs")
                .open(&mut self.neows_ui.neows_window_visible)
                .show(ctx, |ui| {
                    // NEOWS //
                    let mut next_search: Option<NEOFeed> = None;
                    egui::Frame::default().show(ui, |ui| {
                        ui.label("Enter in a date to start your search from.");
                        ui.label("Date format: YYYY-MM-DD");

                        ui.label("Start Date:");
                        ui.text_edit_singleline(&mut self.neows_ui.neows_date);
                        egui::Grid::new("button_grid")
                            .num_columns(3)
                            .spacing([20.0, 20.0])
                            .show(ui, |ui| {
                                if ui.button("Previous").clicked() {
                                    println!("Clicked Previous");
                                    // Set previous cache to current cache
                                } else if ui.button("Search").clicked() {
                                    println!("Clicked Search");
                                    let mut neows = NEOFeed::default();
                                    match neows.get_neows_feed_blocking(&self.neows_ui.neows_date) {
                                        Ok(_) => {
                                            // next_search = Some(neows);
                                            self.neows = Some(neows);
                                            dbg!(&next_search);
                                        }
                                        Err(e) => {
                                            ui.label(e.to_string());
                                            next_search = None
                                        }
                                    }

                                    // Load new search date
                                } else if ui.button("Next").clicked() {
                                    println!("Clicked Next");
                                    // Load the next url cache from the searched date
                                }
                            });
                        match &self.neows {
                            Some(neo) => {
                                // Display any NeoWs
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    for object in neo.near_earth_objects.clone() {
                                        if ui
                                            .link(&object.name.replace("(", "").replace(")", ""))
                                            .clicked()
                                        {
                                            match open::that(format!(
                                                "https://eyes.nasa.gov/apps/asteroids/#/{}",
                                                &object
                                                    .name
                                                    .replace(" ", "_")
                                                    .replace("(", "")
                                                    .replace(")", "")
                                                    .to_lowercase()
                                            )) {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    ui.label(&e.to_string());
                                                }
                                            }
                                        }

                                        ui.add(egui::Label::new(format!(
                                            "Asteroid Id: {}",
                                            &object.neo_reference_id
                                        )));
                                        ui.label(format!(
                                            "Near Miss Date: {}",
                                            &object.close_approach_date_full
                                        ));
                                        ui.label(format!(
                                            "Distance: {} miles from Earth",
                                            &object.miss_distance.3
                                        ));
                                        ui.label(format!(
                                            "Relative Velocity: {} miles per hour",
                                            object.relative_velocity.2
                                        ));
                                        ui.label(format!(
                                            "Estimated Diameter: (min {} feet\nmax {} feet)",
                                            object.estimated_diameter.0 .0,
                                            object.estimated_diameter.0 .1
                                        ));
                                        ui.label(format!(
                                            "Deemed hazardous by NASA: {}",
                                            object.is_potentially_hazardous_asteroid
                                        ));
                                        ui.separator();
                                    }
                                }); // Scroll Area
                                // self.neows = next_search;
                            }
                            None => {
                                self.neows = next_search;
                                // dbg!(&self.neows);
                            }
                        }
                    });
                }); // NEOWS //

            egui::Window::new("NASA Image and Video Library")
                .open(&mut self.nivl_ui.nivl_window_visible)
                .show(ctx, |ui| {
                    egui::Frame::default().show(ui, |ui| {
                        ui.label("Enter a search term");
                        ui.text_edit_singleline(&mut self.nivl_ui.query);
                    })
                }); // NIVL
        });
        if self.api.api_key_window_visible {
            self.show_api_input(&ctx.clone());
        }

        if self.neows_ui.neows_invalid_input_window_visible {
            self.show_neows_invlid_input_win(&ctx.clone());
        }

        if self.about.about_window_visible {
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
