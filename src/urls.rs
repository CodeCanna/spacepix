use crate::errors::{FailedToGetDataNeows, FailedToGetSecretSauce, SecretSauceFileNotFoundError};
use chrono::{Local, NaiveDate};
use std::fmt::Debug;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::usize;

const SAUCE_PATH: &str = "secret.json";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Urls {
    pub apod: String,
    pub neows: String,
    pub donki: String,
}

impl Urls {
    pub fn make_secret_sauce(saucy_key: &str) -> Result<Urls, SecretSauceFileNotFoundError> {
        let mut sauce = Urls::urls();

        sauce.apod = format!("{}{}", Urls::urls().apod, saucy_key);
        sauce.neows = format!("{}{}", Urls::urls().neows, saucy_key);
        sauce.donki = format!("{}{}", Urls::urls().donki, saucy_key);

        Ok(sauce)
    }

    pub fn get_secret_sauce() -> Result<String, FailedToGetSecretSauce> {
        let mut saucy_file = match File::open(Path::new(SAUCE_PATH)) {
            Ok(f) => f,
            Err(_) => {return Err(FailedToGetSecretSauce{})}
        };

        let mut sauce = String::default();
        let _ = saucy_file.read_to_string(&mut sauce).or(Err(FailedToGetSecretSauce{}));

        match json::parse(&sauce) {
            Ok(s) => Ok(s["key"].to_string()),
            Err(_) => {return Err(FailedToGetSecretSauce{})}
        }
    }

    pub fn urls() -> Self {
        Self {
            apod: String::from("https://api.nasa.gov/planetary/apod?api_key="),
            neows: String::from("https://api.nasa.gov/neo/rest/v1/feed?start_date=START_DATE&end_date=END_DATE&api_key="),
            donki: String::from("https://api.nasa.gov/DONKI/CME?startDate=yyyy-MM-dd&endDate=yyyy-MM-dd&api_key=")
        }
    }

    pub fn build_url_neows(
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<String, FailedToGetDataNeows> {
        let sauce = Urls::get_secret_sauce().expect("Failed to get secret.");
        // Get current date
        let current_date: NaiveDate = Local::now().date_naive();
        if start_date > current_date || end_date > current_date {
            return Err(FailedToGetDataNeows {});
        }

        let url: String = Urls::urls().neows;
        let url: String = url
            .replace("START_DATE", &start_date.to_string().as_str())
            .replace("END_DATE", &end_date.to_string().as_str());
        let url = format!("{}{}", url, sauce);
        // Return
        Ok(url)
    }
}

impl Display for Urls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
