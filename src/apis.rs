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
    pub copyright: String,
    pub date: String,
    pub explanation: String,
    pub hdurl: String,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String,
}

impl Default for Apod {
    fn default() -> Self {
        Self {
            copyright: String::default(),
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
        copyright: String,
        date: String,
        explanation: String,
        hdurl: String,
        media_type: String,
        service_version: String,
        title: String,
        url: String,
    ) -> Self {
        Self {
            copyright: copyright,
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
                Ok(json_obj) => {
                    Ok(Self {
                       copyright: json_obj["copyright"].to_string(),
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
#[derive(Debug, Clone, serde::Deserialize)]
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

// impl Default for NearEarthObject {
//     fn default() -> Self {
//         Self {
//             asteroid_id: String::default(),
//             name: String::default(),
//             estimated_diameter: (String::default(), String::default()),
//             is_potentially_hazardous_asteroid: bool::default(),
//             close_approach_date: String::default(),
//             close_approach_time: String::default(),
//             relative_velocity: String::default(),
//             miss_distance: String::default(),
//             orbiting_body: String::default()
//         }
//     }
// }
//
// impl NearEarthObject {
//     pub fn new(
//         asteroid_id: String,
//         name: String,
//         estimated_diameter: (String, String), // (min, max)
//         is_potentially_hazardous_asteroid: bool,
//         close_approach_date: String,
//         close_approach_time: String,
//         relative_velocity: String,
//         miss_distance: String,
//         orbiting_body: String,
//     ) -> Self {
//         Self {
//             asteroid_id,
//             name,
//             estimated_diameter,
//             is_potentially_hazardous_asteroid,
//             close_approach_date,
//             close_approach_time,
//             relative_velocity,
//             miss_distance,
//             orbiting_body,
//         }
//     }
//
//     // Get near earth object feed by date range
//     pub fn get_neows_feed(&self, date: str) Self {
//         match reqwest::blocking::get(Parser::default()::neows_url()) {
//             Ok(d) => {
//                 Self {
//
//                 }
//             }
//         }
//     }
//
//     // Get unique near earth object by it's id
//     pub fn get_neows_by_id(&self, id: &str) {}
// }

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

/**
 * Representative of a NearEarthObject from the NASA API
 * estimated_diameter tuple key ((feet_min, feet_max), (meters_min, meters_max))
 * relative_velocity tuple key (kilometers_per_second, kilometers_per_hour, miles_per_hour)
 * miss_distance tuple key (astronomical, lunar, kilometers, miles)
 */
pub struct NearEarthObject {
    id: String,
    neo_reference_id: String,
    name: String,
    estimated_diameter: ((u8, u8), (u8, u8)), // ((feet_min, feet_max), (meters_min, meters_max))
    close_approach_date: String,
    close_approach_date_full: String,
    epoch_date_close_approach: u64,
    relative_velocity: (String, String, String), // (kilometers_per_second, kilometers_per_hour, miles_per_hour)
    miss_distance: (String, String, String, String), // (astronomical, lunar, kilometers, miles)
    orbiting_body: String,
    is_sentry_object: bool
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NEOFeed {
    links: Links,
    element_count: u8,
    near_earth_objects: Vec<near_earth_object>
}

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
