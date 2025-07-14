// src/lib.rs - Biblioteca principal de SC Score Visualizer

pub mod model;
pub mod audio;
pub mod visual;
pub mod logging;
pub mod errors;
pub mod config;
pub mod osc_server;
pub mod midi;
pub mod gui;
pub mod utils;
pub mod musical_events;
pub mod core;

// Re-exportar tipos importantes para facilitar el uso
pub use model::{Model, Note, ScrollMode, DisplayMode, OscStats};
pub use config::AppConfig;
pub use osc_server::OscServer;
pub use visual::audio_visual_mapping::VisualNote;
pub use visual::audio_visual_mapping_pro::{ColorPalette, ProAudioVisualMapper};
pub use visual::shader_manager::ShaderManager;

// Función de conveniencia para inicializar el sistema completo
pub fn init_system() -> Result<(Model, AppConfig), Box<dyn std::error::Error>> {
    // Cargar configuración
    let config = AppConfig::load_or_default("config.toml");
    
    // Crear servidor OSC
    let osc_server = OscServer::new(config.osc.clone())?;
    
    // Crear modelo
    let model = Model::new_with_config(osc_server, config.clone());
    
    Ok((model, config))
}

// Función para inicializar logging
pub fn init_logging() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
}

// Metadatos de la biblioteca
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
