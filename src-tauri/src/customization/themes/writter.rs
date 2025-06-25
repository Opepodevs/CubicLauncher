use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::customization::types::{
    ColorPalette, CubicThemeHeader, CubicThemeTableEntry, CubicThemeTableFile, Manifest,
};
use crate::error::BackendResponse;
use crate::error::{CubicInternalError, ResponseData};
use blake3::hash;
use lz4_flex::compress_prepend_size;

/// Estructura que representa un archivo dentro del tema
#[derive(Debug, Clone)]
pub struct ThemeFile {
    pub file_type: CubicThemeTableFile,
    pub data: Vec<u8>,
}

impl CubicThemeHeader {
    const HEADER_SIZE: u32 = 32;
    const MAGIC_BYTES: [u8; 4] = [b'C', b'B', b'T', b'H'];
    const VERSION: u8 = 1;

    fn new(
        files_count: u16,
        table_size: u32,
        total_size: u32,
        theme_checksum: u32,
        compressed: bool,
    ) -> Self {
        Self {
            magic_bytes: Self::MAGIC_BYTES,
            version: Self::VERSION,
            flags: if compressed { 1 } else { 0 },
            files_number: files_count,
            table_offset: Self::HEADER_SIZE,
            table_size,
            total_size,
            theme_checksum,
            _reserved: [0; 8],
        }
    }

    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), CubicInternalError> {
        writer
            .write_all(&self.magic_bytes)
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&[self.version])
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&[self.flags])
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&self.files_number.to_le_bytes())
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&self.table_offset.to_le_bytes())
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&self.table_size.to_le_bytes())
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&self.total_size.to_le_bytes())
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&self.theme_checksum.to_le_bytes())
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&self._reserved)
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        Ok(())
    }
}

impl CubicThemeTableEntry {
    const ENTRY_SIZE: u32 = 13;

    fn new(offset: u32, file_size: u32, file_type: CubicThemeTableFile, checksum: u32) -> Self {
        Self {
            offset,
            file_size,
            file_type,
            _reserved: [0; 2],
            checksum,
        }
    }

    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), CubicInternalError> {
        writer
            .write_all(&self.offset.to_le_bytes())
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&self.file_size.to_le_bytes())
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&[self.file_type as u8])
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&self._reserved)
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        writer
            .write_all(&self.checksum.to_le_bytes())
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
        Ok(())
    }
}

fn calculate_checksum(data: &[u8]) -> u32 {
    u32::from_le_bytes(hash(&data).as_bytes()[0..4].try_into().unwrap())
}

/// Escribe un archivo de tema CubicTheme (.cbth)
pub fn write_cubic_theme<P: AsRef<Path>>(
    path: P,
    files: &[ThemeFile],
    compressed: bool,
) -> Result<(), CubicInternalError> {
    if files.is_empty() {
        return Err(CubicInternalError::ThemeEncodeError);
    }

    // Validar límites
    if files.len() > 1024 {
        return Err(CubicInternalError::SizeErrorLimit);
    }

    for file in files {
        if file.data.len() > 100 * 1024 * 1024 {
            // 100MB
            return Err(CubicInternalError::SizeErrorLimit);
        }
    }

    let mut output_file = File::create(path).map_err(|_| CubicInternalError::FileError)?;

    // Calcular dimensiones
    let header_size = CubicThemeHeader::HEADER_SIZE;
    let table_size = (files.len() as u32) * CubicThemeTableEntry::ENTRY_SIZE;
    let data_start_offset = header_size + table_size;

    // Construir tabla de archivos
    let mut table_entries = Vec::with_capacity(files.len());
    let mut current_offset = data_start_offset;
    let mut total_data_size = 0u32;

    for theme_file in files {
        let file_size = theme_file.data.len() as u32;
        let checksum = calculate_checksum(&theme_file.data);

        table_entries.push(CubicThemeTableEntry::new(
            current_offset,
            file_size,
            theme_file.file_type,
            checksum,
        ));

        current_offset += file_size;
        total_data_size += file_size;
    }

    let total_size = header_size + table_size + total_data_size;

    // Calcular checksum del tema completo
    let mut all_data = Vec::new();
    for theme_file in files {
        all_data.extend_from_slice(&theme_file.data);
    }
    let theme_checksum = calculate_checksum(&all_data);

    // Escribir header
    let header = CubicThemeHeader::new(
        files.len() as u16,
        table_size,
        total_size,
        theme_checksum,
        compressed,
    );
    header.write_to(&mut output_file)?;

    // Escribir tabla
    for entry in &table_entries {
        entry.write_to(&mut output_file)?;
    }

    // Escribir datos
    for theme_file in files {
        output_file
            .write_all(&theme_file.data)
            .map_err(|_| CubicInternalError::ThemeEncodeError)?;
    }

    output_file
        .flush()
        .map_err(|_| CubicInternalError::ThemeEncodeError)?;

    Ok(())
}

pub fn load_theme_file<P: AsRef<Path>>(
    path: P,
    file_type: CubicThemeTableFile,
) -> Result<ThemeFile, CubicInternalError> {
    let data = std::fs::read(path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => CubicInternalError::FileNotFound,
        io::ErrorKind::PermissionDenied => CubicInternalError::PermissionError,
        _ => CubicInternalError::FileError,
    })?;

    Ok(ThemeFile { file_type, data })
}

/// Serializa datos usando bincode para el archivo de tema
pub fn serialize_manifest(manifest: &Manifest) -> Result<Vec<u8>, CubicInternalError> {
    let encoded = bincode::encode_to_vec(manifest, bincode::config::standard())
        .map_err(|_| CubicInternalError::ThemeEncodeError)?;
    Ok(compress_prepend_size(&encoded))
}

pub fn serialize_palette(palette: &ColorPalette) -> Result<Vec<u8>, CubicInternalError> {
    let encoded = bincode::encode_to_vec(palette, bincode::config::standard())
        .map_err(|_| CubicInternalError::ThemeEncodeError)?;
    Ok(compress_prepend_size(&encoded))
}

/// Crea un tema completo desde componentes individuales
pub fn create_theme<P: AsRef<Path>>(
    output_path: P,
    manifest: Option<&Manifest>,
    palette: Option<&ColorPalette>,
    background_path: Option<&Path>,
    compressed: bool,
) -> Result<(), CubicInternalError> {
    let mut files = Vec::new();

    // Agregar manifest
    if let Some(manifest) = manifest {
        let data = serialize_manifest(manifest)?;
        files.push(ThemeFile {
            file_type: CubicThemeTableFile::Manifest,
            data,
        });
    }

    // Agregar paleta de colores
    if let Some(palette) = palette {
        let data = serialize_palette(palette)?;
        files.push(ThemeFile {
            file_type: CubicThemeTableFile::Palette,
            data,
        });
    }

    // Agregar imagen de fondo
    if let Some(bg_path) = background_path {
        files.push(load_theme_file(bg_path, CubicThemeTableFile::Background)?);
    }

    if files.is_empty() {
        return Err(CubicInternalError::ThemeEncodeError);
    }

    write_cubic_theme(output_path, &files, compressed)
}
