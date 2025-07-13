use sc_score_visualizer::gui;
use env_logger;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Mostrar información de inicio
    log::info!("Iniciando SC Score Visualizer GUI v2.0.0");
    
    // Verificar argumentos de línea de comandos
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return Ok(());
    }

    if args.len() > 1 && args[1] == "--version" {
        println!("SC Score Visualizer GUI v2.0.0");
        return Ok(());
    }

    // Iniciar la aplicación GUI
    match gui::run_gui() {
        Ok(_) => {
            log::info!("Aplicación GUI terminada correctamente");
            Ok(())
        }
        Err(e) => {
            log::error!("Error en la aplicación GUI: {}", e);
            Err(Box::new(e))
        }
    }
}

fn print_help() {
    println!("SC Score Visualizer GUI v2.0.0");
    println!("Control gráfico para el sistema de visualización de audio en tiempo real");
    println!();
    println!("USO:");
    println!("    sc_score_visualizer_gui [OPTIONS]");
    println!();
    println!("OPCIONES:");
    println!("    -h, --help       Mostrar esta ayuda");
    println!("    --version        Mostrar versión");
    println!();
    println!("CARACTERÍSTICAS:");
    println!("    • Control visual del visualizador principal");
    println!("    • Gestión de conexiones OSC/MIDI");
    println!("    • Selección de paletas de colores");
    println!("    • Carga y ejecución de patches SuperCollider");
    println!("    • Configuración en tiempo real");
    println!();
    println!("Para usar el visualizador sin GUI, ejecute:");
    println!("    sc_score_visualizer");
}
