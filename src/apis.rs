use crate::{errors::NetworkError, Parser};

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

    pub fn get_apod_data_blocking() -> Result<Self, NetworkError> {
        match reqwest::blocking::get(Parser::default().apod_url().replace("\"", "")) { // .replace to get rid of the extra quotes from the URL
            Ok(r) => match json::parse(r.text().unwrap().as_str()) {
                Ok(json_obj) => Ok(Self {
                    copyright: json_obj["copyright"].to_string(),
                    date: json_obj["date"].to_string(),
                    explanation: json_obj["explanation"].to_string(),
                    hdurl: json_obj["hdurl"].to_string(),
                    media_type: json_obj["media_type"].to_string(),
                    service_version: json_obj["service_version"].to_string(),
                    title: json_obj["title"].to_string(),
                    url: json_obj["url"].to_string(),
                }),
                Err(e) => return Err(NetworkError::JsonParseFailed(e)),
            },
            Err(e) => return Err(NetworkError::ConnectionFailed(e)),
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Links {
    next: String,
    previous: String,
    current: String,
}

impl Links {
    pub fn new(next: String, previous: String, current: String) -> Links {
        Self {
            next,
            previous,
            current,
        }
    }
}

impl Default for Links {
    fn default() -> Self {
        Self {
            next: String::default(),
            previous: String::default(),
            current: String::default()
        }
    }
}

/**
 * Representative of a NearEarthObject from the NASA API
 * estimated_diameter tuple key ((feet_min, feet_max), (meters_min, meters_max))
 * relative_velocity tuple key (kilometers_per_second, kilometers_per_hour, miles_per_hour)
 * miss_distance tuple key (astronomical, lunar, kilometers, miles)
 */
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NearEarthObject {
    pub id: String,
    pub neo_reference_id: String,
    pub name: String,
    pub estimated_diameter: ((f32, f32), (f32, f32)), // ((feet_min, feet_max), (meters_min, meters_max))
    pub is_potentially_hazardous_asteroid: bool,
    pub close_approach_date: String,
    pub close_approach_date_full: String,
    pub epoch_date_close_approach: u64,
    pub relative_velocity: (String, String, String), // (kilometers_per_second, kilometers_per_hour, miles_per_hour)
    pub miss_distance: (String, String, String, String), // (astronomical, lunar, kilometers, miles)
    pub orbiting_body: String,
    pub is_sentry_object: bool,
}

impl NearEarthObject {
    pub fn new(
        id: String,
        neo_reference_id: String,
        name: String,
        estimated_diameter: ((f32, f32), (f32, f32)),
        is_potentially_hazardous_asteroid: bool,
        close_approach_date: String,
        close_approach_date_full: String,
        epoch_date_close_approach: u64,
        relative_velocity: (String, String, String),
        miss_distance: (String, String, String, String),
        orbiting_body: String,
        is_sentry_object: bool,
    ) -> Self {
        Self {
            id,
            neo_reference_id,
            name,
            estimated_diameter,
            is_potentially_hazardous_asteroid,
            close_approach_date,
            close_approach_date_full,
            epoch_date_close_approach,
            relative_velocity,
            miss_distance,
            orbiting_body,
            is_sentry_object,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NEOFeed {
    pub links: Links,
    pub element_count: u8,
    pub near_earth_objects: Vec<NearEarthObject>,
}

impl NEOFeed {
    pub fn new(
        links: Links,
        element_count: u8,
        near_earth_objects: Vec<NearEarthObject>
    ) -> Self {
        Self {
            links,
            element_count,
            near_earth_objects
        }
    }

    // Retrieve a list of Near Earth Objects from NASA and return a result with the list or error
    pub fn get_neows_feed_blocking(&mut self, date: &str) -> Result<&mut NEOFeed, NetworkError> {
        match reqwest::blocking::get(Parser::default().neows_url(date).replace("\"", "")) {
            Ok(r) => match json::parse(r.text().unwrap().as_str()) {
                Ok(json_obj) => {
                    // dbg!("{:?}", &json_obj);
                    let neo_objects_json = json_obj["near_earth_objects"][date].members();
                    let mut neo_vec: Vec<NearEarthObject> = Vec::default();
                    for object in neo_objects_json {
                        let neo = NearEarthObject::new(
                            object["id"].to_string(),
                            object["neo_reference_id"].to_string(),
                            object["name"].to_string(),
                            (
                                (
                                    object["estimated_diameter"]["feet"]["estimated_diameter_min"]
                                        .as_f32()
                                        .unwrap(),
                                    object["estimated_diameter"]["feet"]["estimated_diameter_max"]
                                        .as_f32()
                                        .unwrap(),
                                ),
                                (
                                    object["estimated_diameter"]["meters"]
                                        ["estimated_diameter_min"]
                                        .as_f32()
                                        .unwrap(),
                                    object["estimated_diameter"]["meters"]
                                        ["estimated_diameter_max"]
                                        .as_f32()
                                        .unwrap(),
                                ),
                            ), // ((feet_min, feet_max), (meters_min, meters_max))
                            object["is_potentially_hazardous_asteroid"]
                                .as_bool()
                                .unwrap(),
                            object["close_approach_data"][0]["close_approach_date"].to_string(),
                            object["close_approach_data"][0]["close_approach_date_full"]
                                .to_string(),
                            object["close_approach_data"][0]["epoch_date_close_approach"]
                                .as_u64()
                                .unwrap(),
                            (
                                object["close_approach_data"][0]["relative_velocity"]
                                    ["kilometers_per_second"]
                                    .to_string(),
                                object["close_approach_data"][0]["relative_velocity"]
                                    ["kilometers_per_hour"]
                                    .to_string(),
                                object["close_approach_data"][0]["relative_velocity"]
                                    ["miles_per_hour"]
                                    .to_string(),
                            ), // (kilometers_per_second, kilometers_per_hour, miles_per_hour)
                            (
                                object["close_approach_data"][0]["miss_distance"]["astronomical"]
                                    .to_string(),
                                object["close_approach_data"][0]["miss_distance"]["lunar"]
                                    .to_string(),
                                object["close_approach_data"][0]["miss_distance"]["kilometers"]
                                    .to_string(),
                                object["close_approach_data"][0]["miss_distance"]["miles"]
                                    .to_string(),
                            ), // (astronomical, lunar, kilometers, miles)
                            object["close_approach_data"][0]["orbiting_body"].to_string(),
                            object["is_sentry_object"].as_bool().unwrap(),
                        );
                        neo_vec.push(neo);
                    }

                    let links = Links::new(
                        json_obj["links"]["next"].to_string(),
                        json_obj["links"]["previous"].to_string(),
                        json_obj["links"]["self"].to_string(),
                    );
                    
                    self.links = links;
                    self.near_earth_objects = neo_vec;
                    Ok(self)
                }
                Err(e) => return Err(NetworkError::JsonParseFailed(e)),
            },
            Err(e) => return Err(NetworkError::ConnectionFailed(e)),
        }
    }
}

impl Default for NEOFeed {
    fn default() -> Self {
        Self {
            links: Links::default(),
            element_count: u8::default(),
            near_earth_objects: Vec::default()
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct NIVL {

}

impl Default for NIVL {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::Apod;

    #[test]
    fn test_get_apod_data_blocking() {
        todo!()
    }
}
