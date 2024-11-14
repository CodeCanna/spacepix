use std::default;

use clap::builder::Str;
use json::object;

use crate::{errors::NetworkError, Parser, Urls};

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ApiKey {
    pub key: String,
}

impl ApiKey {
    pub fn new(&self, k: &str) -> Self {
        Self { key: k.to_string() }
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
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

    pub fn get_apod_data_blocking(&self) /*-> Result<Self, NetworkError>*/ {
        match reqwest::blocking::get(Parser::default().apod_url()) {
            Ok(r) => {
                // let json_obj = object! {
                //     date: r["date"],
                //     explanation: r["explanation"],
                //     hdurl: r["hdurl"],
                //     media_type: r["media_type"],
                //     service_version: r["service_version"],
                //     title: r["title"],
                //     url: r["url"]
                // };

                // Self {
                //     date: json_obj["date"],
                // }
                dbg!(r.text());
            }
            Err(_) => (),
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

    pub fn get_neows_data_blocking(&self) {
        todo!()
    }
}

#[allow(dead_code)]
struct DONKI {}

#[cfg(test)]
mod tests {
    use super::Apod;

    #[test]
    fn test_get_apod_data_blocking() {
        let test_apod = Apod::default();

        dbg!(test_apod.get_apod_data_blocking());
    }
}
