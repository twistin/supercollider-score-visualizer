// =================================================================
// 🎵 SC SCORE VISUALIZER - PUNTO DE ENTRADA
// =================================================================
// Sistema profesional de auto-visualización para live coding
// Compatible con SuperCollider y ProxySpace

use nannou::prelude::*;
use std::time::Instant;

mod model;
mod events;
mod osc;
mod visuals;

use model::{Model, load_config};
use osc::{setup_osc_receiver, process_osc_packet, OscMessage};
use visuals::draw_visualization;

// =================================================================
// FUNCIÓN PRINCIPAL
// =================================================================

fn main() {
    println!("🎵 SC Score Visualizer - Iniciando...");
    
    // Cargar configuración
    let config = load_config();
    println!("📊 Puerto OSC: {}", config.osc.port);
    println!("📊 Máximo eventos: {}", config.visual.max_events);
    println!("📊 Ventana de tiempo: {:.1}s", config.visual.time_window);
    
    // Inicializar Nannou
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1200, 800)
        .run();
}

// =================================================================
// INICIALIZACIÓN DEL MODELO
// =================================================================

fn model(app: &App) -> Model {
    let config = load_config();
    
    // Configurar receptor OSC
    let osc_receiver = match setup_osc_receiver(config.osc.port) {
        Ok(receiver) => {
            println!("✅ Receptor OSC configurado en puerto {}", config.osc.port);
            receiver
        }
        Err(e) => {
            eprintln!("❌ Error configurando OSC: {}", e);
            std::process::exit(1);
        }
    };
    
    // Configurar ventana
    let window = app.main_window();
    window.set_title("SC Score Visualizer");
    
    println!("🚀 Visualizador listo - Esperando mensajes OSC...");
    
    Model::new(config, osc_receiver)
}

// =================================================================
// BUCLE DE ACTUALIZACIÓN
// =================================================================

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.ui_state.pause_updates {
        return;
    }
    
    // Procesar mensajes OSC
    process_osc_messages(app, model);
    
    // Actualizar eventos existentes
    update_events(model);
    
    // Limpiar eventos antiguos
    model.cleanup_old_events();
    
    // Actualizar estadísticas
    model.update_stats();
}

fn process_osc_messages(app: &App, model: &mut Model) {
    let window_rect = app.window_rect();
    
    // Procesar todos los mensajes OSC pendientes
    while let Ok(packet) = model.osc_receiver.try_recv() {
        if let Some(osc_message) = process_osc_packet(&packet, window_rect) {
            match osc_message {
                OscMessage::MusicalEvent(event) => {
                    model.add_event(event);
                }
                OscMessage::RealtimeData(data) => {
                    model.update_realtime_data(data);
                }
            }
        }
    }
}

fn update_events(model: &mut Model) {
    let dt = 1.0 / 60.0; // Asumiendo 60 FPS
    
    for event in &mut model.events {
        event.update(dt);
    }
}

// =================================================================
// FUNCIÓN DE DIBUJO
// =================================================================

fn view(app: &App, model: &Model, frame: Frame) {
    draw_visualization(app, model, frame);
}

// =================================================================
// MANEJO DE EVENTOS DE TECLADO
// =================================================================

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => model.toggle_ui_element("stats"),
        Key::G => model.toggle_ui_element("grid"),
        Key::T => model.toggle_ui_element("trails"),
        Key::P => model.toggle_ui_element("pause"),
        Key::R => {
            // Reiniciar - limpiar todos los eventos
            model.events.clear();
            model.realtime_data = None;
            println!("🔄 Visualizador reiniciado");
        }
        Key::D => {
            // Debug - mostrar información detallada
            println!("🐛 DEBUG INFO:");
            println!("   Eventos activos: {}", model.events.len());
            println!("   FPS: {:.1}", model.stats.fps);
            println!("   Eventos por segundo: {:.1}", model.stats.events_per_second);
            if let Some(data) = &model.realtime_data {
                println!("   Último análisis: {:.1}Hz, {:.2}amp", data.frequency, data.amplitude);
            }
        }
        Key::Escape => {
            println!("👋 Cerrando visualizador...");
            app.quit();
        }
        _ => {}
    }
}

// =================================================================
// CONFIGURACIÓN DE NANNOU
// =================================================================

fn model_with_events(app: &App) -> Model {
    let config = load_config();
    
    let osc_receiver = setup_osc_receiver(config.osc.port)
        .expect("No se pudo configurar el receptor OSC");
    
    // Configurar ventana con eventos de teclado
    let _window = app
        .new_window()
        .title("SC Score Visualizer")
        .size(1200, 800)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    
    println!("🎹 Controles del teclado:");
    println!("   S - Mostrar/ocultar estadísticas");
    println!("   G - Mostrar/ocultar grilla");
    println!("   T - Mostrar/ocultar trails");
    println!("   P - Pausar/reanudar");
    println!("   R - Reiniciar");
    println!("   D - Información de debug");
    println!("   ESC - Salir");
    
    Model::new(config, osc_receiver)
}

// =================================================================
// ALTERNATIVA CON EVENTOS DE TECLADO
// =================================================================

#[allow(dead_code)]
fn main_with_keyboard() {
    println!("🎵 SC Score Visualizer - Versión con controles de teclado");
    
    nannou::app(model_with_events)
        .update(update)
        .run();
}
