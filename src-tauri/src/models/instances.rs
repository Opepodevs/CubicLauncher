// [CubicLauncher]
// src/models/instances.rs
// Modelos para las instancias y sus conversiones.
// Autor: Santiagolxx
use crate::models::errors::CubicError;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum Loader {
    Vanilla = 0,
    Forge = 1,
    Quilt = 2,
    Fabric = 3,
}

pub struct Instance<'a> {
    pub name: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub version_type: Loader,
}

impl<'a> Instance<'a> {
    // Constructor que acepta referencias
    pub fn new(name: &'a str, version: &'a str, version_type: Loader) -> Self {
        Self {
            name: Cow::Borrowed(name),
            version: Cow::Borrowed(version),
            version_type,
        }
    }

    // Constructor que acepta String owned
    pub fn new_owned(name: String, version: String, version_type: Loader) -> Self {
        Self {
            name: Cow::Owned(name),
            version: Cow::Owned(version),
            version_type,
        }
    }

    // Método para convertir a owned
    pub fn into_owned(self) -> Instance<'static> {
        Instance {
            name: Cow::Owned(self.name.into_owned()),
            version: Cow::Owned(self.version.into_owned()),
            version_type: self.version_type,
        }
    }
}

impl TryFrom<u8> for Loader {
    type Error = CubicError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Loader::Vanilla),
            1 => Ok(Loader::Forge),
            2 => Ok(Loader::Quilt),
            3 => Ok(Loader::Fabric),
            _ => Err(CubicError::InvalidLoaderValue(value)),
        }
    }
}

impl From<Loader> for u8 {
    fn from(loader: Loader) -> Self {
        loader as u8
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_conversion() {
        assert_eq!(Loader::try_from(0), Ok(Loader::Vanilla));
        assert_eq!(Loader::try_from(1), Ok(Loader::Forge));
        assert_eq!(Loader::try_from(2), Ok(Loader::Quilt));
        assert_eq!(Loader::try_from(3), Ok(Loader::Fabric));

        // Test invalid conversion
        match Loader::try_from(255) {
            Err(CubicError::InvalidLoaderValue(255)) => (),
            _ => panic!("Expected InvalidLoaderValue error"),
        }
    }

    #[test]
    fn test_instance_creation() {
        let instance = Instance::new("Instancia123", "1.20.1", Loader::Fabric);
        assert_eq!(instance.name, "Instancia123");
        assert_eq!(instance.version, "1.20.1");
        assert_eq!(instance.version_type, Loader::Fabric);
    }
}
