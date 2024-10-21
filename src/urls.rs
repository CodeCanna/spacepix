#[derive(Clone)]
pub struct Urls {
    pub apod: String,
    pub neows: String,
    pub donki: String
}

impl Default for Urls {
    fn default() -> Self {
        Self {
            apod: String::from("https://api.nasa.gov/planetary/apod?api_key=dHoShRN6KfreJoy5hrA946bifL6tb3amyotVyAQt"),
            neows: String::from("https://api.nasa.gov/neo/rest/v1/feed?start_date=START_DATE&end_date=END_DATE&api_key=dHoShRN6KfreJoy5hrA946bifL6tb3amyotVyAQt"),
            donki: String::from("https://api.nasa.gov/DONKI/CME?startDate=yyyy-MM-dd&endDate=yyyy-MM-dd&api_key=dHoShRN6KfreJoy5hrA946bifL6tb3amyotVyAQt")
        }
    }
}