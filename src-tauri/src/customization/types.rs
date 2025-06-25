use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub type Color = [u8; 4];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Encode, Decode)]
pub enum ThemeTone {
    Dark,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Manifest {
    pub manifest_version: u8,
    pub inherits_from: Option<String>,
    pub theme_tone: ThemeTone,
    pub author: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ColorPalette {
    pub base: BaseColors,
    pub text: TextColors,
    pub accent: AccentColors,
    pub border: BorderColors,
    pub button: ButtonColors,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct BaseColors {
    pub background: Color,
    pub logo: Color,
    pub surface: Color,
    pub surface_variant: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct TextColors {
    pub primary: Color,
    pub secondary: Color,
    pub disabled: Color,
    pub on_surface: Color, // Texto sobre superficies
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct AccentColors {
    pub error: Color,
    pub warning: Color,
    pub success: Color, // Corregido el typo "sucess"
    pub info: Color,
    pub primary: Color, // Color primario del tema
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct BorderColors {
    pub default: Color,
    pub subtle: Color,
    pub focus: Color, // Para elementos enfocados
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ButtonColors {
    pub primary: ButtonState,
    pub secondary: ButtonState,
    pub accent: ButtonState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ButtonState {
    pub base: Color,
    pub hover: Color,
    pub active: Color,
    pub disabled: Color,
}

pub struct CubicThemeFile {
    pub header: CubicThemeHeader,
    pub files: Vec<u8>,
}

/*
 * CBTH => CuBic THeme
 * Formato para temas de CubicLauncher
 *
 * [ARCHIVOS]:
 * - Limite de 1024 archivos
 * No creo que algun tema use mas de eso XD.
 * - Limite de 100MB por archivos
 * Tampoco creo que 100MB sea poco, es bastante razonable, de hecho es de mas.
 * - Versiones?
 * De 1 hasta 255.
 * TODO: Posiblemente agregar fuentes custom
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubicThemeTableFile {
    Background = 1,
    Palette = 2,
    Manifest = 3,
    // Espacio para futuros tipos de archivo (4-255)
}

#[derive(Debug, Clone, Copy)]
pub struct CubicThemeTableEntry {
    pub offset: u32,                    // Offset del archivo (little-endian)
    pub file_size: u32,                 // Tamaño en bytes (little-endian)
    pub file_type: CubicThemeTableFile, // Tipo de archivo (1 byte)
    pub _reserved: [u8; 2],             // Padding (debe ser cero, reservado para futuro)
    pub checksum: u32,                  // Blake3 hash truncado a 32 bits (little-endian)
}

#[derive(Debug, Clone, Copy)]
pub struct CubicThemeHeader {
    pub magic_bytes: [u8; 4], // Siempre b'C', b'B', b'T', b'H'
    pub version: u8,          // Versión del formato (actualmente 1)
    pub flags: u8,            // Flags globales (bit 0: compresión habilitada, bits 1-7: reservados)
    pub files_number: u16,    // Número de archivos (little-endian)
    pub table_offset: u32,    // Offset al inicio de la tabla (little-endian, típicamente 32)
    pub table_size: u32,      // Tamaño de la tabla en bytes (little-endian)
    pub total_size: u32,      // Tamaño total del archivo (little-endian)
    pub theme_checksum: u32,  // Blake3 del contenido completo (little-endian)
    pub _reserved: [u8; 8],   // Reservado para extensiones futuras (debe ser cero)
}
