pub mod model;
pub mod config;
pub mod events;
pub mod visual;
pub mod osc_server;
pub mod logging;
pub mod midi;
pub mod errors;

use nannou::prelude::*;
use nannou::event::{Event, Key, WindowEvent::KeyPressed};
use clap::Parser;
use crate::osc_server::OscServer;
use crate::model::Model;
use crate::model::DisplayMode;
use crate::errors::VisualizerError; // Asegúrate de importar VisualizerError

#[derive(Parser)]
#[command(name = "SC Score Visualizer")]
struct CliArgs {
    #[arg(long, help = "Activar modo debug detallado")]
    debug: bool,
}

/// Función principal - punto de entrada de la aplicación
fn main() {
    let args = CliArgs::parse();
    if args.debug {
        println!("🛠️  Modo debug activado por argumento");
    }

    println!("Logger inicializado y funcionando.");
    println!("🚀 Iniciando SC Score Visualizer v2.0");
    
    nannou::app(model_setup)
        .update(update)
        .event(event)
        .view(view) // <--- ¡AÑADE ESTA LÍNEA AQUÍ!
        .run();
}

/// Configuración inicial del modelo y ventana
fn model_setup(app: &App) -> Model {
    println!("DEBUG: model_setup ha sido llamado.");
    println!("🔧 Configurando ventana principal...");
    
    println!("🔧 Cargando configuración...");
    let mut config = crate::config::AppConfig::load().expect("Error al cargar configuración");
    println!("✅ Configuración cargada y validada");
    
    // --- Lógica de inicialización del servidor OSC más robusta ---
    let (osc_server_instance, osc_rx_for_events) = {
        let initial_config_port = config.osc.listen_port; // Puerto inicial de config.toml
        let mut osc_server_result = Err(VisualizerError::OscConnectionError { message: "Initial attempt".to_string() });
        let max_attempts = 10; // Aumentamos el número máximo de puertos a intentar

        // Definir una lista de puertos a intentar, priorizando el de la config
        let mut ports_to_try: Vec<u16> = vec![initial_config_port];
        // Añadir puertos alternativos comunes y algunos muy altos para mayor probabilidad de éxito
        if initial_config_port != 57120 { ports_to_try.push(57120); } // Puerto por defecto de SuperCollider
        if initial_config_port != 7771 { ports_to_try.push(7771); } // Puerto que usaste en SC
        if initial_config_port != 7773 { ports_to_try.push(7773); } // Tu fallback anterior
        if initial_config_port != 60000 { ports_to_try.push(60000); } // Puerto alto sugerido
        ports_to_try.push(57123); // Otro puerto común OSC
        ports_to_try.push(57124); // Otro puerto común OSC
        ports_to_try.push(8000);  // Puerto común para desarrollo
        ports_to_try.push(8080);  // Otro puerto común para desarrollo
        ports_to_try.push(49152); // Inicio del rango de puertos efímeros (a veces usados por macOS)
        ports_to_try.push(49153);
        ports_to_try.push(49154);
        ports_to_try.push(50000); // Un puerto alto aleatorio
        ports_to_try.push(50001);
        ports_to_try.push(50002);
        ports_to_try.push(50003);
        ports_to_try.push(50004);
        ports_to_try.push(50005);
        ports_to_try.push(50006);
        ports_to_try.push(50007);
        ports_to_try.push(50008);
        ports_to_try.push(50009);
        ports_to_try.push(50010);


        // Eliminar duplicados y asegurar que el puerto inicial esté al principio
        ports_to_try.sort_unstable();
        ports_to_try.dedup();
        
        // Asegurarse de que el puerto configurado inicialmente sea el primero en la lista
        if let Some(pos) = ports_to_try.iter().position(|&p| p == initial_config_port) {
            let initial_port_val = ports_to_try.remove(pos);
            ports_to_try.insert(0, initial_port_val);
        }

        for (attempt_num, &port) in ports_to_try.iter().take(max_attempts).enumerate() {
            let mut temp_osc_config = config.osc.clone();
            temp_osc_config.listen_port = port;

            println!("📡 Intento {}/{} - Iniciando canal de comunicación OSC en puerto {}...", attempt_num + 1, max_attempts, port);
            osc_server_result = OscServer::new(temp_osc_config, config.audio.clone());

            match &osc_server_result {
                Ok(_) => {
                    println!("✅ Servidor OSC iniciado exitosamente en puerto: {}", port);
                    config.osc.listen_port = port; // Actualiza la config con el puerto exitoso
                    break; // Éxito, salir del bucle
                },
                Err(e) => {
                    if e.to_string().contains("Address already in use") {
                        println!("⚠️  Puerto {} ocupado. Intentando con el siguiente...", port);
                        std::thread::sleep(std::time::Duration::from_millis(100)); // Pequeña pausa
                    } else {
                        eprintln!("❌ Error inesperado al iniciar OSC en puerto {}: {}", port, e);
                        // Para otros errores, podríamos querer salir inmediatamente o loguear más detalles
                    }
                }
            }
        }

        // Después del bucle, verificar si tuvimos éxito
        match osc_server_result {
            Ok((server, rx)) => (server, rx),
            Err(e) => {
                eprintln!("❌ Error fatal: No se pudo iniciar el servidor OSC después de {} intentos. Último error: {}", max_attempts, e);
                eprintln!("Por favor, asegúrate de que no haya otras aplicaciones usando los puertos de red.");
                eprintln!("Puedes intentar reiniciar tu sistema para liberar todos los puertos.");
                std::process::exit(1);
            }
        }
    };
    // --- Fin de la lógica de inicialización del servidor OSC robusta ---

    println!("✅ Receptor OSC de eventos musicales iniciado.");

    println!("📊 Inicializando modelo de datos...");
    let musical_rx = std::sync::mpsc::Receiver::from(osc_rx_for_events);
    let rx = crate::osc_server::map_processed_to_musical(musical_rx);
    let model = Model::new(app, config.clone(), rx, osc_server_instance)
        .expect("No se pudo crear el modelo");

    println!("✅ Modelo de datos inicializado."); // Nuevo mensaje de depuración
    model
}

