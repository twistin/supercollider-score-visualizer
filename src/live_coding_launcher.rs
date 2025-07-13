use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("🚀 Iniciando sistema de live coding...");

    // Paso 1: Iniciar el servidor SuperCollider
    println!("🎵 Iniciando SuperCollider...");
    let supercollider = Command::new("sclang")
        .arg("inicio_maestro.scd")
        .spawn();

    match supercollider {
        Ok(_) => println!("✅ SuperCollider iniciado correctamente"),
        Err(e) => println!("❌ Error al iniciar SuperCollider: {}", e),
    }

    // Esperar unos segundos para que SuperCollider se inicialice
    thread::sleep(Duration::from_secs(5));

    // Paso 2: Iniciar el visualizador Nannou
    println!("🎨 Iniciando visualizador Nannou...");
    let visualizer = Command::new("cargo")
        .arg("run")
        .spawn();

    match visualizer {
        Ok(_) => println!("✅ Visualizador iniciado correctamente"),
        Err(e) => println!("❌ Error al iniciar el visualizador: {}", e),
    }

    println!("🎯 Sistema listo para live coding. ¡Disfruta!");
}
