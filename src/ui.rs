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