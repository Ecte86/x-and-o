use std::fmt::Display;

use anyhow::anyhow;

#[derive(Debug)]
pub(crate) enum InputError {
    InvalidInput,
    InvalidCoordinates,
}

// Compare InputError with anyhow::Error
impl From<InputError> for anyhow::Error {
    fn from(val: InputError) -> Self {
        match val {
            InputError::InvalidInput => anyhow!("Invalid input"),
            InputError::InvalidCoordinates => anyhow!("Invalid coordinates"),
        }
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::InvalidInput => write!(f, "Invalid input"),
            InputError::InvalidCoordinates => write!(f, "Invalid coordinates"),
        }
    }
}
