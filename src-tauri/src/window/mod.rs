use crate::types::{BackendResponse, CubicInternalError, ResponseData, WindowActionResult};
use tauri::{command, Window};
use tracing::{info, warn};

#[command]
pub async fn minimize_window(window: Window) -> Result<BackendResponse, BackendResponse> {
    info!("Intentando minimizar la ventana");

    // Verificamos si se puede minimizar
    let is_minimizable = window.is_minimizable().map_err(|_| {
        warn!("No se pudo verificar si la ventana es minimizable");
        BackendResponse::error(
            CubicInternalError::WindowMinimizeError,
            Some("No se pudo verificar si la ventana es minimizable.".into()),
        )
    })?;

    if !is_minimizable {
        warn!("La ventana no es minimizable");
        return Err(BackendResponse::error(
            CubicInternalError::WindowIsNotMinimizable,
            Some("La ventana no puede minimizarse.".into()),
        ));
    }

    // Intentamos minimizar
    window.minimize().map_err(|_| {
        warn!("Falló al intentar minimizar la ventana");
        BackendResponse::error(
            CubicInternalError::WindowMinimizeError,
            Some("Falló al minimizar la ventana.".into()),
        )
    })?;

    info!("Ventana minimizada con éxito");
    Ok(BackendResponse::success(ResponseData::WindowAction(
        WindowActionResult::MinimizeSuccess,
    )))
}

#[command]
pub async fn maximize_window(window: Window) -> Result<BackendResponse, BackendResponse> {
    info!("Intentando maximizar/desmaximizar la ventana");

    // Verificamos si se puede maximizar
    let is_maximizable = window.is_maximizable().map_err(|_| {
        warn!("No se pudo verificar si la ventana es maximizable");
        BackendResponse::error(
            CubicInternalError::WindowMaximizeError,
            Some("No se pudo verificar si la ventana es maximizable.".into()),
        )
    })?;

    let is_maximized = window.is_maximized().map_err(|_| {
        warn!("No se pudo verificar si la ventana está maximizada");
        BackendResponse::error(
            CubicInternalError::WindowMaximizeError,
            Some("No se pudo verificar si la ventana está maximizada.".into()),
        )
    })?;

    if !is_maximizable {
        warn!("Se intentó maximizar la ventana, pero no es posible");
        return Err(BackendResponse::error(
            CubicInternalError::WindowIsNotMaximizable,
            Some("La ventana no puede maximizarse.".into()),
        ));
    }

    // Maximizar o desmaximizar según el estado actual
    if !is_maximized {
        info!("Maximizando la ventana");
        window.maximize().map_err(|_| {
            warn!("Falló al intentar maximizar la ventana");
            BackendResponse::error(
                CubicInternalError::WindowMaximizeError,
                Some("Falló al maximizar la ventana.".into()),
            )
        })?;
    } else {
        info!("Desmaximizando la ventana");
        window.unmaximize().map_err(|_| {
            warn!("Falló al intentar desmaximizar la ventana");
            BackendResponse::error(
                CubicInternalError::WindowMaximizeError,
                Some("Falló al desmaximizar la ventana.".into()),
            )
        })?;
    }

    info!("Acción de maximizar/desmaximizar completada con éxito");
    Ok(BackendResponse::success(ResponseData::WindowAction(
        WindowActionResult::MaximizeSuccess,
    )))
}

#[command]
pub async fn close_window(window: Window) -> Result<BackendResponse, BackendResponse> {
    info!("Intentando cerrar la ventana");

    // Verificamos si se puede cerrar
    let is_closeable = window.is_closable().map_err(|_| {
        warn!("No se pudo verificar si la ventana es cerrable");
        BackendResponse::error(
            CubicInternalError::WindowCloseError,
            Some("No se pudo verificar si la ventana es cerrable.".into()),
        )
    })?;

    if !is_closeable {
        warn!("Se intentó cerrar la ventana, pero no puede cerrarse");
        return Err(BackendResponse::error(
            CubicInternalError::WindowIsNotClosable,
            Some("La ventana no puede cerrarse.".into()),
        ));
    }

    // Cerramos la ventana
    window.close().map_err(|_| {
        warn!("Falló al intentar cerrar la ventana");
        BackendResponse::error(
            CubicInternalError::WindowCloseError,
            Some("Falló al cerrar la ventana.".into()),
        )
    })?;

    info!("Ventana cerrada con éxito");
    Ok(BackendResponse::success(ResponseData::WindowAction(
        WindowActionResult::CloseSuccess,
    )))
}
