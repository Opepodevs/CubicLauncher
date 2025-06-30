use directories::ProjectDirs;
use once_cell::sync::Lazy;
use std::path::PathBuf;

const QUALIFIER: &str = "me";
const ORGANIZATION: &str = "cubicmc";
const APPLICATION: &str = "phalcon";

// ProjectDirs global, inicializado una sola vez
pub static PROJECT_DIRS: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .expect("No se pudieron obtener los directorios del proyecto")
});

// Rutas específicas
pub static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| PROJECT_DIRS.config_dir().to_path_buf());
pub static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| PROJECT_DIRS.data_dir().to_path_buf());
pub static CACHE_DIR: Lazy<PathBuf> = Lazy::new(|| PROJECT_DIRS.cache_dir().to_path_buf());