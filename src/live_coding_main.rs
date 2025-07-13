// =============================================================================
// 🎵 SC SCORE VISUALIZER - LIVE CODING EDITION
// =============================================================================
// Optimizado para sesiones de live coding con respuesta ultra-rápida

use nannou::prelude::*;
use nannou_osc as osc;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::collections::VecDeque;
use noise::{NoiseFn, Perlin};
use serde::Deserialize;
use std::fs;

// =============================================================================
// 🎛️ CONFIGURACIÓN OPTIMIZADA PARA LIVE CODING
// =============================================================================

#[derive(Debug, Clone, Deserialize)]
struct LiveCodingConfig {
    // Parámetros de respuesta ultra-rápida
    max_events: usize,
    fade_time: f64,
    sensitivity: f64,
    visual_mode: String,
}

impl Default for LiveCodingConfig {
    fn default() -> Self {
        Self {
            max_events: 500,        // Más eventos para sesiones largas
            fade_time: 2.0,         // Desvanecimiento rápido
            sensitivity: 0.8,       // Alta sensibilidad
            visual_mode: "xenakis".to_string(), // Modo por defecto
        }
    }
}

// =============================================================================
// 🎨 EVENTO AUDIOVISUAL
// =============================================================================

#[derive(Debug, Clone)]
struct AudioEvent {
    timestamp: f64,
    freq: f64,
    amplitude: f64,
    event_type: EventType,
    x: f32,
    y: f32,
    color: Rgb,
    life: f64,
    max_life: f64,
}

#[derive(Debug, Clone)]
enum EventType {
    Point,
    Gliss { end_freq: f64 },
    Cluster { spread: f64, voices: usize },
}

// =============================================================================
// 🎛️ MODELO PRINCIPAL
// =============================================================================

struct Model {
    osc_receiver: Receiver<osc::Message>,
    events: VecDeque<AudioEvent>,
    config: LiveCodingConfig,
    noise: Perlin,
    time: f64,
    window_width: f32,
    window_height: f32,
    
    // Estadísticas en vivo
    event_count: usize,
    last_freq: f64,
    last_amp: f64,
    fps_counter: f32,
    
    // Modos visuales
    visual_mode: VisualMode,
}

#[derive(Debug, Clone)]
enum VisualMode {
    Xenakis,    // Curvas orgánicas
    Ikeda,      // Geometría minimalista  
    Henke,      // Grid dinámico
}

// =============================================================================
// 🚀 INICIALIZACIÓN ULTRA-RÁPIDA
// =============================================================================

fn main() {
    // Configuración optimizada para live coding
    nannou::app(model)
        .update(update)
        .view(view)
        .size(1200, 800)  // Tamaño fijo para consistencia
        .run();
}

fn model(app: &App) -> Model {
    println!("🎵 Iniciando Live Coding Visualizer...");
    
    // Crear ventana optimizada
    let _window = app
        .new_window()
        .title("🎵 Live Coding Visualizer")
        .size(1200, 800)
        .build()
        .unwrap();

    // Configurar OSC con puerto fijo para live coding
    let (sender, receiver) = mpsc::channel();
    let port = 57124u16;
    
    thread::spawn(move || {
        println!("🔌 OSC Listener iniciado en puerto {}", port);
        let osc_receiver = osc::receiver(port).unwrap();
        
        for packet in osc_receiver.iter() {
            let (packet, _addr) = packet;
            match packet {
                osc::Packet::Message(msg) => {
                    println!("📡 OSC message received: {:?}", msg);
                    if let Err(e) = sender.send(msg) {
                        eprintln!("❌ Error sending message: {}", e);
                    }
                },
                osc::Packet::Bundle(bundle) => {
                    println!("📦 OSC bundle received with {} elements", bundle.content.len());
                }
            }
        }
    });

    println!("✅ Live Coding Visualizer listo!");
    println!("🎛️ Puerto OSC: {}", port);
    println!("🎨 Modo visual: Xenakis");

    Model {
        osc_receiver: receiver,
        events: VecDeque::new(),
        config: LiveCodingConfig::default(),
        noise: Perlin::new(42),
        time: 0.0,
        window_width: 1200.0,
        window_height: 800.0,
        event_count: 0,
        last_freq: 0.0,
        last_amp: 0.0,
        fps_counter: 0.0,
        visual_mode: VisualMode::Xenakis,
    }
}

// =============================================================================
// 🔄 ACTUALIZACIÓN EN TIEMPO REAL
// =============================================================================

fn update(app: &App, model: &mut Model, _update: Update) {
    model.time = app.time as f64;
    model.fps_counter = app.fps();
    
    // Procesar mensajes OSC entrantes
    while let Ok(msg) = model.osc_receiver.try_recv() {
        process_osc_message(model, msg);
    }
    
    // Actualizar eventos existentes
    update_events(model);
    
    // Limpiar eventos antiguos
    cleanup_old_events(model);
}

