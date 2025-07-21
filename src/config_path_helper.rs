use crate::config::AppConfig;
use std::path::PathBuf;
use nannou::App;

/// Devuelve el path absoluto a config.toml desde el contexto de la app
pub fn get_config_path(app: &App) -> PathBuf {
    app.project_path().unwrap().join("config.toml")
}

/// Carga la configuración desde config.toml usando el contexto de la app
pub fn load_config(app: &App) -> AppConfig {
    let config_path = get_config_path(app);
    AppConfig::load_or_default(config_path)
}
