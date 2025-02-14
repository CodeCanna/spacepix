use std::fmt::Debug;
use std::fmt::Display;
use std::usize;

// NASA API URLs
const APOD: &str = "https://api.nasa.gov/planetary/apod?api_key=";
const NEOWS: &str = "https://api.nasa.gov/neo/rest/v1/feed?start_date=START_DATE&end_date=END_DATE&api_key=";
const DONKI: &str = "https://api.nasa.gov/DONKI/CME?startDate=yyyy-MM-dd&endDate=yyyy-MM-dd&api_key=";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Urls {
    pub apod: String,
    pub neows: String,
    pub donki: String,
}

impl Default for Urls {
    fn default() -> Self {
        Self {
            apod: String::from(APOD),
            neows: String::from(NEOWS),
            donki: String::from(DONKI)
        }
    }
}

impl Display for Urls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
