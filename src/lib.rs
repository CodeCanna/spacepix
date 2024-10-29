mod app;
mod apis;
mod urls;
mod errors;
pub use app::SpacePixUi;
pub use urls::Urls;
pub use apis::*;

mod json_objects {
    pub struct NearEarthObject {
        asteroid_id: String,
        name: String,
        estimagted_diameter: (f32, f32), // (min, max)
        is_potentially_hazardous_asteroid: bool,
        close_approach_date: time::Date,
        close_approach_time: time::Time,
        relative_velocity: f32,
        miss_distance: f64,
        orbiting_body: String
    }
}