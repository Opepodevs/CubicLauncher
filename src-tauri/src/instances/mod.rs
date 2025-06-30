use bincode::decode_from_slice;
use bincode::{config::standard, encode_to_vec, Decode, Encode};
use serde::{Deserialize, Serialize};
use tauri::command;
use tokio::fs::{read, read_dir, write};
use tracing::{error, info, warn};

use crate::types::BackendResponse;
use crate::paths::{CONFIG_DIR, DATA_DIR};

#[derive(Debug, Encode, Decode, Deserialize, Serialize, Clone)]
pub enum Loaders {
    Forge,
    Fabric,
    Quilt,
    Vanilla,
}

#[derive(Debug, Encode, Decode, Deserialize, Serialize, Clone)]
pub struct Instance {
    name: String,
    loader: Loaders,
    version: String,
    custom_args: Vec<String>,
    downloaded: bool,
}

impl Instance {
    pub fn new(
        name: String,
        loader: Loaders,
        version: String,
        custom_args: Vec<String>,
    ) -> Instance {
        Instance {
            name,
            loader,
            version,
            custom_args,
            downloaded: false,
        }
    }

    pub async fn write_instance(&self) -> Result<BackendResponse, BackendResponse> {
        // Log para debugging - verificar qué estamos encodindo
        info!("Intentando encodear instancia: {:?}", self.name);

        match encode_to_vec(&self, standard()) {
            Ok(encoded) => {
                let dir_path = DATA_DIR.join(&self.name);
                let file_path = dir_path.join("instance.cin");
                // Crear el directorio si no existe
                if let Err(e) = tokio::fs::create_dir_all(&dir_path).await {
                    error!("Error creando directorio {:?}: {:?}", dir_path, e);
                    return Err(BackendResponse::error(
                        crate::types::CubicInternalError::FileError,
                        Some(format!("Failed to create directory: {}", e)),
                    ));
                }

                match write(&file_path, &encoded).await {
                    Ok(_) => Ok(BackendResponse::success(
                        crate::types::ResponseData::InstanceData(encoded),
                    )),
                    Err(e) => {
                        error!("Error escribiendo archivo: {:?}", e);
                        Err(BackendResponse::error(
                            crate::types::CubicInternalError::FileError,
                            Some(e.to_string()),
                        ))
                    }
                }
            }
            Err(encode_err) => {
                // Log detallado del error para debugging
                error!("Error al encodear instancia - Debug: {:?}", encode_err);
                error!("Error al encodear instancia - Display: {}", encode_err);

                // Intentar diferentes formas de convertir el error a string
                let error_msg = format!("Encoding failed: {:?}", encode_err);

                Err(BackendResponse::error(
                    crate::types::CubicInternalError::InstanceEncodeError,
                    Some(error_msg),
                ))
            }
        }
    }
}

#[command]
pub async fn save_instance(instance: Instance) -> Result<BackendResponse, BackendResponse> {
    let response = instance.write_instance().await?;
    println!("{:?}", response);
    Ok(response)
}

#[command]
pub async fn get_instances() -> Result<BackendResponse, BackendResponse> {
    let mut instances = Vec::new();
    
    // Leer el directorio CONFIG_DIR
    let mut dir_entries = match read_dir(&*DATA_DIR).await {
        Ok(entries) => entries,
        Err(e) => {
            error!("Error leyendo directorio de configuración: {:?}", e);
            return Err(BackendResponse::error(
                crate::types::CubicInternalError::FileError,
                Some(format!("Failed to read config directory: {}", e)),
            ));
        }
    };

    // Iterar sobre cada entrada del directorio
    while let Some(entry) = dir_entries.next_entry().await.map_err(|e| {
        error!("Error iterando directorio: {:?}", e);
        BackendResponse::error(
            crate::types::CubicInternalError::FileError,
            Some(format!("Failed to iterate directory: {}", e)),
        )
    })? {
        let path = entry.path();
        
        // Solo procesar directorios
        if path.is_dir() {
            let instance_file = path.join("instance.cin");
            
            // Verificar si existe el archivo instance.cin
            if instance_file.exists() {
                match read(&instance_file).await {
                    Ok(data) => {
                        // Decodificar la instancia
                        match decode_from_slice::<Instance, _>(&data, standard()) {
                            Ok((instance, _)) => {
                                info!("Instancia cargada: {:?}", instance.name);
                                instances.push(instance);
                            }
                            Err(e) => {
                                error!("Error decodificando instancia en {:?}: {:?}", instance_file, e);
                                // Continuar con la siguiente instancia en lugar de fallar completamente
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error leyendo archivo {:?}: {:?}", instance_file, e);
                        // Continuar con la siguiente instancia
                        continue;
                    }
                }
            }
        }
    }

    Ok(BackendResponse::success(
        crate::types::ResponseData::InstancesVec(instances),
    ))
}