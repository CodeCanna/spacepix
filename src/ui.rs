use egui::Image;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ApodWindow {
    pub apod_window_visible: bool,
    pub apod_full_window_visible: bool,
}

impl Default for ApodWindow {
    fn default() -> Self {
        Self {
            apod_window_visible: false,
            apod_full_window_visible: false
        }
    }
}

impl ApodWindow {
    pub fn apod_full_window(
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
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct NeowsWindow {
    pub neows_date: String,
    pub neows_invalid_input_window_visible: bool,
    pub neows_window_visible: bool
}

impl Default for NeowsWindow {
    fn default() -> Self {
        Self {
            neows_date: String::default(),
            neows_invalid_input_window_visible: false,
            neows_window_visible: false
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct NIVLWindow {
    pub query: String,
    pub nivl_window_visible: bool
}

impl Default for NIVLWindow {
    fn default() -> Self {
        Self {
            query: String::default(),
            nivl_window_visible: false
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AboutWindow {
    pub about_window_visible: bool,
}

impl Default for AboutWindow {
    fn default() -> Self {
        Self {
            about_window_visible: false
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiKeyWindow {
    pub api_key_window_visible: bool,
    pub key: String,
    pub key_set_label: String
}

impl Default for ApiKeyWindow {
    fn default() -> Self {
        Self {
            api_key_window_visible: false,
            key: String::default(),
            key_set_label: String::default()
        }
    }
}
