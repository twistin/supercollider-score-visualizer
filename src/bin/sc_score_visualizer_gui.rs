use env_logger;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
// Binario deshabilitado temporalmente por error de módulo gui
/*
// Configurar logging
env_logger::Builder::from_default_env()
    .filter_level(log::LevelFilter::Info)
    .init();

// Mostrar información de inicio
log::info!("Iniciando {}", &format!("SC Score Visualizer GUI v{}", env!("CARGO_PKG_VERSION")));
    
// Verificar argumentos de línea de comandos
let args: Vec<String> = env::args().collect();
if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
    print_help();
    return Ok(());
}
    
if args.len() > 1 && args[1] == "--version" {
    println!("{}", &format!("SC Score Visualizer GUI v{}", env!("CARGO_PKG_VERSION")));
    return Ok(());
}
    
if args.contains(&"--run-headless".to_string()) {
    log::info!("Ejecutando en modo sin GUI (--run-headless)");
    return Ok(());
}
    
// Iniciar la aplicación GUI
sc_score_visualizer::gui::run_gui().expect("Error al iniciar la GUI");
log::info!("Aplicación GUI terminada correctamente");
Ok(())
*/
    Ok(())
}

fn print_help() {
    println!("{}", &format!("SC Score Visualizer GUI v{}", env!("CARGO_PKG_VERSION")));
    println!("Control gráfico para el sistema de visualización de audio en tiempo real");
    println!();
    println!("USO:");
    println!("    sc_score_visualizer_gui [OPTIONS]");
    println!();
    println!("OPCIONES:");
    println!("    -h, --help       Mostrar esta ayuda");
    println!("    --version        Mostrar versión");
    println!("    --run-headless   Ejecutar sin iniciar la interfaz gráfica");
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