fn process_osc_message(model: &mut Model, msg: osc::Message) {
    match msg.addr.as_str() {
        "/realtime_audio" => {
            if let Some(freq) = extract_f32_arg(&msg, 0) {
                if let Some(amp) = extract_f32_arg(&msg, 1) {
                    create_audio_event(model, freq as f64, amp as f64);
                }
            }
        }
        "/event" => {
            process_complex_event(model, &msg);
        }
        _ => {} // Ignorar otros mensajes
    }
}

fn extract_f32_arg(msg: &osc::Message, index: usize) -> Option<f32> {
    match msg.args.as_ref()?.get(index)? {
        osc::Type::Float(f) => Some(*f),
        osc::Type::Int(i) => Some(*i as f32),
        _ => None,
    }
}

fn create_audio_event(model: &mut Model, freq: f64, amp: f64) {
    // Filtrar eventos de baja energía en live coding
    if amp < 0.01 {
        return;
    }
    
    model.last_freq = freq;
    model.last_amp = amp;
    model.event_count += 1;
    
    // Mapeo visual optimizado para live coding
    let x = map_freq_to_x(freq, model.window_width);
    let y = map_amp_to_y(amp, model.window_height);
    let color = freq_to_color(freq);
    
    let event = AudioEvent {
        timestamp: model.time,
        freq,
        amplitude: amp,
        event_type: EventType::Point,
        x,
        y,
        color,
        life: model.config.fade_time,
        max_life: model.config.fade_time,
    };
    
    model.events.push_back(event);
    
    // Limitar eventos para performance en live coding
    while model.events.len() > model.config.max_events {
        model.events.pop_front();
    }
}

fn process_complex_event(model: &mut Model, msg: &osc::Message) {
    // Procesar eventos complejos (gliss, cluster) para live coding avanzado
    if let Some(args) = msg.args.as_ref() {
        if let Some(osc::Type::String(event_type)) = args.get(0) {
        match event_type.as_str() {
            "gliss" => {
                if let (Some(start_freq), Some(end_freq), Some(amp)) = (
                    extract_f32_arg(msg, 1),
                    extract_f32_arg(msg, 2), 
                    extract_f32_arg(msg, 4)
                ) {
                    create_gliss_event(model, start_freq as f64, end_freq as f64, amp as f64);
                }
            }
            "cluster" => {
                if let (Some(center_freq), Some(spread), Some(amp)) = (
                    extract_f32_arg(msg, 1),
                    extract_f32_arg(msg, 2),
                    extract_f32_arg(msg, 4)
                ) {
                    create_cluster_event(model, center_freq as f64, spread as f64, amp as f64);
                }
            }
            _ => {}
        }
        }
    }
}

fn create_gliss_event(model: &mut Model, start_freq: f64, end_freq: f64, amp: f64) {
    let x = map_freq_to_x(start_freq, model.window_width);
    let y = map_amp_to_y(amp, model.window_height);
    let color = freq_to_color(start_freq);
    
    let event = AudioEvent {
        timestamp: model.time,
        freq: start_freq,
        amplitude: amp,
        event_type: EventType::Gliss { end_freq },
        x,
        y,
        color,
        life: 4.0, // Gliss duran más
        max_life: 4.0,
    };
    
    model.events.push_back(event);
}

fn create_cluster_event(model: &mut Model, center_freq: f64, spread: f64, amp: f64) {
    let voices = (spread / 50.0).max(3.0).min(12.0) as usize; // 3-12 voces
    
    for i in 0..voices {
        let freq = center_freq + (i as f64 - voices as f64 / 2.0) * spread / voices as f64;
        let x = map_freq_to_x(freq, model.window_width);
        let y = map_amp_to_y(amp, model.window_height) + (i as f32 - voices as f32 / 2.0) * 5.0;
        let color = freq_to_color(freq);
        
        let event = AudioEvent {
            timestamp: model.time,
            freq,
            amplitude: amp / voices as f64,
            event_type: EventType::Cluster { spread, voices },
            x,
            y,
            color,
            life: 3.0,
            max_life: 3.0,
        };
        
        model.events.push_back(event);
    }
}

// =============================================================================
// 🎨 MAPEO VISUAL OPTIMIZADO
// =============================================================================

fn map_freq_to_x(freq: f64, width: f32) -> f32 {
    // Mapeo logarítmico optimizado para live coding
    let log_freq = freq.max(20.0).log2();
    let log_min = 20.0f64.log2();  // ~4.32
    let log_max = 20000.0f64.log2(); // ~14.29
    
    let normalized = (log_freq - log_min) / (log_max - log_min);
    let x = (normalized * width as f64) as f32;
    
    x.clamp(0.0, width)
}

fn map_amp_to_y(amp: f64, height: f32) -> f32 {
    // Mapeo no-lineal para mejor distribución visual
    let normalized = amp.sqrt(); // Raíz cuadrada para mejor distribución
    (normalized * height as f64 * 0.8) as f32 + height * 0.1
}

