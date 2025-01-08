use crate::Urls;
use crate::errors::ApiKeyError;
use json::object;
use std::{fs, path::Path};
use std::io::{Read, Write};


#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Parser {
    pub urls: Urls,
    key: String,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            urls: Urls::default(),
            key: String::from("DEMO_KEY"),
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
                let key_json = json::from(key);
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
}

#[cfg(test)]
mod tests {
    fn test_apod_url() {
        todo!()
    }

    fn test_neows_url() {
        todo!()
    }
}
