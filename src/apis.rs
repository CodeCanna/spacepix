use json::object;

use crate::{errors::NetworkError, Parser};

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ApiKey {
    pub key: String,
}
// beans
impl ApiKey {
    pub fn new(&self, k: &str) -> Self {
        Self { key: k.to_string() }
    }
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct Apod {
    date: String,
    explanation: String,
    hdurl: String,
    media_type: String,
    service_version: String,
    title: String,
    url: String,
}

impl Default for Apod {
    fn default() -> Self {
        Self {
            date: String::default(),
            explanation: String::default(),
            hdurl: String::default(),
            media_type: String::default(),
            service_version: String::default(),
            title: String::default(),
            url: String::default(),
        }
    }
}

impl Apod {
    pub fn new(
        &self,
        date: String,
        explanation: String,
        hdurl: String,
        media_type: String,
        service_version: String,
        title: String,
        url: String,
    ) -> Self {
        Self {
            date: date,
            explanation: explanation,
            hdurl: hdurl,
            media_type: media_type,
            service_version: service_version,
            title: title,
            url: url,
        }
    }

    pub fn get_apod_data_blocking(&self) -> Result<Self, NetworkError> {
        match reqwest::blocking::get(Parser::default().apod_url()) {
            Ok(r) => match json::parse(r.text().unwrap().as_str()) {
                Ok(j) => {
                    let json_obj = object! {
                        date: j["date"].clone(),
                        explanation: j["explanation"].clone(),
                        hdurl: j["hdurl"].clone(),
                        media_type: j["media_type"].clone(),
                        service_version: j["service_version"].clone(),
                        title: j["title"].clone(),
                        url: j["url"].clone()
                    };

                    Ok(Self {
                        date: json_obj["date"].to_string(),
                        explanation: json_obj["explanation"].to_string(),
                        hdurl: json_obj["hdurl"].to_string(),
                        media_type: json_obj["media_type"].to_string(),
                        service_version: json_obj["service_version"].to_string(),
                        title: json_obj["title"].to_string(),
                        url: json_obj["url"].to_string()
                    })
                },
                Err(e) => return Err(NetworkError::JsonParseFailed(e)),
            }
            Err(e) => return Err(NetworkError::ConnectionFailed(e))
        }
    }
}

pub struct Links {
    next: String,
    previous: String,
    current: String
}

impl Links {
    pub fn new(next: String, previous: String, current: String) -> Links {
        Self {
            next,
            previous,
            current
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NearEarthObject {
    pub asteroid_id: String,
    pub name: String,
    pub estimated_diameter: (String, String), // (min, max)
    pub is_potentially_hazardous_asteroid: bool,
    pub close_approach_date: String,
    pub close_approach_time: String,
    pub relative_velocity: String,
    pub miss_distance: String,
    pub orbiting_body: String,
}

impl Default for NearEarthObject {
    fn default() -> Self {
        Self {
            asteroid_id: String::default(),
            name: String::default(),
            estimated_diameter: (String::default(), String::default()),
            is_potentially_hazardous_asteroid: bool::default(),
            close_approach_date: String::default(),
            close_approach_time: String::default(),
            relative_velocity: String::default(),
            miss_distance: String::default(),
            orbiting_body: String::default()
        }
    }
}

impl NearEarthObject {
    pub fn new(
        asteroid_id: String,
        name: String,
        estimated_diameter: (String, String), // (min, max)
        is_potentially_hazardous_asteroid: bool,
        close_approach_date: String,
        close_approach_time: String,
        relative_velocity: String,
        miss_distance: String,
        orbiting_body: String,
    ) -> Self {
        Self {
            asteroid_id,
            name,
            estimated_diameter,
            is_potentially_hazardous_asteroid,
            close_approach_date,
            close_approach_time,
            relative_velocity,
            miss_distance,
            orbiting_body,
        }
    }

    // Get near earth object feed by date range
    pub fn get_neows_feed(&self, start_date: &str, end_date: &str) {}

    // Get unique near earth object by it's id
    pub fn get_neows_by_id(&self, id: &str){}
}

// pub struct NearEarthObjectFeed {
//     links: Links,
//     element_count: u8,
//     near_earth_objects: Vec<NearEarthObject>
// }
//
// impl NearEarthObjectFeed {
//     pub fn new(links: Links, element_count: u8, near_earth_objects: Vec<NearEarthObject>) -> Links{
//         Self {
//             links,
//             element_count,
//             near_earth_objects
//         }
//     }
// }

#[allow(dead_code)]
struct DONKI {}

#[cfg(test)]
mod tests {
    use super::Apod;

    #[test]
    fn test_get_apod_data_blocking() {
        todo!()
    }
}