fn freq_to_color(freq: f64) -> Rgb {
    // Mapeo musical de frecuencias a colores
    let log_freq = freq.max(20.0).log2();
    let hue = ((log_freq - 4.0) / 10.0 * 360.0) % 360.0;
    
    hsv(hue as f32, 0.8, 0.9).into()
}

// =============================================================================
// 🔄 GESTIÓN DE EVENTOS
// =============================================================================

fn update_events(model: &mut Model) {
    let dt = 1.0 / 60.0; // Asumiendo 60 FPS
    
    for event in &mut model.events {
        event.life -= dt;
    }
}

fn cleanup_old_events(model: &mut Model) {
    model.events.retain(|event| event.life > 0.0);
}

// =============================================================================
// 🎨 RENDERIZADO OPTIMIZADO
// =============================================================================

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    // Fondo oscuro para live coding
    draw.background().color(rgb(0.1, 0.12, 0.15));
    
    // Renderizar eventos según el modo visual
    match model.visual_mode {
        VisualMode::Xenakis => render_xenakis_style(&draw, model),
        VisualMode::Ikeda => render_ikeda_style(&draw, model),
        VisualMode::Henke => render_henke_style(&draw, model),
    }
    
    // Panel de información para live coding
    render_live_info_panel(&draw, model, app);
    
    draw.to_frame(app, &frame).unwrap();
}

fn render_xenakis_style(draw: &Draw, model: &Model) {
    // Estilo orgánico tipo Xenakis
    for event in &model.events {
        let alpha = (event.life / event.max_life) as f32;
        let size = (event.amplitude * 20.0) as f32 * alpha;
        
        // Círculo principal
        draw.ellipse()
            .x_y(event.x - model.window_width / 2.0, event.y - model.window_height / 2.0)
            .radius(size)
            .color(rgba(event.color.red, event.color.green, event.color.blue, alpha * 0.8));
        
        // Glow effect
        draw.ellipse()
            .x_y(event.x - model.window_width / 2.0, event.y - model.window_height / 2.0)
            .radius(size * 2.0)
            .color(rgba(event.color.red, event.color.green, event.color.blue, alpha * 0.2));
    }
}

fn render_ikeda_style(draw: &Draw, model: &Model) {
    // Estilo minimalista tipo Ikeda
    for event in &model.events {
        let alpha = (event.life / event.max_life) as f32;
        let size = (event.amplitude * 15.0) as f32;
        
        // Punto preciso
        draw.rect()
            .x_y(event.x - model.window_width / 2.0, event.y - model.window_height / 2.0)
            .w_h(size, size)
            .color(rgba(1.0, 1.0, 1.0, alpha));
    }
}

fn render_henke_style(draw: &Draw, model: &Model) {
    // Estilo grid tipo Henke
    // Grid de fondo
    let grid_size = 50.0;
    for x in (-6..=6).map(|i| i as f32 * grid_size) {
        draw.line()
            .start(pt2(x, -model.window_height / 2.0))
            .end(pt2(x, model.window_height / 2.0))
            .color(rgba(0.3, 0.3, 0.3, 0.3))
            .weight(1.0);
    }
    
    for y in (-4..=4).map(|i| i as f32 * grid_size) {
        draw.line()
            .start(pt2(-model.window_width / 2.0, y))
            .end(pt2(model.window_width / 2.0, y))
            .color(rgba(0.3, 0.3, 0.3, 0.3))
            .weight(1.0);
    }
    
    // Eventos en el grid
    for event in &model.events {
        let alpha = (event.life / event.max_life) as f32;
        let size = (event.amplitude * 25.0) as f32;
        
        draw.ellipse()
            .x_y(event.x - model.window_width / 2.0, event.y - model.window_height / 2.0)
            .radius(size)
            .color(rgba(event.color.red, event.color.green, event.color.blue, alpha * 0.9));
    }
}

fn render_live_info_panel(draw: &Draw, model: &Model, app: &App) {
    // Panel de información para live coding (más compacto)
    let panel_x = model.window_width / 2.0 - 150.0;
    let panel_y = model.window_height / 2.0 - 50.0;
    
    // Fondo del panel
    draw.rect()
        .x_y(panel_x, panel_y)
        .w_h(280.0, 80.0)
        .color(rgba(0.0, 0.0, 0.0, 0.8));
    
    // Información esencial para live coding
    let info_lines = vec![
        format!("🎵 Events: {}", model.event_count),
        format!("🎛️ Freq: {:.0} Hz", model.last_freq),
        format!("🔊 Amp: {:.2}", model.last_amp),
        format!("⚡ FPS: {:.0}", model.fps_counter),
    ];
    
    for (i, line) in info_lines.iter().enumerate() {
        draw.text(line)
            .x_y(panel_x - 120.0, panel_y + 25.0 - i as f32 * 18.0)
            .font_size(12)
            .color(rgba(0.9, 0.95, 1.0, 0.9))
            .left_justify();
    }
}
