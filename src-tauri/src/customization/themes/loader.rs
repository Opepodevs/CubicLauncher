use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

use crate::customization::types::{
    ColorPalette, CubicThemeHeader, CubicThemeTableEntry, CubicThemeTableFile, Manifest,
};
use crate::error::CubicInternalError;
use blake3::hash;
use lz4_flex::decompress_size_prepended;

/// Estructura que contiene el tema cargado con sus componentes
pub struct LoadedTheme {
    pub manifest: Option<Manifest>,
    pub palette: Option<ColorPalette>,
    pub background: Option<Vec<u8>>,
    pub header: CubicThemeHeader,
}

/// Información de un archivo dentro del tema
#[derive(Debug, Clone)]
pub struct ThemeFileInfo {
    pub file_type: CubicThemeTableFile,
    pub offset: u32,
    pub size: u32,
    pub checksum: u32,
}

impl CubicThemeHeader {
    /// Lee el header desde un archivo
    fn read_from<R: Read>(reader: &mut R) -> Result<Self, CubicInternalError> {
        let mut buffer = [0u8; 32];
        reader
            .read_exact(&mut buffer)
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        // Verificar magic bytes
        let magic_bytes = [b'C', b'B', b'T', b'H'];
        if &buffer[0..4] != &magic_bytes {
            return Err(CubicInternalError::ThemeDecodeError);
        }

        let version = buffer[4];
        if version != 1 {
            return Err(CubicInternalError::ThemeDecodeError);
        }

        Ok(Self {
            magic_bytes: [buffer[0], buffer[1], buffer[2], buffer[3]],
            version,
            flags: buffer[5],
            files_number: u16::from_le_bytes([buffer[6], buffer[7]]),
            table_offset: u32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]),
            table_size: u32::from_le_bytes([buffer[12], buffer[13], buffer[14], buffer[15]]),
            total_size: u32::from_le_bytes([buffer[16], buffer[17], buffer[18], buffer[19]]),
            theme_checksum: u32::from_le_bytes([buffer[20], buffer[21], buffer[22], buffer[23]]),
            _reserved: [
                buffer[24], buffer[25], buffer[26], buffer[27], buffer[28], buffer[29], buffer[30],
                buffer[31],
            ],
        })
    }

    /// Verifica si el tema está comprimido
    pub fn is_compressed(&self) -> bool {
        self.flags & 1 != 0
    }
}

impl CubicThemeTableEntry {
    /// Lee una entrada de la tabla desde un archivo
    fn read_from<R: Read>(reader: &mut R) -> Result<Self, CubicInternalError> {
        let mut buffer = [0u8; 15];
        reader
            .read_exact(&mut buffer)
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        let file_type = match buffer[8] {
            1 => CubicThemeTableFile::Background,
            2 => CubicThemeTableFile::Palette,
            3 => CubicThemeTableFile::Manifest,
            _ => return Err(CubicInternalError::ThemeDecodeError),
        };

        Ok(Self {
            offset: u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]),
            file_size: u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
            file_type,
            _reserved: [buffer[9], buffer[10]],
            checksum: u32::from_le_bytes([buffer[11], buffer[12], buffer[13], buffer[14]]),
        })
    }
}

fn calculate_checksum(data: &[u8]) -> u32 {
    u32::from_le_bytes(hash(data).as_bytes()[0..4].try_into().unwrap())
}

/// Deserializa un manifest desde datos comprimidos
fn deserialize_manifest(data: &[u8]) -> Result<Manifest, CubicInternalError> {
    let decompressed =
        decompress_size_prepended(data).map_err(|_| CubicInternalError::ThemeDecodeError)?;

    let (manifest, _): (Manifest, usize) =
        bincode::decode_from_slice(&decompressed, bincode::config::standard())
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

    Ok(manifest)
}

/// Deserializa una paleta de colores desde datos comprimidos
fn deserialize_palette(data: &[u8]) -> Result<ColorPalette, CubicInternalError> {
    let decompressed =
        decompress_size_prepended(data).map_err(|_| CubicInternalError::ThemeDecodeError)?;

    let (palette, _): (ColorPalette, usize) =
        bincode::decode_from_slice(&decompressed, bincode::config::standard())
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

    Ok(palette)
}

/// Lee la información de los archivos del tema sin cargar el contenido
pub fn read_theme_info<P: AsRef<Path>>(
    path: P,
) -> Result<(CubicThemeHeader, Vec<ThemeFileInfo>), CubicInternalError> {
    let mut file = File::open(path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => CubicInternalError::FileNotFound,
        io::ErrorKind::PermissionDenied => CubicInternalError::PermissionError,
        _ => CubicInternalError::FileError,
    })?;

    // Leer header
    let header = CubicThemeHeader::read_from(&mut file)?;

    // Validar que el archivo tenga el tamaño correcto
    let file_size = file
        .metadata()
        .map_err(|_| CubicInternalError::FileError)?
        .len() as u32;

    if file_size != header.total_size {
        return Err(CubicInternalError::ThemeDecodeError);
    }

    // Ir a la tabla de archivos
    file.seek(SeekFrom::Start(header.table_offset as u64))
        .map_err(|_| CubicInternalError::ThemeDecodeError)?;

    // Leer entradas de la tabla
    let mut file_infos = Vec::with_capacity(header.files_number as usize);
    for _ in 0..header.files_number {
        let entry = CubicThemeTableEntry::read_from(&mut file)?;
        file_infos.push(ThemeFileInfo {
            file_type: entry.file_type,
            offset: entry.offset,
            size: entry.file_size,
            checksum: entry.checksum,
        });
    }

    Ok((header, file_infos))
}

