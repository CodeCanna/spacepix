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

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::*;

    #[test]
    fn test_make_secret_sauce() {
        match Urls::make_secret_sauce("TEST_KEY") {
            Ok(s) => {
                assert_eq!(s.apod, "https://api.nasa.gov/planetary/apod?api_key=TEST_KEY");
                assert_eq!(s.neows, "https://api.nasa.gov/neo/rest/v1/feed?start_date=START_DATE&end_date=END_DATE&api_key=TEST_KEY");
                assert_eq!(s.donki, "https://api.nasa.gov/DONKI/CME?startDate=yyyy-MM-dd&endDate=yyyy-MM-dd&api_key=TEST_KEY")
            },
            Err(_e) => {}
        }
    }

    #[test]
    fn test_get_secret_sauce() {
        // Create a test secret.json
        match File::create("secret.json") {
            Ok(mut f) => match f.write(String::from("{\"key\": \"TEST_KEY\"}").as_bytes()) {
                Ok(_) => match Urls::get_secret_sauce() {
                    Ok(s) => {
                        assert_eq!(s, "TEST_KEY");
                    },
                    Err(e) => panic!("{}", e)
                }
                Err(e) => panic!("{}", e)
            }
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn test_urls() {
        assert_eq!(Urls::urls().apod, "https://api.nasa.gov/planetary/apod?api_key=");
        assert_eq!(Urls::urls().neows, "https://api.nasa.gov/neo/rest/v1/feed?start_date=START_DATE&end_date=END_DATE&api_key=");
        assert_eq!(Urls::urls().donki, "https://api.nasa.gov/DONKI/CME?startDate=yyyy-MM-dd&endDate=yyyy-MM-dd&api_key=");
    }

    #[test]
    fn test_build_url_neows() {
        match Urls::build_url_neows(NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(), NaiveDate::from_ymd_opt(2024, 10, 7).unwrap()) {
            Ok(u) => {assert_eq!(u, "https://api.nasa.gov/neo/rest/v1/feed?start_date=2024-10-01&end_date=2024-10-07&api_key=TEST_KEY")},
            Err(_) => {}
        } 
    }
}
