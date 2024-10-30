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
pub struct SecretSauceFileReadError {}
impl Error for SecretSauceFileReadError {}

impl Display for SecretSauceFileReadError {
 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     write!(f, "Failed to read Secret Sauce file!")
 }
}

#[derive(Debug)]
pub struct NeoWsInvalidDate {}
impl Error for NeoWsInvalidDate {}

impl Display for NeoWsInvalidDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid date detected for NeoWs.")
    }
}