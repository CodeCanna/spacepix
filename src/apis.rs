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

#[allow(dead_code)]
struct NEOWS {}

#[allow(dead_code)]
struct DONKI {}