/// Bucle de actualización principal
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update(); // El método update del modelo ahora maneja la lógica principal
    
    model.frame_counter += 1;
    let frame = model.frame_counter;

    if model.config.logging.show_performance_stats && 
       frame % model.config.logging.stats_interval_frames as u64 == 0 {
        let osc_stats = model.osc_server.get_stats();
        println!("⚡ Frame {}: {} eventos activos | {} notas visuales | OSC: {:.1} msg/s | Conectado: {}", 
               frame, 
               model.musical_events.len(), // Usar musical_events
               model.visual_notes.len(),
               osc_stats.messages_per_second,
               osc_stats.is_connected);
    }
    
    if frame % model.config.performance.cleanup_interval_frames as u64 == 0 {
        let before_musical_count = model.musical_events.len();
        let before_visual_count = model.visual_notes.len();
        model.cleanup_expired_events();
        let after_musical_count = model.musical_events.len();
        let after_visual_count = model.visual_notes.len();
        
        if before_musical_count != after_musical_count || before_visual_count != after_visual_count {
            println!("🧹 Limpieza automática: {} -> {} eventos musicales | {} -> {} notas visuales", 
                   before_musical_count, after_musical_count, before_visual_count, after_visual_count);
        }
    }
}

/// Función de renderizado

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    // Simplemente dibuja un color de fondo.
    draw.background().color(nannou::prelude::PLUM);
    draw.to_frame(app, &frame).unwrap();

    // Añade un log para confirmar que se está ejecutando
    println!("-> Frame dibujado en view()");
}

/// **Función de manejo de eventos (teclado, mouse, etc.)**
fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple: Some(window_event), .. } => {
            match window_event {
                KeyPressed(key) => {
                    handle_key_pressed(model, key);
                }
                _ => {}
            }
        }
        _ => {}
    }
}

/// **Maneja eventos de teclado**
fn handle_key_pressed(model: &mut Model, key: Key) {
    match key {
        Key::S => {
            model.toggle_scroll_mode();
            println!("🔄 Modo de scroll cambiado a: {:?}", model.get_scroll_mode());
        }
        
        Key::Left => {
            let new_speed = (model.get_scroll_speed() - 20.0).max(0.0);
            model.set_scroll_speed(new_speed);
            println!("⬅️  Velocidad de scroll: {:.0} px/s", new_speed);
        }
        Key::Right => {
            let new_speed = (model.get_scroll_speed() + 20.0).min(500.0);
            model.set_scroll_speed(new_speed);
            println!("➡️  Velocidad de scroll: {:.0} px/s", new_speed);
        }
        
        Key::M => {
            let next_mode = cycle_display_mode(&model.display_mode);
            model.set_display_mode(next_mode.clone());
            println!("🎭 Modo de display cambiado a: {:?}", next_mode);
        }
        
        Key::Key1 => {
            model.set_display_mode(DisplayMode::Events);
            println!("🎵 Modo: Solo Eventos");
        }
        Key::Key2 => {
            model.set_display_mode(DisplayMode::Analysis);
            println!("📊 Modo: Solo Análisis");
        }
        Key::Key3 => {
            model.set_display_mode(DisplayMode::Drones);
            println!("🎛️ Modo: Solo Drones");
        }
        Key::Key4 => {
            model.set_display_mode(DisplayMode::Cluster);
            println!("🌌 Modo: Solo Cluster");
        }
        Key::Key5 => {
            model.set_display_mode(DisplayMode::Combined);
            println!("🔄 Modo: Combinado");
        }
        
        Key::C => {
            model.clear_events();
            model.clear_visual_notes();
            println!("🧹 Eventos limpiados");
        }
        
        Key::D => {
            let show_debug = !model.display_config().visual.show_debug;
            model.set_display_config(show_debug, model.display_config().visual.show_grid);
            println!("🐛 Debug info: {}", if show_debug { "ON" } else { "OFF" });
        }
        
        Key::G => {
            let show_grid = !model.display_config().visual.show_grid;
            model.set_display_config(model.display_config().visual.show_debug, show_grid);
            println!("📐 Grid: {}", if show_grid { "ON" } else { "OFF" });
        }
        
        Key::H => {
            println!("\n🎹 === CONTROLES DE TECLADO ===");
            println!("S          - Alternar modo scroll (Fijo/Scrolling)");
            println!("← →        - Ajustar velocidad de scroll");
            println!("M          - Cambiar modo de display (Eventos/Análisis/Drones/Cluster/Combinado)");
            println!("1-5        - Cambiar modo de display (alternativo)");
            println!("C          - Limpiar eventos");
            println!("D          - Alternar debug info");
            println!("G          - Alternar grid");
            println!("H          - Mostrar esta ayuda");
            println!("ESC        - Salir de la aplicación");
            println!("===============================\n");
        }
        
        Key::Escape => {
            println!("👋 Cerrando visualizador...");
            std::process::exit(0);
        }
        
        _ => {}
    }
}

fn cycle_display_mode(current: &DisplayMode) -> DisplayMode {
    match current {
        DisplayMode::Events => DisplayMode::Analysis,
        DisplayMode::Analysis => DisplayMode::Drones,
        DisplayMode::Drones => DisplayMode::Cluster,
        DisplayMode::Cluster => DisplayMode::Combined,
        DisplayMode::Combined => DisplayMode::Events,
    }
}
