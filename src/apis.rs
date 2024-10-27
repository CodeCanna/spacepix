#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct Apod {
    pub data: Option<(String, String)>,
    pub cache: Option<(String, String)>
}

impl Default for Apod {
    fn default() -> Self {
        Self {
            data: None,
            cache: None
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct NEOWS {
    pub start_date: String,
    pub end_date: String
}

impl Default for NEOWS {
    fn default() -> Self {
        Self {
            start_date: String::default(),
            end_date: String::default()
        }
    }
}

#[allow(dead_code)]
struct DONKI {}