use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct SecretSauceFileNotFoundError {}
impl Error for SecretSauceFileNotFoundError {}

impl Display for SecretSauceFileNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to find Secret Sauce file!")
    }
}

#[derive(Debug)]
pub struct FailedToCreateSecretSauce {}
impl Error for FailedToCreateSecretSauce {}

impl Display for FailedToCreateSecretSauce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to create secret.json file.")
    }
}

#[derive(Debug)]
pub struct FailedToGetSecretSauce {}
impl Error for FailedToGetSecretSauce {}

impl Display for FailedToGetSecretSauce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No API key found!")
    }
}

#[derive(Debug)]
pub struct FailedToGetDataApod {}
impl Error for FailedToGetDataApod {}

impl Display for FailedToGetDataApod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to get APOD data, check your connection.")
    }
}

#[derive(Debug)]
pub struct SecretSauceFileReadError {}
impl Error for SecretSauceFileReadError {}

impl Display for SecretSauceFileReadError {
 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     write!(f, "Failed to read Secret Sauce file!")
 }
}

#[derive(Debug)]
pub struct FailedToGetDataNeows {}
impl Error for FailedToGetDataNeows {}

impl Display for FailedToGetDataNeows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to retrieve data for NeoWs")
    }
}