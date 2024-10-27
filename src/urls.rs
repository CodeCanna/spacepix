use std::fmt::Debug;
use std::io::Read;
use std::path::Path;
use std::fmt::Display;
use std::fs::File;
use json::JsonError;
use crate::errors::SecretSauceFileNotFoundError;

const SAUCE_PATH: &str = "secret.json";

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Urls {
    pub apod: String,
    pub neows: String,
    pub donki: String
}

impl Urls {
    pub fn make_secret_sauce(saucy_key: &str) -> Result<Urls, SecretSauceFileNotFoundError> {
        let mut sauce = Urls::urls();
        
        sauce.apod = format!("{}{}", Urls::urls().apod, saucy_key);
        sauce.neows = format!("{}{}", Urls::urls().neows, saucy_key);
        sauce.donki = format!("{}{}", Urls::urls().donki, saucy_key);

        Ok(sauce)
    }

    pub fn get_secret_sauce() -> Result<String, JsonError> {
        let path_to_the_sauce = match Path::new(SAUCE_PATH).exists() {
            true => Path::new(SAUCE_PATH),
            false => panic!("Missing secret sauce file!")
        };

        let mut saucy_file = match File::open(path_to_the_sauce) {
            Ok(f) => f,
            Err(f) => panic!("{}", f)
        };

        let mut saucy_string: String = String::from("");

        saucy_file.read_to_string(&mut saucy_string).unwrap();

        match json::parse(&saucy_string) {
            Ok(f) => Ok(f["key"].to_string()),
            Err(f) => Err(f)
        }
    }

    pub fn urls() -> Self {
        Self {
            apod: String::from("https://api.nasa.gov/planetary/apod?api_key="),
            neows: String::from("https://api.nasa.gov/neo/rest/v1/feed?start_date=START_DATE&end_date=END_DATE&api_key="),
            donki: String::from("https://api.nasa.gov/DONKI/CME?startDate=yyyy-MM-dd&endDate=yyyy-MM-dd&api_key=")
        }
    }
}

impl Display for Urls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}