use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use crate::logging::Logger;
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
use crate::visual::shader_manager::ShaderManager;
use std::sync::Arc;
use crate::errors::VisualizerError; // Asegúrate de importar VisualizerError

#[derive(Parser)]
#[command(name = "SC Score Visualizer")]
struct CliArgs {
    #[arg(long, help = "Activar modo debug detallado")]
    debug: bool,
}

/// Punto de entrada principal de la aplicación SC Score Visualizer.
/// Inicializa argumentos de línea de comandos, el logger y lanza la aplicación nannou.
fn main() {
    let args = CliArgs::parse();
    if args.debug {
        println!("🛠️  Modo debug activado por argumento");
    }

    crate::logging::Logger::init();
    println!("Logger inicializado y funcionando.");
    println!("🚀 Iniciando SC Score Visualizer v2.0");
    
    nannou::app(model_setup)
        .update(update)
        .event(event)
        .view(view)
        .run();
}

/// Configura la ventana principal y carga la configuración de la aplicación.
/// También inicia el servidor OSC intentando múltiples puertos hasta encontrar uno libre.
/// Devuelve el modelo de datos completamente inicializado.
fn model_setup(app: &App) -> Model {
    // --- Configuración de ventana y carga de configuración ---
    println!("🔧 Cargando configuración...");
    let mut config = crate::config::AppConfig::load().expect("Error al cargar configuración");
    println!("✅ Configuración cargada y validada");
    // Leer parámetros de ventana desde config.toml
    let window_width = config.window.width;
    let window_height = config.window.height;
    let window_title = config.window.title.clone();
    let window_resizable = config.window.resizable;
    app.new_window()
        .title(&window_title)
        .size(window_width, window_height)
        .resizable(window_resizable)
        .view(view)
        .build()
        .unwrap();
    eprintln!("DEBUG: Ventana configurada");
    println!("DEBUG: model_setup ha sido llamado.");
    println!("🔧 Configurando ventana principal...");
    
    // --- Lógica de inicialización del servidor OSC más robusta ---
    let (osc_server_instance, osc_rx_for_events) = {
        // --- Inicialización del servidor OSC SOLO en el puerto especificado ---
        let osc_server_result = OscServer::new(config.osc.clone(), config.audio.clone());
        match osc_server_result {
            Ok((server, rx)) => {
                println!("✅ Servidor OSC iniciado exitosamente en puerto: {}", config.osc.listen_port);
                (server, rx)
            },
            Err(e) => {
                eprintln!("❌ Error: No se pudo iniciar el servidor OSC en el puerto {}: {e}", config.osc.listen_port);
                eprintln!("Asegúrate de que el puerto esté libre y no esté siendo usado por otra aplicación.");
                std::process::exit(1);
            }
        }
    };
    // --- Fin de la inicialización del servidor OSC ---

    println!("✅ Receptor OSC de eventos musicales iniciado.");

    println!("📊 Inicializando modelo de datos...");
    let model = Model::new_with_receiver(config.clone(), osc_rx_for_events, osc_server_instance);

    println!("✅ Modelo de datos inicializado."); // Nuevo mensaje de depuración
    model
}

/// Bucle de actualización que se ejecuta en cada frame.
/// Realiza limpieza periódica y muestra estadísticas si están activadas.
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update(); // El método update del modelo ahora maneja la lógica principal
    
    model.time_info.frame_counter += 1;
    let frame = model.time_info.frame_counter;

    if model.config.logging.show_performance_stats && 
       frame % model.config.logging.stats_interval_frames as u64 == 0 {
        // osc_stats no se actualiza automáticamente, pero mostramos el contador de eventos
        println!("⚡ Frame {}: {} eventos activos | {} notas visuales | OSC: N/A | Conectado: N/A", 
               frame, 
               model.musical_events.len(),
               model.visual_notes.len());
    }
    
    if frame % model.config.performance.cleanup_interval_frames as u64 == 0 {
        let before_musical_count = model.musical_events.len();
        let before_visual_count = model.visual_notes.len();
        model.cleanup_expired_events();
        let after_musical_count = model.musical_events.len();
        let after_visual_count = model.visual_notes.len();
        
        if before_musical_count != after_musical_count || before_visual_count != after_visual_count {
            println!("🧹 Limpieza automática: {before_musical_count} -> {after_musical_count} eventos musicales | {before_visual_count} -> {after_visual_count} notas visuales");
        }
    }
}

