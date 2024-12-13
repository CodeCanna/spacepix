use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Failed to connect to NASA API")]
    ConnectionFailed(#[from] reqwest::Error),
    #[error("Failed to parse APOD image JSON")]
    JsonParseFailed(#[from] json::Error)
}

#[derive(Error, Debug)]
pub enum NeowsError {
    #[error("Bad Request")]
    BadRequest(#[from] reqwest::Error)
}

#[derive(Error, Debug)]
pub enum ApiKeyError {
    #[error("Invalid API key")]
    InvalidApiKey(),
    #[error("Spacepix either can't find or can't access the API key file")]
    KeyFile(#[from] std::io::Error)
}