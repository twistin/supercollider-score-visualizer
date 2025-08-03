use crate::config::AppConfig;
use std::path::PathBuf;
use nannou::App;

/// Obtiene la ruta absoluta al archivo `config.toml` utilizando el contexto de la aplicación Nannou.
/// Usa `app.project_path()` para localizar el directorio raíz del proyecto.
/// # Panics
/// La función hace `unwrap()` sobre `project_path`, por lo que puede fallar si el proyecto no está bien inicializado.
pub fn get_config_path(app: &App) -> PathBuf {
    app.project_path().unwrap().join("config.toml")
}

/// Carga la configuración de la aplicación desde el archivo `config.toml`.
/// Si el archivo no existe o contiene errores, se usa una configuración por defecto (`AppConfig::default()`).
/// Requiere acceso al objeto `App` para localizar el path del proyecto mediante `get_config_path`.
pub fn load_config(app: &App) -> AppConfig {
    let config_path = get_config_path(app);
    AppConfig::load_or_default(config_path)
}
