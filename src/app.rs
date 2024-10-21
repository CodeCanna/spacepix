use crate::Urls;
use eframe::egui::{Color32, FontId, Id, RichText};

const URL: &str =
    "https://api.nasa.gov/planetary/apod?api_key=dHoShRN6KfreJoy5hrA946bifL6tb3amyotVyAQt";

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
enum Views {
    APOD,
    NEOWS,
    DONKI,
}
// This is the object that the view port will represent
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct SpacePixUi {
    current_view: Views,
    image_url: String,
    image_desc: String,
}

impl Default for SpacePixUi {
    fn default() -> Self {
        Self {
            current_view: Views::APOD,
            image_url: String::from("Beans"), // This should point to some default logo file for now
            image_desc: String::from("Beans"), // This can say something like: "Welcome to spacepix!"
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

    pub fn load_apod_view(&mut self) {
        println!("We are currently in APOD view");
    }

    pub fn load_neows_view(&mut self) {
        println!("We are currently in NEOWS view");
    }

    pub fn load_donki_view(&mut self) {
        println!("We are in DONKI view");
    }

    #[allow(dead_code)]
    pub async fn get_pic_data() -> Result<(String, String), reqwest::Error> {
        let data = reqwest::get(
            "https://api.nasa.gov/planetary/apod?api_key=dHoShRN6KfreJoy5hrA946bifL6tb3amyotVyAQt",
        )
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

    pub fn get_pic_data_blocking() -> Result<(String, String), reqwest::Error> {
        let data = reqwest::blocking::get(URL)?
            .text()
            .expect("Failed to retrieve image from API...");

        let json_object = json::parse(&data).expect("Failed to parse image data...");
        let image_data: (String, String) = (
            json_object["hdurl"].to_string(),
            json_object["explanation"].to_string(),
        );

        Ok(image_data)
    }
}

impl eframe::App for SpacePixUi {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let image_data = self::SpacePixUi::get_pic_data_blocking().expect("Failed to get image...");
        self.image_desc = image_data.1;
        self.image_url = image_data.0;
        let image = egui::Image::from_uri(self.image_url.clone()); // I had to clone here for some reason...

        egui_extras::install_image_loaders(&ctx);
        match &self.current_view {
            Views::APOD => SpacePixUi::load_apod_view(self),
            Views::NEOWS => SpacePixUi::load_neows_view(self),
            Views::DONKI => SpacePixUi::load_donki_view(self),
        };

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

                ui.menu_button("Views", |ui| {
                    if ui.button("APOD").clicked() {
                        println!("Show pic of day view");
                        if self.current_view != Views::APOD {
                            self.current_view = Views::APOD;
                        }
                    }

                    if ui.button("Asteroids - NeoWs").clicked() {
                        if self.current_view != Views::NEOWS {
                            self.current_view = Views::NEOWS;
                        }
                    }

                    if ui.button("DONKI").clicked() {
                        if self.current_view != Views::DONKI {
                            self.current_view = Views::DONKI;
                        }
                    }
                });
            });
        });

        egui::SidePanel::new(egui::panel::Side::Left, Id::new("panel-left")).show(ctx, |ui| {
            ui.add_space(40.0);
            if ui.button("Previous").clicked() {
                println!("Previous Image");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                //ui.image(image.source(&ctx));
                ui.image(
                    egui::Image::new(image.source(&ctx))
                        .shrink_to_fit()
                        .source(&ctx),
                );
            });
        });

        egui::TopBottomPanel::bottom(Id::new("description")).show(ctx, |ui| {
            ui.heading(RichText::new("Description:").font(FontId::monospace(30.0)));
            ui.separator();
            ui.label(RichText::new(&self.image_desc).font(FontId::monospace(17.0)));
            ui.separator();
        });
    }
}
