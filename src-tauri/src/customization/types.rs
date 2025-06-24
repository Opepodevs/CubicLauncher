use serde::{Deserialize, Serialize};

pub enum ThemeTones {
    Dark,
    Light,
}

pub struct Manifest {
    manifest_version: u8,
    pub theme_tone: ThemeTones,
    pub author: String,
}

pub struct ColorFile {}
