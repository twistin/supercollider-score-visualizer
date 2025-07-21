use std::process::Command;
use std::thread;
use std::time::Duration;

fn launch_supercollider() -> Result<(), String> {
    use std::path::Path;
    let path = Path::new("inicio_maestro.scd");
    if !path.exists() {
        return Err("⚠️ Archivo 'inicio_maestro.scd' no encontrado.".to_string());
    }

    let status = Command::new("sclang")
        .arg("inicio_maestro.scd")
        .status()
        .map_err(|e| format!("❌ No se pudo ejecutar sclang: {}", e))?;

    if status.success() {
        println!("✅ SuperCollider iniciado correctamente");
        Ok(())
    } else {
        Err("❌ sclang terminó con error.".to_string())
    }
}

fn launch_visualizer() -> Result<(), String> {
    let status = Command::new("cargo")
        .args(&["run", "--bin", "sc_score_visualizer"])
        .status()
        .map_err(|e| format!("❌ No se pudo ejecutar cargo: {}", e))?;

    if status.success() {
        println!("✅ Visualizador iniciado correctamente");
        Ok(())
    } else {
        Err("❌ El visualizador terminó con error.".to_string())
    }
}

fn main() {
    println!("🚀 Iniciando sistema de live coding...");

    println!("🎵 Iniciando SuperCollider...");
    if let Err(e) = launch_supercollider() {
        println!("{}", e);
        return;
    }

    thread::sleep(Duration::from_secs(5));

    println!("🎨 Iniciando visualizador Nannou...");
    if let Err(e) = launch_visualizer() {
        println!("{}", e);
        return;
    }

    println!("🎯 Sistema listo para live coding. ¡Disfruta!");
}
