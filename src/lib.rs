mod app;
mod apis;
mod urls;
mod errors;
pub use app::SpacePixUi;
pub use urls::Urls;
pub use apis::*;

mod json_objects {
    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
    pub struct NearEarthObject {
        pub asteroid_id: String,
        pub name: String,
        pub estimagted_diameter: (String, String), // (min, max)
        pub is_potentially_hazardous_asteroid: bool,
        pub close_approach_date: String,
        pub close_approach_time: String,
        pub relative_velocity: String,
        pub miss_distance: String,
        pub orbiting_body: String
    }

    impl NearEarthObject {
        pub fn new(
            asteroid_id: String,
            name: String,
            estimagted_diameter: (String, String), // (min, max)
            is_potentially_hazardous_asteroid: bool,
            close_approach_date: String,
            close_approach_time: String,
            relative_velocity: String,
            miss_distance: String,
            orbiting_body: String
        ) -> Self {
            Self {
                asteroid_id,
                name,
                estimagted_diameter,
                is_potentially_hazardous_asteroid,
                close_approach_date,
                close_approach_time,
                relative_velocity,
                miss_distance,
                orbiting_body
            }
        }
    }
}