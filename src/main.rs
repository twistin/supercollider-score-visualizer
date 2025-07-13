// src/main.rs - SC Score Visualizer - Punto de entrada principal

use nannou::prelude::*;
use nannou::event::{Event, Key, WindowEvent::KeyPressed};

// Módulos principales refactorizados
mod model;
mod audio;
mod visual;
mod logging;
mod errors;
mod config;
mod osc_server;
mod midi;
mod capture;
mod visuals;
// Importaciones principales
use audio::process_osc_messages_robust;
use model::Model;
use visuals::draw_visualization;
use config::AppConfig;
use osc_server::OscServer;

/// Función principal - punto de entrada de la aplicación
fn main() {
    println!("🚀 Iniciando SC Score Visualizer v2.0");
    
    // Cargar configuración
    println!("🔧 Cargando configuración...");
    let config = AppConfig::load_or_default("config.toml");
    
    // Validar configuración
    if let Err(e) = config.validate() {
        eprintln!("❌ Error en configuración: {}", e);
        std::process::exit(1);
    }
    
    println!("✅ Configuración cargada y validada");
    
    // Crear y ejecutar aplicación Nannou
    nannou::app(model_setup)
        .update(update)
        .event(event)
        .run();
}

/// Variable global para la configuración (simplificación temporal)
static mut GLOBAL_CONFIG: Option<AppConfig> = None;

/// Obtiene la configuración global de forma segura
fn get_config() -> AppConfig {
    unsafe {
        GLOBAL_CONFIG.clone().unwrap_or_else(|| {
            let config = AppConfig::load_or_default("config.toml");
            GLOBAL_CONFIG = Some(config.clone());
            config
        })
    }
}

/// Configuración inicial del modelo y ventana
fn model_setup(app: &App) -> Model {
    let config = get_config();
    
    println!("🔧 Configurando ventana principal...");
    
    // Configurar ventana principal usando configuración externa
    let window_result = app.new_window()
        .size(config.window.width, config.window.height)
        .title(&config.window.title)
        .view(view)
        .resizable(config.window.resizable)
        .build();

    match window_result {
        Ok(_) => println!("✅ Ventana creada exitosamente ({}x{})", 
                         config.window.width, config.window.height),
        Err(e) => {
            eprintln!("❌ Error creando ventana: {}", e);
            std::process::exit(1);
        }
    };

    println!("📡 Iniciando servidor OSC robusto...");
    
    // Crear servidor OSC robusto
    let osc_server = match OscServer::new(config.osc.clone()) {
        Ok(server) => {
            println!("✅ Servidor OSC iniciado exitosamente");
            server.self_test();
            server
        }
        Err(e) => {
            eprintln!("❌ Error iniciando servidor OSC: {}", e);
            std::process::exit(1);
        }
    };

    println!("📊 Inicializando modelo de datos...");
    let mut model = Model::new_with_config(osc_server, config.clone());
    
    // Inicializar MIDI si está habilitado
    model.init_midi();
    
    println!("🚀 SC Score Visualizer iniciado exitosamente");
    println!("🎨 Configuración: {} | {} | Debug: {}", 
           model.config.visual.quality,
           model.config.visual.background_style,
           model.config.visual.show_debug);

    model
}

/// Bucle de actualización principal
fn update(app: &App, model: &mut Model, update: Update) {
    // Actualizar tiempo transcurrido en el modelo y shader manager
    model.update_time(app.elapsed_frames() as f32 / 60.0); // Asumiendo 60 FPS
    
    // Procesar mensajes OSC entrantes con el nuevo sistema robusto
    process_osc_messages_robust(model, app);
    
    // Procesar mensajes MIDI si está habilitado
    let win = app.window_rect();
    model.process_midi_messages(win);

    // Actualizar ciclo de vida de eventos
    let delta_time = update.since_last.as_secs_f32();
    model.update_events(delta_time);
    
    // Actualizar notas visuales avanzadas
    model.update_visual_notes(delta_time, win);

    // Log de rendimiento según configuración
    static mut FRAME_COUNTER: u64 = 0;
    unsafe {
        FRAME_COUNTER += 1;
        
        if model.config.logging.show_performance_stats && 
           FRAME_COUNTER % model.config.logging.stats_interval_frames as u64 == 0 {
            let osc_stats = model.osc_server.get_stats();
            println!("⚡ Frame {}: {} eventos activos | {} notas visuales | OSC: {:.1} msg/s | Conectado: {}", 
                   FRAME_COUNTER, 
                   model.notes.len() + model.drone_events.len(),
                   model.visual_notes.len(),
                   osc_stats.messages_per_second,
                   osc_stats.is_connected);
        }
        
        // Limpieza automática de eventos expirados
        if FRAME_COUNTER % model.config.performance.cleanup_interval_frames as u64 == 0 {
            let before_count = model.notes.len() + model.drone_events.len();
            let before_visual_count = model.visual_notes.len();
            model.cleanup_expired_events();
            let after_count = model.notes.len() + model.drone_events.len();
            let after_visual_count = model.visual_notes.len();
            
            if before_count != after_count || before_visual_count != after_visual_count {
                println!("🧹 Limpieza automática: {} -> {} eventos legacy | {} -> {} notas visuales", 
                       before_count, after_count, before_visual_count, after_visual_count);
            }
        }
    }
}

