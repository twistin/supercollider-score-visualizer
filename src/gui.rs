// src/gui.rs - Interfaz gráfica básica para SC Score Visualizer
use crate::config::AppConfig;
use std::error::Error;
use std::fmt;
use std::path::Path;

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

impl From<std::io::Error> for GuiError {
    fn from(e: std::io::Error) -> Self {
        GuiError::new(&format!("IO error: {}", e))
    }
}

fn print_welcome() {
    println!("🎨 SC Score Visualizer GUI v2.0");
    println!("═══════════════════════════════════════");
    println!("Esta es una interfaz de control básica.");
    println!("Para el visualizador completo, ejecute:");
    println!("  cargo run --bin sc_score_visualizer");
    println!("═══════════════════════════════════════");
}

/// Función principal para ejecutar la GUI
pub fn run_gui() -> Result<(), GuiError> {
    log::info!("Iniciando SC Score Visualizer GUI v2.0...");

    print_welcome();

    let using_default = !Path::new("config.toml").exists();
    let config = AppConfig::load_or_default("config.toml");

    if using_default {
        println!("⚠️  No se encontró 'config.toml'. Usando configuración por defecto.");
    }

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

    println!("\n¿Desea iniciar el visualizador ahora? (s/N)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "s" {
        println!("🚀 Iniciando visualizador...");
        std::process::Command::new("cargo")
            .args(&["run", "--bin", "sc_score_visualizer"])
            .spawn()
            .map_err(|e| GuiError::new(&format!("Error lanzando visualizador: {}", e)))?;
        return Ok(());
    }

    println!("\n🎯 Presione Enter para continuar o Ctrl+C para salir...");
    input.clear();
    std::io::stdin().read_line(&mut input)?;

    println!("✅ GUI básica ejecutada correctamente.");
    println!("💡 Para futuras versiones se implementará una GUI completa con egui/iced.");

    Ok(())
}
