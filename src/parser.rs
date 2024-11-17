use crate::Urls;

pub struct Parser {
    pub urls: Urls,
    key: String,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            urls: Urls::default(),
            key: String::from("DEMO_KEY")
        }
    }
}

impl Parser {
    pub fn new(key: &str) -> Self {
        Self {
            urls: Urls::default(),
            key: key.to_string(),
        }
    }

    pub fn apod_url(&self) -> String {
        format!("{}{}", self.urls.apod, self.key)
    }

    fn neows_url(&self, start_date: String, end_date: String) -> String {
        self.urls
            .neows
            .replace("START_DATE", &start_date.to_string().as_str())
            .replace("END_DATE", &end_date.to_string().as_str())
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