/// Dibuja el frame actual con un fondo simple (PLUM).
/// Se puede extender para representar notas visuales u otros elementos.
use crate::visual::renderer::render_visualizer;

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(rgba(0.05, 0.05, 0.1, 1.0));

    // Timeline animado: los eventos se desplazan de derecha a izquierda según su tiempo de aparición
    let now = model.time_info.elapsed_time;
    let timeline_secs = model.config.visual.timeline_duration;
    let win_left = win.left();
    let win_right = win.right();
    let win_width = win_right - win_left;

    for event in &model.musical_events {
        // Cada evento debe tener un campo de tiempo de aparición (start_time)
        let (start_time, duration, y, shape, color, size1, size2, opacity) = match event {
            crate::events::MusicalEvent::Note { frequency, amplitude, start_time, .. } => {
                let y = crate::events::map_freq_to_y(*frequency, &model.config.audio, win);
                let radius = amplitude.abs() * 40.0 + 10.0;
                // Color según frecuencia (hue mapeado de 0.0 a 360.0 para espectro completo)
                let min_freq = 20.0;
                let max_freq = 5000.0;
                let norm = ((*frequency - min_freq) / (max_freq - min_freq)).clamp(0.0, 1.0);
                let hue_deg = norm * 360.0; // 0.0 a 360.0 grados
                let hsv = nannou::color::Hsv::new(hue_deg, 0.85, 1.0);
                let rgb: nannou::color::Rgb<f32> = nannou::color::Rgb::from(hsv);
                let (r, g, b) = rgb.into_components();
                let color = rgba(r, g, b, 0.8);
                (*start_time, 1.0, y, "ellipse", color, radius, 0.0, 0.8)
            }
            crate::events::MusicalEvent::Drone { frequency, amplitude, duration, start_time, .. } => {
                let y = crate::events::map_freq_to_y(*frequency, &model.config.audio, win);
                let width = duration.abs() * 60.0 + 30.0;
                let height = amplitude.abs() * 20.0 + 8.0;
                // Color verde fijo para drones
                let color = rgba(0.1, 0.9, 0.3, 0.6);
                (*start_time, *duration, y, "rect", color, width, height, 0.6)
            }
            crate::events::MusicalEvent::Cluster { center_freq, density, amplitude, start_time, duration, .. } => {
                let y = crate::events::map_freq_to_y(*center_freq, &model.config.audio, win);
                let height = density.abs() * 80.0 + 20.0;
                let width = amplitude.abs() * 10.0 + 4.0;
                // Color violeta fijo para clusters
                let color = rgba(0.7, 0.2, 0.8, 0.7);
                (*start_time, *duration, y, "line", color, width, height, 0.7)
            }
            _ => { continue; }
        };

        // Calcular posición X en el timeline (izquierda a derecha)
        let elapsed = Instant::now().duration_since(start_time).as_secs_f32();
        if elapsed < 0.0 || elapsed > timeline_secs { continue; } // Solo mostrar eventos activos en el timeline
        let x = win_left + (elapsed / timeline_secs) * win_width;

        // Fade out visual al final de la vida
        let fade = if elapsed > duration && duration > 0.0 {
            let t = ((elapsed - duration) / (timeline_secs - duration)).min(1.0).max(0.0);
            1.0 - t
        } else { 1.0 };
        let final_opacity = opacity * fade;
        let (r, g, b, _a) = color.into_components();
        let color = rgba(r, g, b, final_opacity as f32);

        match shape {
            "ellipse" => {
                draw.ellipse()
                    .x_y(x, y)
                    .radius(size1)
                    .color(color);
            }
            "rect" => {
                draw.rect()
                    .x_y(x, y)
                    .w(size1)
                    .h(size2)
                    .color(color);
            }
            "line" => {
                draw.line()
                    .start(pt2(x - size1, y - size2/2.0))
                    .end(pt2(x + size1, y + size2/2.0))
                    .color(color)
                    .weight(size1);
            }
            _ => {}
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

/// Maneja eventos de entrada como teclado o ratón.
/// Actualmente reacciona a eventos de teclado y delega su manejo.
fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent { simple: Some(window_event), .. } = event {
        if let KeyPressed(key) = window_event {
            handle_key_pressed(model, key);
        }
    }
}

/// Procesa pulsaciones de teclado individuales para controlar la visualización,
/// alternar modos de scroll, cambiar modos visuales, mostrar ayuda o cerrar la app.
fn handle_key_pressed(model: &mut Model, key: Key) {
    match key {
        Key::S => {
            model.toggle_scroll_mode();
            println!("🔄 Modo de scroll cambiado a: {:?}", model.get_scroll_mode());
        }
        
        Key::Left => {
            let new_speed = (model.get_scroll_speed() - 20.0).max(0.0);
            model.set_scroll_speed(new_speed);
            println!("⬅️  Velocidad de scroll: {new_speed:.0} px/s");
        }
        Key::Right => {
            let new_speed = (model.get_scroll_speed() + 20.0).min(500.0);
            model.set_scroll_speed(new_speed);
            println!("➡️  Velocidad de scroll: {new_speed:.0} px/s");
        }
        
        Key::M => {
            let next_mode = cycle_display_mode(&model.display_mode);
            model.set_display_mode(next_mode);
            println!("🎭 Modo de display cambiado a: {next_mode:?}");
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

/// Cambia entre los modos de visualización en orden cíclico.
/// Usado al presionar la tecla 'M'.
fn cycle_display_mode(current: &DisplayMode) -> DisplayMode {
    match current {
        DisplayMode::Events => DisplayMode::Analysis,
        DisplayMode::Analysis => DisplayMode::Drones,
        DisplayMode::Drones => DisplayMode::Cluster,
        DisplayMode::Cluster => DisplayMode::Combined,
        DisplayMode::Combined => DisplayMode::Events,
    }
}
