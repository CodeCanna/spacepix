use crate::{Apod, NEOWS, Urls};
use eframe::egui::{FontId, RichText};

// This is the object that the view port will represent
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct SpacePixUi {
    apod: Apod,
    neows: NEOWS,
    apod_cache: Option<(String, String)>
}

impl Default for SpacePixUi {
    fn default() -> Self {
        Self {
            apod: Apod::default(),
            neows: NEOWS::default(),
            apod_cache: None
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
                let sauce = Urls::get_secret_sauce()
                        .expect("Failed to get the sauuuuuuuce!!!");
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

    fn show_about(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Immediate Viewport")
                .with_inner_size([200.0, 100.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Hello from immediate viewport");
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    //self.show_immediate_viewport = false;
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
                    }

                    if ui.button("DONKI").clicked() {
                        println!("DONKI Settings");
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        println!("Show Help");
                        self.show_about(&ctx.clone());
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| { // APOD //
            egui::Window::new("APOD (Astronomy Pic Of the Day)").max_height(1000.0).show(ctx, |ui| { // APOD Window //
                egui::Frame::default().show(ui, |ui| {
                    let image_data = self.get_apod_data_blocking().unwrap();
                    let image = egui::Image::from_uri(image_data.0);
                    ui.image(image.source(&ctx));
                    ui.heading(RichText::new("Description:").font(FontId::monospace(30.0)));
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(RichText::new(&image_data.1).font(FontId::monospace(17.0)));
                    });
                });
            }); // APOD //

            egui::Window::new("Asteroids - NeoWs").show(ctx, |ui| { // NEOWS //
                egui::Frame::default().show(ui, |ui| {
                    ui.label("NEOWS!!");

                    ui.label("Start Date:");
                    ui.text_edit_singleline(&mut self.neows.start_date);

                    ui.label("End Date:");
                    ui.text_edit_singleline(&mut self.neows.end_date);
                }); // NEOWS //
            });
        });
    }
}
