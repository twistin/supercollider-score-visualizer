// src/gui.rs - Interfaz gráfica básica para SC Score Visualizer
use crate::config::AppConfig;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GuiError {
    message: String,
}

impl fmt::Display for GuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GUI Error: {}", self.message)
    }
}

impl Error for GuiError {}

impl GuiError {
    pub fn new(message: &str) -> Self {
        GuiError {
            message: message.to_string(),
        }
    }
}

/// Función principal para ejecutar la GUI
pub fn run_gui() -> Result<(), GuiError> {
    log::info!("Iniciando SC Score Visualizer GUI v2.0...");
    
    println!("🎨 SC Score Visualizer GUI v2.0");
    println!("═══════════════════════════════════════");
    println!("Esta es una interfaz de control básica.");
    println!("Para el visualizador completo, ejecute:");
    println!("  cargo run --bin sc_score_visualizer");
    println!("═══════════════════════════════════════");
    
    // Cargar y mostrar configuración
    let config = AppConfig::load_or_default("config.toml");
    match config.validate() {
        Ok(_) => {
            println!("✅ Configuración cargada correctamente");
            println!("📡 OSC: {}:{}", config.osc.listen_host, config.osc.listen_port);
            println!("🖼️  Ventana: {}x{}", config.window.width, config.window.height);
        }
        Err(e) => {
            println!("❌ Error en configuración: {}", e);
            return Err(GuiError::new(&format!("Error en configuración: {}", e)));
        }
    }
    
    println!("\n🎯 Presione Enter para continuar o Ctrl+C para salir...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).map_err(|e| {
        GuiError::new(&format!("Error leyendo entrada: {}", e))
    })?;
    
    println!("✅ GUI básica ejecutada correctamente.");
    println!("💡 Para futuras versiones se implementará una GUI completa con egui/iced.");
    
    Ok(())
}
