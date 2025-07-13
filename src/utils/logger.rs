// 📝 Sistema de logging profesional con configuración avanzada
// Configuración de tracing con niveles y salidas personalizables

use tracing::{info};
use tracing_subscriber::{
    fmt::{self},
    prelude::*,
    EnvFilter,
};
use crate::config::AppConfig;
use std::fs::File;

/// Inicializa el sistema de logging con configuración avanzada
pub fn init_logging(config: &AppConfig) -> anyhow::Result<()> {
    // Crear filtro de nivel basado en configuración
    let level = match config.performance.log_level.to_lowercase().as_str() {
        "error" => "error",
        "warn" => "warn", 
        "info" => "info",
        "debug" => "debug",
        "trace" => "trace",
        _ => "info", // Por defecto
    };
    
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("sc_score_visualizer={}", level)));

    let subscriber = tracing_subscriber::registry()
        .with(filter);

    // Configurar salida a consola
    let console_layer = fmt::layer()
        .with_target(config.development.verbose_logging)
        .with_thread_ids(config.development.debug_mode)
        .with_file(config.development.debug_mode)
        .with_line_number(config.development.debug_mode)
        .compact();

    // Si está habilitado logging a archivo, agregarlo
    if config.performance.log_to_file {
        // Crear directorio de logs si no existe
        if let Some(parent) = std::path::Path::new(&config.performance.log_file_path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| anyhow::anyhow!("Error creando directorio de logs: {}", e))?;
        }
        
        let file = File::create(&config.performance.log_file_path)
            .map_err(|e| anyhow::anyhow!("Error creando archivo de log: {}", e))?;
        
        let file_layer = fmt::layer()
            .with_writer(file)
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true);
        
        tracing::subscriber::set_global_default(
            subscriber
                .with(console_layer)
                .with(file_layer)
        )
        .map_err(|e| anyhow::anyhow!("Failed to set global logger: {}", e))?;
    } else {
        tracing::subscriber::set_global_default(
            subscriber.with(console_layer)
        )
        .map_err(|e| anyhow::anyhow!("Failed to set global logger: {}", e))?;
    }

    info!("🔧 Sistema de logging avanzado inicializado (nivel: {})", level);
    if config.performance.log_to_file {
        info!("📄 Logging a archivo habilitado: {}", config.performance.log_file_path);
    }
    
    Ok(())
}
