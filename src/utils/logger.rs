// 📝 Sistema de logging simplificado usando env_logger

use log::LevelFilter;
use env_logger::Builder;
use crate::config::AppConfig;

/// Inicializa el sistema de logging con configuración simple
pub fn init_logging(config: &AppConfig) -> anyhow::Result<()> {
    // Crear filtro de nivel basado en configuración
    let level = match config.logging.level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info, // Por defecto
    };
    
    let mut builder = Builder::from_default_env();
    builder
        .filter_level(level)
        .filter_module("sc_score_visualizer", level)
        .init();
    
    log::info!("✅ Sistema de logging inicializado con nivel: {:?}", level);
    
    Ok(())
}

/// Función simple para inicializar logging sin configuración
pub fn init_simple_logging() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();
}