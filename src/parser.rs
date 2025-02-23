use crate::Urls;
use crate::errors::ApiKeyError;
use json::object;
use std::{fs, path::Path};
use std::io::{Read, Write};


#[derive(Clone, serde::Serialize, serde::Deserialize, std::fmt::Debug)]
pub struct Parser {
    pub urls: Urls,
    key: String,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            urls: Urls::default(),
            key: {
                // println!("{}", Self::read_key_file(fs::File::open("secret.json").unwrap()).unwrap());
                Self::read_key_file(fs::File::open("secret.json").unwrap()).unwrap()
            },
        }
    }
}

impl Parser {
    pub fn new(key: String) -> Self {
        Self {
            urls: Urls::default(),
            key,
        }
    }

    pub fn set_api_key(&mut self, secret_path: &Path, key: String) -> Result<(), ApiKeyError> {
        match fs::File::create(secret_path) {
            Ok(mut f) => {
                let json_buff = object! {key: key};
                let _ = f.write(json_buff.to_string().as_bytes());
                Ok(())
            }
            Err(e) => Err(ApiKeyError::KeyFile(e)),
        }
    }

    pub fn read_key_file(mut file: fs::File) -> Result<String, ApiKeyError> {
        let mut key = String::default();
        match file.read_to_string(&mut key) {
            Ok(_) => {
                // let key_json = json::from(key);
                let key_json: serde_json::Value = serde_json::from_str(&key.as_str()).unwrap();
                Ok(key_json["key"].to_string())
            },
            Err(e) => Err(ApiKeyError::KeyFile(e))
        }
    }

    pub fn apod_url(&self) -> String {
        format!("{}{}", self.urls.apod, self.key)
    }

    pub fn neows_url(&self, date: &str) -> String {
        format!(
            "{}{}",
            self.urls
                .neows
                .replace("START_DATE", date)
                .replace("END_DATE", date),
            self.key
        )
    }

    pub fn get_api_key(&self) -> String {
        self.key.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn test_apod_url() {
        assert_eq!(Parser::new(String::from("DEMO_KEY")).apod_url(), "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY");
    }

    #[test]
    fn test_neows_url() {
        assert_eq!(Parser::new(String::from("DEMO_KEY")).neows_url("2020-10-10"), "https://api.nasa.gov/neo/rest/v1/feed?start_date=2020-10-10&end_date=2020-10-10&api_key=DEMO_KEY");
    }
}
