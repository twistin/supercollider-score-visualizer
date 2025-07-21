// src/logging.rs

// Importar el crate log para usar sus macros subyacentes
// Esto asume que tienes "log" y "env_logger" en tu Cargo.toml
// [dependencies]
// log = "0.4"
// env_logger = "0.11"

pub struct Logger;

impl Logger {
    pub fn log_debug(message: &str) {
        log::debug!("{}", message);
    }

    pub fn log_info(message: &str) {
        log::info!("{}", message);
    }

    pub fn log_warn(message: &str) {
        log::warn!("{}", message);
    }

    pub fn log_error(message: &str) {
        log::error!("{}", message);
    }
}
