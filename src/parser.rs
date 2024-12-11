use crate::Urls;

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
    pub fn new(key: &str) -> Self {
        Self {
            urls: Urls::default(),
            key: key.to_string(),
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
