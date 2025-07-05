use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, PartialEq)]
pub enum CubicError {
    // Errores de loaders
    #[error("Invalid loader")]
    InvalidLoaderValue(u8),
    #[error("Unsupported loader: {0}")]
    UnsupportedLoader(String),
}
