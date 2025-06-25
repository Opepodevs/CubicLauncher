// use tauri::command;

// use crate::customization::themes::writter::create_theme;
// use crate::customization::types::{
//     AccentColors, BaseColors, BorderColors, ButtonColors, ButtonState, ColorPalette, Manifest,
//     TextColors, ThemeTone,
// };
// use crate::error::CubicInternalError;
// use std::path::{Path, PathBuf};

// #[command]
// pub fn create_example_theme() -> Result<(), CubicInternalError> {
//     let manifest = create_example_manifest("niggerson", "nigger", ThemeTone::Dark);
//     let palette = create_example_palette(&ThemeTone::Dark);

//     create_theme(
//         PathBuf::from("/home/santiagolxx/noir.cbth"),
//         Some(&manifest),
//         Some(&palette),
//         Some(PathBuf::from("/home/santiagolxx/xd.png")), // Sin imagen de fondo en el ejemplo
//     )
// }

// /// Crea un manifest de ejemplo
// fn create_example_manifest(name: &str, author: &str, tone: ThemeTone) -> Manifest {
//     Manifest {
//         manifest_version: 1,
//         inherits_from: None,
//         theme_tone: tone,
//         author: author.to_string(),
//         name: name.to_string(),
//         description: Some("Tema de ejemplo generado automáticamente".to_string()),
//     }
// }

// /// Crea una paleta de colores de ejemplo basada en el tono
// fn create_example_palette(tone: &ThemeTone) -> ColorPalette {
//     match tone {
//         ThemeTone::Dark => create_dark_palette(),
//         ThemeTone::Light => create_light_palette(),
//     }
// }

// pub fn create_dark_palette() -> ColorPalette {
//     ColorPalette {
//         base: BaseColors {
//             background: [18, 18, 18, 255],      // Gris muy oscuro
//             logo: [255, 255, 255, 255],         // Blanco
//             surface: [33, 33, 33, 255],         // Gris oscuro
//             surface_variant: [48, 48, 48, 255], // Gris medio oscuro
//         },
//         text: TextColors {
//             primary: [255, 255, 255, 255],    // Blanco
//             secondary: [189, 189, 189, 255],  // Gris claro
//             disabled: [117, 117, 117, 255],   // Gris medio
//             on_surface: [222, 222, 222, 255], // Gris muy claro
//         },
//         accent: AccentColors {
//             error: [244, 67, 54, 255],    // Rojo
//             warning: [255, 152, 0, 255],  // Naranja
//             success: [76, 175, 80, 255],  // Verde
//             info: [33, 150, 243, 255],    // Azul
//             primary: [156, 39, 176, 255], // Púrpura
//         },
//         border: BorderColors {
//             default: [66, 66, 66, 255], // Gris
//             subtle: [48, 48, 48, 255],  // Gris sutil
//             focus: [156, 39, 176, 255], // Púrpura (mismo que primary)
//         },
//         button: ButtonColors {
//             primary: ButtonState {
//                 base: [156, 39, 176, 255],   // Púrpura
//                 hover: [171, 71, 188, 255],  // Púrpura más claro
//                 active: [123, 31, 162, 255], // Púrpura más oscuro
//                 disabled: [66, 66, 66, 255], // Gris
//             },
//             secondary: ButtonState {
//                 base: [48, 48, 48, 255],     // Gris oscuro
//                 hover: [66, 66, 66, 255],    // Gris medio
//                 active: [33, 33, 33, 255],   // Gris más oscuro
//                 disabled: [33, 33, 33, 128], // Gris transparente
//             },
//             accent: ButtonState {
//                 base: [33, 150, 243, 255],   // Azul
//                 hover: [66, 165, 245, 255],  // Azul más claro
//                 active: [25, 118, 210, 255], // Azul más oscuro
//                 disabled: [66, 66, 66, 255], // Gris
//             },
//         },
//     }
// }

// /// Paleta clara de ejemplo
// fn create_light_palette() -> ColorPalette {
//     ColorPalette {
//         base: BaseColors {
//             background: [255, 255, 255, 255],      // Blanco
//             logo: [33, 33, 33, 255],               // Gris muy oscuro
//             surface: [248, 249, 250, 255],         // Gris muy claro
//             surface_variant: [241, 243, 244, 255], // Gris claro
//         },
//         text: TextColors {
//             primary: [33, 33, 33, 255],     // Gris muy oscuro
//             secondary: [97, 97, 97, 255],   // Gris medio
//             disabled: [158, 158, 158, 255], // Gris claro
//             on_surface: [66, 66, 66, 255],  // Gris oscuro
//         },
//         accent: AccentColors {
//             error: [211, 47, 47, 255],    // Rojo
//             warning: [245, 124, 0, 255],  // Naranja
//             success: [56, 142, 60, 255],  // Verde
//             info: [25, 118, 210, 255],    // Azul
//             primary: [123, 31, 162, 255], // Púrpura
//         },
//         border: BorderColors {
//             default: [224, 224, 224, 255], // Gris claro
//             subtle: [245, 245, 245, 255],  // Gris muy claro
//             focus: [123, 31, 162, 255],    // Púrpura (mismo que primary)
//         },
//         button: ButtonColors {
//             primary: ButtonState {
//                 base: [123, 31, 162, 255],      // Púrpura
//                 hover: [106, 27, 154, 255],     // Púrpura más oscuro
//                 active: [74, 20, 140, 255],     // Púrpura muy oscuro
//                 disabled: [224, 224, 224, 255], // Gris claro
//             },
//             secondary: ButtonState {
//                 base: [245, 245, 245, 255],     // Gris muy claro
//                 hover: [238, 238, 238, 255],    // Gris claro
//                 active: [224, 224, 224, 255],   // Gris medio claro
//                 disabled: [250, 250, 250, 128], // Gris muy claro transparente
//             },
//             accent: ButtonState {
//                 base: [25, 118, 210, 255],      // Azul
//                 hover: [21, 101, 192, 255],     // Azul más oscuro
//                 active: [13, 71, 161, 255],     // Azul muy oscuro
//                 disabled: [224, 224, 224, 255], // Gris claro
//             },
//         },
//     }
// }

