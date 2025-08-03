// src/logging.rs

// Importar el crate log para usar sus macros subyacentes
// Esto asume que tienes "log" y "env_logger" en tu Cargo.toml
// [dependencies]
// log = "0.4"
// env_logger = "0.11"

pub struct Logger;

impl Logger {
    /// Inicializa el sistema de logging usando env_logger.
    /// Debe llamarse una vez al inicio del programa.
    pub fn init() {
        let _ = env_logger::builder().format_timestamp(None).try_init();
    }

    /// Muestra un mensaje de depuración si el nivel de log lo permite.
    pub fn log_debug(message: &str) {
        log::debug!("{message}");
    }

    /// Muestra un mensaje informativo.
    pub fn log_info(message: &str) {
        log::info!("{message}");
    }

    /// Muestra una advertencia.
    pub fn log_warn(message: &str) {
        log::warn!("{message}");
    }

    /// Muestra un mensaje de error.
    pub fn log_error(message: &str) {
        log::error!("{message}");
    }
}
