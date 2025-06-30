use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::instances::Instance;

//
// === RESPUESTA GENÉRICA ===
//

/// Respuesta estándar del backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendResponse {
    pub success: bool,
    pub error: Option<ClientError>,
    pub data: Option<ResponseData>,
}

impl BackendResponse {
    /// Crea una respuesta exitosa con datos
    pub fn success(data: ResponseData) -> Self {
        Self {
            success: true,
            error: None,
            data: Some(data),
        }
    }

    /// Crea una respuesta de error
    pub fn error(error_type: CubicInternalError, message: Option<String>) -> Self {
        Self {
            success: false,
            error: Some(ClientError {
                error_type,
                error_message: message,
            }),
            data: None,
        }
    }
}

/// Error enviado al cliente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientError {
    pub error_type: CubicInternalError,
    pub error_message: Option<String>,
}

/// Tipos de datos que puede retornar el backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseData {
    MinecraftVersions(Vec<String>),
    Settings(Vec<String>),
    Instances(Vec<String>),
    WindowAction(WindowActionResult),
    
    // Internal
    InstanceData(Vec<u8>),
    InstancesVec(Vec<Instance>)
}

/// Resultado de acciones sobre ventanas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowActionResult {
    MinimizeSuccess,
    MaximizeSuccess,
    CloseSuccess,
}

//
// === ERRORES INTERNOS ===
//

/// Errores internos del sistema Cubic
#[derive(Debug, Error, Serialize, Deserialize, Clone)]
pub enum CubicInternalError {
    #[error("No se pudo minimizar la ventana")]
    WindowMinimizeError,

    #[error("La ventana no puede minimizarse")]
    WindowIsNotMinimizable,

    #[error("La ventana no puede maximizarse")]
    WindowIsNotMaximizable,

    #[error("No se pudo maximizar la ventana")]
    WindowMaximizeError,

    #[error("La ventana no puede cerrarse")]
    WindowIsNotClosable,

    #[error("No se pudo cerrar la ventana")]
    WindowCloseError,

    #[error("Error general del launcher")]
    LauncherError,

    #[error("Error de configuración")]
    ConfigError,

    #[error("Error de instancia de Minecraft")]
    MinecraftInstanceError,

    #[error("Error de red")]
    NetworkError,

    #[error("Error de IO")]
    FileError,

    #[error("Error de permisos")]
    PermissionError,

    #[error("Error de serialización/deserialización de instancia")]
    InstanceEncodeError,

    #[error("Loader de mods invalido.")]
    InvalidLoader
}