// src-tauri/src/commands/theme.rs
use crate::customization::theme_loader::{
    load_cubic_theme, load_theme_essentials, load_theme_file_by_type, read_theme_info,
    verify_theme_integrity, LoadedTheme, ThemeFileInfo,
};
use crate::customization::types::{ColorPalette, CubicThemeHeader, CubicThemeTableFile, Manifest};
use crate::error::CubicInternalError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::command;

// Estructuras serializables para el frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableLoadedTheme {
    pub manifest: Option<Manifest>,
    pub palette: Option<ColorPalette>,
    pub background: Option<Vec<u8>>,
    pub header: CubicThemeHeader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableThemeFileInfo {
    pub file_type: String, // Convertimos enum a string para JSON
    pub offset: u32,
    pub size: u32,
    pub checksum: u32,
}

impl From<LoadedTheme> for SerializableLoadedTheme {
    fn from(theme: LoadedTheme) -> Self {
        Self {
            manifest: theme.manifest,
            palette: theme.palette,
            background: theme.background,
            header: theme.header,
        }
    }
}

impl From<ThemeFileInfo> for SerializableThemeFileInfo {
    fn from(info: ThemeFileInfo) -> Self {
        let file_type = match info.file_type {
            CubicThemeTableFile::Background => "background".to_string(),
            CubicThemeTableFile::Palette => "palette".to_string(),
            CubicThemeTableFile::Manifest => "manifest".to_string(),
        };

        Self {
            file_type,
            offset: info.offset,
            size: info.size,
            checksum: info.checksum,
        }
    }
}

/// Carga un tema completo
#[command]
pub async fn load_theme(path: String) -> Result<SerializableLoadedTheme, String> {
    let theme_path = PathBuf::from(path);

    match load_cubic_theme(theme_path) {
        Ok(theme) => Ok(theme.into()),
        Err(e) => Err(format!("Error cargando tema: {:?}", e)),
    }
}

/// Lee información del tema sin cargar el contenido
#[command]
pub async fn get_theme_info(
    path: String,
) -> Result<(CubicThemeHeader, Vec<SerializableThemeFileInfo>), String> {
    let theme_path = PathBuf::from(path);

    match read_theme_info(theme_path) {
        Ok((header, file_infos)) => {
            let serializable_infos: Vec<SerializableThemeFileInfo> =
                file_infos.into_iter().map(|info| info.into()).collect();
            Ok((header, serializable_infos))
        }
        Err(e) => Err(format!("Error leyendo información del tema: {:?}", e)),
    }
}

/// Carga solo manifest y paleta
#[command]
pub async fn load_theme_essentials(
    path: String,
) -> Result<(Option<Manifest>, Option<ColorPalette>), String> {
    let theme_path = PathBuf::from(path);

    match load_theme_essentials(theme_path) {
        Ok(essentials) => Ok(essentials),
        Err(e) => Err(format!(
            "Error cargando elementos esenciales del tema: {:?}",
            e
        )),
    }
}

/// Carga un archivo específico del tema
#[command]
pub async fn load_theme_file(path: String, file_type: String) -> Result<Option<Vec<u8>>, String> {
    let theme_path = PathBuf::from(path);

    let cubic_file_type = match file_type.as_str() {
        "background" => CubicThemeTableFile::Background,
        "palette" => CubicThemeTableFile::Palette,
        "manifest" => CubicThemeTableFile::Manifest,
        _ => return Err("Tipo de archivo no válido".to_string()),
    };

    match load_theme_file_by_type(theme_path, cubic_file_type) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Error cargando archivo del tema: {:?}", e)),
    }
}

/// Verifica la integridad del tema
#[command]
pub async fn verify_theme(path: String) -> Result<bool, String> {
    let theme_path = PathBuf::from(path);

    match verify_theme_integrity(theme_path) {
        Ok(is_valid) => Ok(is_valid),
        Err(e) => Err(format!("Error verificando tema: {:?}", e)),
    }
}

/// Comando para abrir un diálogo de selección de archivos .cbth
#[command]
pub async fn select_theme_file(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri::api::dialog::FileDialogBuilder;

    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new()
        .add_filter("Cubic Theme Files", &["cbth"])
        .pick_file(move |file_path| {
            tx.send(file_path).unwrap();
        });

    match rx.recv() {
        Ok(Some(path)) => Ok(Some(path.to_string_lossy().to_string())),
        Ok(None) => Ok(None),
        Err(_) => Err("Error en el diálogo de selección de archivos".to_string()),
    }
}