/// Carga un archivo específico del tema
pub fn load_theme_file_by_type<P: AsRef<Path>>(
    path: P,
    file_type: CubicThemeTableFile,
) -> Result<Option<Vec<u8>>, CubicInternalError> {
    let (header, file_infos) = read_theme_info(&path)?;

    // Buscar el archivo del tipo solicitado
    let file_info = file_infos.iter().find(|info| info.file_type == file_type);
    let Some(file_info) = file_info else {
        return Ok(None);
    };

    let mut file = File::open(path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => CubicInternalError::FileNotFound,
        io::ErrorKind::PermissionDenied => CubicInternalError::PermissionError,
        _ => CubicInternalError::FileError,
    })?;

    // Ir al offset del archivo
    file.seek(SeekFrom::Start(file_info.offset as u64))
        .map_err(|_| CubicInternalError::ThemeDecodeError)?;

    // Leer datos del archivo
    let mut data = vec![0u8; file_info.size as usize];
    file.read_exact(&mut data)
        .map_err(|_| CubicInternalError::ThemeDecodeError)?;

    // Verificar checksum
    let calculated_checksum = calculate_checksum(&data);
    if calculated_checksum != file_info.checksum {
        return Err(CubicInternalError::ThemeDecodeError);
    }

    Ok(Some(data))
}

/// Carga un tema completo desde un archivo .cbth
pub fn load_cubic_theme<P: AsRef<Path>>(path: P) -> Result<LoadedTheme, CubicInternalError> {
    let (header, file_infos) = read_theme_info(&path)?;

    let mut file = File::open(&path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => CubicInternalError::FileNotFound,
        io::ErrorKind::PermissionDenied => CubicInternalError::PermissionError,
        _ => CubicInternalError::FileError,
    })?;

    let mut manifest = None;
    let mut palette = None;
    let mut background = None;

    // Cargar cada archivo
    for file_info in &file_infos {
        // Ir al offset del archivo
        file.seek(SeekFrom::Start(file_info.offset as u64))
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        // Leer datos
        let mut data = vec![0u8; file_info.size as usize];
        file.read_exact(&mut data)
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        // Verificar checksum
        let calculated_checksum = calculate_checksum(&data);
        if calculated_checksum != file_info.checksum {
            return Err(CubicInternalError::ThemeDecodeError);
        }

        // Procesar según el tipo de archivo
        match file_info.file_type {
            CubicThemeTableFile::Manifest => {
                manifest = Some(deserialize_manifest(&data)?);
            }
            CubicThemeTableFile::Palette => {
                palette = Some(deserialize_palette(&data)?);
            }
            CubicThemeTableFile::Background => {
                background = Some(data);
            }
        }
    }

    // Verificar checksum del tema completo
    let mut all_data = Vec::new();
    for file_info in &file_infos {
        file.seek(SeekFrom::Start(file_info.offset as u64))
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        let mut data = vec![0u8; file_info.size as usize];
        file.read_exact(&mut data)
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        all_data.extend_from_slice(&data);
    }

    let calculated_theme_checksum = calculate_checksum(&all_data);
    if calculated_theme_checksum != header.theme_checksum {
        return Err(CubicInternalError::ThemeDecodeError);
    }

    Ok(LoadedTheme {
        manifest,
        palette,
        background,
        header,
    })
}

/// Función de conveniencia para cargar solo la paleta y el manifest
pub fn load_theme_essentials<P: AsRef<Path>>(
    path: P,
) -> Result<(Option<Manifest>, Option<ColorPalette>), CubicInternalError> {
    let theme = load_cubic_theme(path)?;
    Ok((theme.manifest, theme.palette))
}

/// Verifica la integridad de un archivo de tema sin cargarlo completamente
pub fn verify_theme_integrity<P: AsRef<Path>>(path: P) -> Result<bool, CubicInternalError> {
    let (header, file_infos) = read_theme_info(&path)?;

    let mut file = File::open(&path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => CubicInternalError::FileNotFound,
        io::ErrorKind::PermissionDenied => CubicInternalError::PermissionError,
        _ => CubicInternalError::FileError,
    })?;

    // Verificar checksums individuales
    for file_info in &file_infos {
        file.seek(SeekFrom::Start(file_info.offset as u64))
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        let mut data = vec![0u8; file_info.size as usize];
        file.read_exact(&mut data)
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        let calculated_checksum = calculate_checksum(&data);
        if calculated_checksum != file_info.checksum {
            return Ok(false);
        }
    }

    // Verificar checksum del tema completo
    let mut all_data = Vec::new();
    for file_info in &file_infos {
        file.seek(SeekFrom::Start(file_info.offset as u64))
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        let mut data = vec![0u8; file_info.size as usize];
        file.read_exact(&mut data)
            .map_err(|_| CubicInternalError::ThemeDecodeError)?;

        all_data.extend_from_slice(&data);
    }

    let calculated_theme_checksum = calculate_checksum(&all_data);
    Ok(calculated_theme_checksum == header.theme_checksum)
}