/// Función de renderizado
fn view(app: &App, model: &Model, frame: Frame) {
    draw_visualization(app, model, frame);
}

/// **Función de manejo de eventos (teclado, mouse, etc.)**
fn event(app: &App, model: &mut Model, event: Event) {
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
        // **S - Alternar modo de scroll**
        Key::S => {
            model.toggle_scroll_mode();
            println!("🔄 Modo de scroll cambiado a: {:?}", model.get_scroll_mode());
        }
        
        // **Flecha Izquierda/Derecha - Ajustar velocidad de scroll**
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
        
        // **M - Alternar modo visual (Fixed/Scrolling)**
        Key::M => {
            model.toggle_scroll_mode();
            let mode_name = match model.get_scroll_mode() {
                model::ScrollMode::Fixed => "FIJO",
                model::ScrollMode::Scrolling => "SCROLL",
            };
            println!("🎭 Modo visual cambiado a: {} (Speed: {:.0} px/s)", 
                   mode_name, model.get_scroll_speed());
        }
        
        // **Números 1-5 - Cambiar modo de display**
        Key::Key1 => {
            model.set_display_mode(model::DisplayMode::Events);
            println!("🎵 Modo: Solo Eventos");
        }
        Key::Key2 => {
            model.set_display_mode(model::DisplayMode::Analysis);
            println!("📊 Modo: Solo Análisis");
        }
        Key::Key3 => {
            model.set_display_mode(model::DisplayMode::Drones);
            println!("🎛️ Modo: Solo Drones");
        }
        Key::Key4 => {
            model.set_display_mode(model::DisplayMode::Cluster);
            println!("🌌 Modo: Solo Cluster");
        }
        Key::Key5 => {
            model.set_display_mode(model::DisplayMode::Combined);
            println!("🔄 Modo: Combinado");
        }
        
        // **C - Limpiar eventos**
        Key::C => {
            model.clear_events();
            model.clear_visual_notes();
            println!("🧹 Eventos limpiados");
        }
        
        // **D - Alternar debug info**
        Key::D => {
            let show_debug = !model.display_config.show_debug;
            model.set_display_config(show_debug, model.display_config.show_grid);
            println!("🐛 Debug info: {}", if show_debug { "ON" } else { "OFF" });
        }
        
        // **G - Alternar grid**
        Key::G => {
            let show_grid = !model.display_config.show_grid;
            model.set_display_config(model.display_config.show_debug, show_grid);
            println!("📐 Grid: {}", if show_grid { "ON" } else { "OFF" });
        }
        
        // **Espacio - Información de ayuda**
        Key::Space => {
            print_help();
        }
        
        _ => {} // Ignorar otras teclas
    }
}

/// **Muestra la ayuda de controles de teclado**
fn print_help() {
    println!("\n🎹 === CONTROLES DE TECLADO ===");
    println!("S          - Alternar modo scroll (Fijo/Scrolling)");
    println!("M          - Alternar modo visual (Fijo/Scroll)");
    println!("← →        - Ajustar velocidad de scroll");
    println!("1-5        - Cambiar modo de display");
    println!("C          - Limpiar eventos");
    println!("D          - Alternar debug info");
    println!("G          - Alternar grid");
    println!("ESPACIO    - Mostrar esta ayuda");
    println!("===============================\n");
}
