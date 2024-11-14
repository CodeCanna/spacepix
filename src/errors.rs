use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Failed to connect to NASA API")]
    ConnectionFailed(#[from] reqwest::Error),
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
    #[error("Failed to save API key")]
    KeyStore(#[from] std::io::Error)
}