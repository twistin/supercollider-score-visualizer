
use nannou::prelude::*;
use nannou_osc as osc;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::collections::VecDeque;
use noise::{NoiseFn, Perlin};
use serde::Deserialize;
use std::fs;
use nannou_osc::Packet;

// =====================================================================
// 🎵 CONFIGURACIÓN Y MODELO DE DATOS
// =====================================================================

#[derive(Debug, Clone, Deserialize)]
struct Config {
    visual: VisualSettings,
    osc: OscSettings,
}

#[derive(Debug, Clone, Deserialize)]
struct VisualSettings {
    time_window: f64,
    max_events: usize,
    background_color: [u8; 3],
    event_fade_time: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct OscSettings {
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            visual: VisualSettings {
                time_window: 10.0,
                max_events: 200,
                background_color: [25, 28, 32], // Fondo oscuro por defecto
                event_fade_time: 3.0,
            },
            osc: OscSettings {
                port: 57124, // Puerto correcto por defecto
            },
        }
    }
}

#[derive(Debug, Clone)]
struct MusicalEvent {
    event_type: EventType,
    start_time: f64,
    duration: f64,
    amplitude: f32,
    // Atributos estéticos
    color: Hsv,
    shape_seed: f64,
}

#[derive(Debug, Clone)]
enum EventType {
    // Eventos manuales originales
    Point { freq: f32 },
    Glissando { start_freq: f32, end_freq: f32, curvature: f32 },
    Cluster { center_freq: f32, spread: f32, voices: u32 },
    Noise { center_freq: f32, bandwidth: f32 },
    SoundMass { components: Vec<(f32, f32)> },
    
    // Nuevos eventos de análisis en tiempo real
    RealtimeAudio {
        pitch: f32,
        amplitude: f32,
        onset: f32,
        has_freq: f32,
        spectral_centroid: f32,
        spectral_flux: f32,
        spectral_rolloff: f32,
        spectral_flatness: f32,
        harmonicity: f32,
        noisiness: f32,
        spectral_slope: f32,
    },
}

#[derive(Debug, Clone)]
struct AudioAnalysis {
    pitch: f32,
    amplitude: f32,
    onset: f32,
    has_freq: f32,
    spectral_centroid: f32,
    spectral_flux: f32,
    spectral_rolloff: f32,
    spectral_flatness: f32,
    harmonicity: f32,
    noisiness: f32,
    spectral_slope: f32,
    timestamp: f64,
}

struct Model {
    config: Config,
    events: VecDeque<MusicalEvent>,
    osc_receiver: Receiver<osc::Message>,
    app_start_time: f64,
    perlin: Perlin,
    // Nuevo: buffer de análisis de audio en tiempo real
    audio_analysis_buffer: VecDeque<AudioAnalysis>,
    last_audio_analysis: Option<AudioAnalysis>,
}

// =====================================================================
// 🚀 INICIALIZACIÓN
// =====================================================================

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // Cargar configuración desde config.toml
    let config = match load_config() {
        Ok(cfg) => {
            println!("✅ Configuración 'config.toml' cargada exitosamente.");
            cfg
        }
        Err(e) => {
            eprintln!("⚠️ No se pudo cargar 'config.toml': {}. Usando configuración por defecto.", e);
            Config::default()
        }
    };

    // Crear ventana con configuración compatible
    app.new_window()
        .size(1200, 700)
        .view(view)
        .title("SC Score Visualizer [Estética Profesional]")
        .build()
        .unwrap();

    // Configurar receptor OSC
    let (osc_sender, osc_receiver) = mpsc::channel();
    let osc_port = config.osc.port;

    thread::spawn(move || {
        let receiver = osc::receiver(osc_port).expect("No se pudo crear el receptor OSC");
        println!("📡 Receptor OSC activo en puerto {}", osc_port);
        for (packet, _addr) in receiver.iter() {
            match packet {
                Packet::Message(msg) => {
                    if osc_sender.send(msg).is_err() {
                        break;
                    }
                }
                Packet::Bundle(_bundle) => {
                    // Skip bundle processing for now - just handle individual messages
                    // This is a simplified implementation
                }
            }
        }
    });

    Model {
        events: VecDeque::new(),
        app_start_time: app.time as f64,
        perlin: Perlin::new(rand::random()),
        config,
        osc_receiver,
        audio_analysis_buffer: VecDeque::new(),
        last_audio_analysis: None,
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

// =====================================================================
// 🔄 LÓGICA DE ACTUALIZACIÓN
// =====================================================================

fn update(app: &App, model: &mut Model, _update: Update) {
    let current_time = app.time as f64 - model.app_start_time;

    // Procesar nuevos eventos OSC
    while let Ok(msg) = model.osc_receiver.try_recv() {
        
        match msg.addr.as_str() {
            "/realtime_audio" => {
                // Procesar datos de análisis de audio en tiempo real
                if let Some(analysis) = parse_realtime_audio(&msg, current_time) {
                    model.audio_analysis_buffer.push_back(analysis.clone());
                    model.last_audio_analysis = Some(analysis);
                    
                    // Mantener solo los últimos 2 segundos de análisis
                    let buffer_duration = 2.0;
                    model.audio_analysis_buffer.retain(|a| {
                        current_time - a.timestamp < buffer_duration
                    });
                }
            },
            "/event" => {
                // Procesar eventos manuales tradicionales
                if let Some(event) = parse_osc_message(&msg, current_time) {
                    model.events.push_back(event);
                }
            },
            _ => {
                // Otros tipos de mensajes
                if let Some(event) = parse_osc_message(&msg, current_time) {
                    model.events.push_back(event);
                }
            }
        }
    }

    // Limpiar eventos viejos que ya terminaron de desvanecerse
    let fade_time = model.config.visual.event_fade_time;
    model.events.retain(|event| {
        let event_end_time = event.start_time + event.duration + fade_time;
        event_end_time > current_time
    });

    // Limitar el número máximo de eventos más agresivamente para estabilidad
    let safe_max_events = model.config.visual.max_events.min(100);
    while model.events.len() > safe_max_events {
        model.events.pop_front();
    }
}

// =====================================================================
// 🎨 LÓGICA DE DIBUJO (VIEW)
// =====================================================================

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let current_time = app.time as f64 - model.app_start_time;

    // Debug del estado de renderizado - removed for performance

    // Fondo con gradiente sutil
    let bg = model.config.visual.background_color;
    draw.background().color(rgb8(bg[0], bg[1], bg[2]));

    // Dibujar rejilla profesional
    draw_grid(&draw, &win);

    // Dibujar cada evento (optimizado para rendimiento)
    let max_render_events = 100; // Increased from 50 for better visualization
    for event in model.events.iter().take(max_render_events) {
        render_event(&draw, &win, event, current_time, &model.config, &model.perlin);
    }

    // Renderizar análisis de audio en tiempo real
    render_realtime_audio(&draw, &win, model, current_time);

    // Panel de información PERFECTAMENTE CENTRADO
    let panel_width = 290.0;
    let panel_height = 100.0; // Incrementado para la línea adicional
    let panel_x = win.right() - panel_width / 2.0 - 20.0;
    let panel_y = win.top() - panel_height / 2.0 - 20.0;

    // Fondo del panel con borde azul homogéneo
    draw.rect()
        .x_y(panel_x, panel_y)
        .w_h(panel_width, panel_height)
        .color(rgba(0.0, 0.0, 0.0, 0.88))
        .stroke(rgba(0.4, 0.6, 0.9, 0.9))
        .stroke_weight(2.5);

    let status_lines = [
        "🎵 SC Score Visualizer".to_string(),
        format!("📊 Eventos: {}", model.events.len()),
        format!("⏱️  Tiempo: {:.1}s", current_time),
        format!("📡 OSC: {}", model.config.osc.port),
        // Agregar información de análisis en tiempo real
        if let Some(ref analysis) = model.last_audio_analysis {
            format!("🎼 Audio: {:.0}Hz {:.3}amp", analysis.pitch, analysis.amplitude)
        } else {
            "🎼 Audio: Sin señal".to_string()
        },
    ];

    // CENTRADO PERFECTO: Calcular el centro del bloque de texto
    let line_spacing = 15.0;
    let total_text_height = (status_lines.len() as f32 - 1.0) * line_spacing;
    let text_center_y = panel_y; // Centro vertical del panel
    let text_start_y = text_center_y + (total_text_height / 2.0);

    // Renderizar cada línea centrada horizontalmente y verticalmente
    for (i, line) in status_lines.iter().enumerate() {
        let text_y = text_start_y - (i as f32 * line_spacing);
        
        draw.text(line)
            .xy(pt2(panel_x, text_y)) // Centrado horizontalmente en panel_x
            .font_size(11)
            .color(rgba(0.5, 0.7, 0.95, 0.95)) // Color azul homogéneo con el borde
            .center_justify(); // Centrado horizontal automático
    }

    draw.to_frame(app, &frame).unwrap();
}

// =====================================================================
// ✨ RENDERIZADO DE EVENTOS
// =====================================================================

fn draw_grid(draw: &Draw, win: &Rect) {
    // Grilla profesional mejorada con más líneas de frecuencia
    
    // Líneas verticales de tiempo (reducidas para rendimiento)
    let main_step_x = 120.0;
    let main_line_color = rgba(0.4, 0.6, 0.8, 0.35);
    let count_main_x = (win.w() / main_step_x) as i32;
    
    for i in 0..=count_main_x.min(12) {
        let x = win.left() + i as f32 * main_step_x;
        draw.line()
            .start(pt2(x, win.bottom()))
            .end(pt2(x, win.top()))
            .stroke_weight(1.0)
            .color(main_line_color);
    }

    // GRILLA DE FRECUENCIAS MEJORADA - Más líneas para mejor referencia
    // Líneas principales de frecuencia (octavas)
    let main_freq_markers = [
        (55.0, "55Hz"),      // A1
        (110.0, "110"),      // A2  
        (220.0, "220"),      // A3
        (440.0, "440"),      // A4 (La central)
        (880.0, "880"),      // A5
        (1760.0, "1.76k"),   // A6
        (3520.0, "3.52k"),   // A7
        (7040.0, "7k"),      // A8
    ];
    
    // Líneas principales más gruesas y visibles
    for (freq, label) in &main_freq_markers {
        let y = freq_to_y(*freq, win);
        
        // Línea principal de frecuencia
        draw.line()
            .start(pt2(win.left(), y))
            .end(pt2(win.right(), y))
            .stroke_weight(1.2)
            .color(rgba(0.5, 0.7, 0.9, 0.4));
            
        // Etiqueta de frecuencia
        draw.text(label)
            .x_y(win.left() + 25.0, y + 6.0)
            .font_size(10)
            .color(rgba(0.8, 0.9, 1.0, 0.7));
    }
    
    // Líneas secundarias de frecuencia (semitonos importantes)
    let secondary_freq_markers = [
        65.4,   // C2
        73.4,   // D2
        82.4,   // E2
        98.0,   // G2
        130.8,  // C3
        146.8,  // D3
        164.8,  // E3
        196.0,  // G3
        261.6,  // C4 (Do central)
        293.7,  // D4
        329.6,  // E4
        392.0,  // G4
        523.3,  // C5
        587.3,  // D5
        659.3,  // E5
        784.0,  // G5
        1046.5, // C6
        1174.7, // D6
        1318.5, // E6
        1568.0, // G6
        2093.0, // C7
        2349.3, // D7
        2637.0, // E7
        3136.0, // G7
        4186.0, // C8
    ];
    
    // Líneas secundarias más sutiles
    for freq in &secondary_freq_markers {
        let y = freq_to_y(*freq, win);
        
        draw.line()
            .start(pt2(win.left(), y))
            .end(pt2(win.right(), y))
            .stroke_weight(0.5)
            .color(rgba(0.4, 0.6, 0.8, 0.15));
    }
}

fn render_event(draw: &Draw, win: &Rect, event: &MusicalEvent, current_time: f64, config: &Config, perlin: &Perlin) {
    let event_age = current_time - event.start_time;
    if event_age < 0.0 { return; }

    // Add bounds checking to prevent graphics issues
    if !event_age.is_finite() || event_age > 1000.0 {
        return;
    }

    // CORREGIDO: Calcular la posición X correctamente (de izquierda a derecha como un score tradicional)
    let time_window = config.visual.time_window;
    
    // Los eventos nuevos aparecen en la izquierda y se mueven hacia la derecha
    let time_progress = event_age / time_window;
    let x = map_range(time_progress, 0.0, 1.0, win.left() + 50.0, win.right() - 50.0);

    // No renderizar si está fuera de la pantalla
    if x < win.left() - 100.0 || x > win.right() + 100.0 {
        return;
    }

    // Calcular alpha para desvanecimiento cuando se acerca al borde derecho
    let fade_start_x = win.right() - 150.0; // Empieza a desvanecerse cerca del borde derecho
    let alpha = if x > fade_start_x {
        map_range(x, fade_start_x, win.right(), 1.0, 0.0).max(0.0)
    } else {
        1.0 // Completamente visible hasta llegar al área de desvanecimiento
    };

    if alpha <= 0.01 { return; }

    let color_with_alpha = hsva(event.color.hue.into(), event.color.saturation, event.color.value, alpha);

    match &event.event_type {
        EventType::Point { freq } => {
            let y = freq_to_y(*freq, win);
            let base_radius = (event.amplitude * 15.0 + 3.0).clamp(1.0, 50.0);
            
            // Simplified rendering to reduce GPU load
            let radius = base_radius;

            // Simple single circle to reduce GPU load
            draw.ellipse()
                .x_y(x, y)
                .radius(radius)
                .color(color_with_alpha);
        }
        EventType::Glissando { start_freq, end_freq, curvature } => {
            let segments = 10; // Reduced segments for better performance
            let mut points_vec = Vec::new();
            
            // Dibujar glissando que se extiende en el tiempo (de izquierda a derecha)
            let gliss_width = 100.0; // Ancho del glissando en pixels
            let start_x = x - gliss_width / 2.0;
            let end_x = x + gliss_width / 2.0;
            
            for i in 0..=segments {
                let t = i as f32 / segments as f32;
                
                // Interpolar frecuencia con curvatura
                let t_curved = if *curvature != 0.0 {
                    t.powf(1.0 + curvature * 2.0)
                } else {
                    t
                };
                let freq = map_range(t_curved, 0.0, 1.0, *start_freq, *end_freq);
                let y = freq_to_y(freq, win);
                
                // Posición X a lo largo del glissando
                let segment_x = map_range(t, 0.0, 1.0, start_x, end_x);
                points_vec.push(pt2(segment_x, y));
            }

            // Dibujar línea con grosor variable
            for i in 0..points_vec.len() - 1 {
                let segment_alpha = alpha * (1.0 - (i as f32 / points_vec.len() as f32) * 0.2);
                draw.line()
                    .start(points_vec[i])
                    .end(points_vec[i+1])
                    .color(hsva(event.color.hue.into(), event.color.saturation, event.color.value, segment_alpha))
                    .stroke_weight(event.amplitude * 6.0 + 2.0);
            }
        }
        EventType::Cluster { center_freq, spread, voices } => {
            let cluster_width = 40.0;
            for i in 0..*voices {
                let noise_x = perlin.get([event.shape_seed + i as f64 * 0.1, event_age * 0.3]) as f32;
                let noise_y = perlin.get([event.shape_seed + i as f64 * 0.1 + 100.0, event_age * 0.2]) as f32;
                
                let freq_offset = (noise_y - 0.5) * 2.0 * spread;
                let y = freq_to_y(center_freq + freq_offset, win);
                let voice_x = x + (noise_x - 0.5) * cluster_width;
                
                let radius = event.amplitude * 8.0 + 2.0;

                // Partícula individual del cluster
                draw.ellipse()
                    .x_y(voice_x, y)
                    .radius(radius)
                    .color(hsva(event.color.hue.into(), event.color.saturation, event.color.value, alpha * 0.8));
            }
        }
        EventType::Noise { center_freq, bandwidth } => {
            // Renderizar ruido como múltiples puntos aleatorios
            let noise_points = (bandwidth * 0.1) as usize + 10;
            for _i in 0..noise_points {
                let freq_variation = (rand::random::<f32>() - 0.5) * bandwidth;
                let freq = center_freq + freq_variation;
                let y = freq_to_y(freq, win);
                
                let noise_x = x + (rand::random::<f32>() - 0.5) * 40.0;
                let radius = event.amplitude * 3.0 + 1.0;
                
                draw.ellipse()
                    .x_y(noise_x, y)
                    .radius(radius)
                    .color(hsva(event.color.hue.into(), event.color.saturation * 0.7, event.color.value, alpha * 0.6));
            }
        }
        EventType::SoundMass { components } => {
            // Renderizar masa sonora como múltiples componentes conectadas
            for (i, (freq, amp)) in components.iter().enumerate() {
                let y = freq_to_y(*freq, win);
                let radius = (amp * event.amplitude * 12.0 + 2.0).clamp(1.0, 20.0);
                
                // Posición ligeramente desplazada para cada componente
                let component_x = x + (i as f32 - components.len() as f32 / 2.0) * 8.0;
                
                draw.ellipse()
                    .x_y(component_x, y)
                    .radius(radius)
                    .color(hsva(event.color.hue.into(), event.color.saturation, event.color.value, alpha * amp));
                
                // Conectar componentes con líneas
                if i > 0 {
                    let prev_freq = components[i - 1].0;
                    let prev_y = freq_to_y(prev_freq, win);
                    let prev_x = x + ((i - 1) as f32 - components.len() as f32 / 2.0) * 8.0;
                    
                    draw.line()
                        .start(pt2(prev_x, prev_y))
                        .end(pt2(component_x, y))
                        .color(hsva(event.color.hue.into(), event.color.saturation, event.color.value, alpha * 0.3))
                        .stroke_weight(1.0);
                }
            }
        }
        EventType::RealtimeAudio { pitch, amplitude, onset, has_freq, spectral_centroid: _, spectral_flux: _, spectral_rolloff: _, spectral_flatness: _, harmonicity, noisiness: _, spectral_slope: _ } => {
            // Renderizar datos de audio en tiempo real como visualización compleja
            if *has_freq > 0.5 {
                let y = freq_to_y(*pitch, win);
                let radius = amplitude * 20.0 + 5.0;
                
                // Color basado en harmonicidad
                let hue = if *harmonicity > 0.7 { 0.6 } else { 0.1 }; // Azul para tonal, rojo para ruidoso
                
                draw.ellipse()
                    .x_y(x, y)
                    .radius(radius)
                    .color(hsva(hue, 0.8, 0.9, alpha * amplitude));
                
                // Onset como flash
                if *onset > 0.5 {
                    draw.ellipse()
                        .x_y(x, y)
                        .radius(radius * 2.0)
                        .color(hsva(hue, 0.4, 1.0, alpha * onset * 0.5));
                }
            }
        }
        // Implementar Noise y SoundMass si es necesario
    }
}

fn render_realtime_audio(draw: &Draw, win: &Rect, model: &Model, current_time: f64) {
    // Solo renderizar si tenemos datos recientes de análisis
    if let Some(ref analysis) = model.last_audio_analysis {
        let data_age = current_time - analysis.timestamp;
        
        // Solo mostrar datos de los últimos 100ms para evitar lag visual
        if data_age > 0.1 {
            return;
        }
        
        // Solo renderizar si hay actividad significativa (threshold más permisivo)
        if analysis.amplitude < 0.001 {
            return;
        }
        
        
        // === RENDERIZADO ESTILO XENAKIS ===
        render_xenakis_style(draw, win, analysis, current_time);
        
        // === RENDERIZADO ESTILO IKEDA ===
        render_ikeda_style(draw, win, analysis);
        
        // === HUD DE ANÁLISIS (ESQUINA INFERIOR IZQUIERDA) ===
        render_analysis_hud(draw, win, analysis);
    }
}

fn render_xenakis_style(draw: &Draw, win: &Rect, analysis: &AudioAnalysis, _current_time: f64) {
    if analysis.has_freq > 0.5 && analysis.pitch > 60.0 {
        // Renderizar pitch como partícula brillante en movimiento
        let y = freq_to_y(analysis.pitch, win);
        let x = win.right() - 100.0; // Lado derecho, se mueve hacia la izquierda
        
        let radius = analysis.amplitude * 30.0 + 3.0;
        let color = freq_to_color(analysis.pitch);
        
        // Uso de spectral_flux para modificar la intensidad del glow
        let glow_intensity = (analysis.spectral_flux * 2.0).clamp(0.0, 1.0);
        
        // Partícula principal con glow variable basado en flux
        draw.ellipse()
            .x_y(x, y)
            .radius(radius * (2.0 + glow_intensity))
            .color(hsva(color.hue.into(), color.saturation * 0.3, color.value * 0.5, 0.3 * glow_intensity));
            
        draw.ellipse()
            .x_y(x, y)
            .radius(radius)
            .color(hsva(color.hue.into(), color.saturation, color.value, analysis.amplitude));
        
        // Uso de spectral_rolloff para modificar el trail
        let trail_length = ((analysis.spectral_rolloff / 2000.0) * 15.0).clamp(5.0, 20.0) as usize;
        
        // Estela de movimiento (trail effect) usando spectral_flatness
        for i in 0..trail_length {
            let trail_x = x + (i as f32 * 8.0);
            let trail_alpha = analysis.amplitude * (1.0 - i as f32 / trail_length as f32) * 0.5;
            let trail_radius = radius * (1.0 - i as f32 / trail_length as f32 * 0.7);
            
            // Usar spectral_flatness para la distorsión del trail
            let y_offset = analysis.spectral_flatness * 10.0 * ((i as f32 / 3.0).sin());
            
            if trail_alpha > 0.01 {
                draw.ellipse()
                    .x_y(trail_x, y + y_offset)
                    .radius(trail_radius)
                    .color(hsva(color.hue.into(), color.saturation, color.value, trail_alpha));
            }
        }
    }
    
    // Renderizar ruido como textura granular usando spectral_slope
    if analysis.noisiness > 0.3 {
        let noise_particles = (analysis.noisiness * 20.0) as usize;
        for i in 0..noise_particles {
            let x = win.left() + (i as f32 / noise_particles as f32) * win.w();
            let y = win.bottom() + rand::random::<f32>() * win.h();
            let size = analysis.amplitude * analysis.noisiness * 5.0;
            
            // Usar spectral_slope para afectar el color del ruido
            let slope_hue = (analysis.spectral_slope + 1.0) * 0.5; // Normalize to 0-1
            
            draw.ellipse()
                .x_y(x, y)
                .radius(size)
                .color(hsva(slope_hue, 0.6, 0.9, analysis.amplitude * analysis.noisiness * 0.3));
        }
    }
}

fn render_ikeda_style(draw: &Draw, win: &Rect, analysis: &AudioAnalysis) {
    // Onset como flash estroboscópico
    if analysis.onset > 0.0 {
        let flash_intensity = analysis.onset * analysis.amplitude;
        
        // Flash de pantalla completa
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w(), win.h())
            .color(rgba(1.0, 1.0, 1.0, flash_intensity * 0.1));
            
        // Líneas verticales precisas
        let line_count = (flash_intensity * 20.0) as usize;
        for i in 0..line_count {
            let x = map_range(i as f32, 0.0, line_count as f32, win.left(), win.right());
            draw.line()
                .start(pt2(x, win.bottom()))
                .end(pt2(x, win.top()))
                .stroke_weight(1.0)
                .color(rgba(1.0, 1.0, 1.0, flash_intensity * 0.8));
        }
    }
    
    // Representación digital del centroide espectral
    if analysis.spectral_centroid > 100.0 {
        let centroid_y = freq_to_y(analysis.spectral_centroid, win);
        let bar_width = analysis.amplitude * win.w() * 0.8;
        
        draw.rect()
            .x_y(0.0, centroid_y)
            .w_h(bar_width, 2.0)
            .color(rgba(0.0, 1.0, 1.0, analysis.amplitude * 0.7));
    }
}

fn render_analysis_hud(draw: &Draw, win: &Rect, analysis: &AudioAnalysis) {
    let hud_x = win.left() + 20.0;
    let hud_y = win.bottom() + 80.0;
    let line_height = 15.0;
    
    let hud_lines = [
        format!("🎵 Pitch: {:.0} Hz", analysis.pitch),
        format!("📊 Amp: {:.3}", analysis.amplitude),
        format!("🎯 Onset: {:.1}", analysis.onset),
        format!("🌈 Centroid: {:.0} Hz", analysis.spectral_centroid),
        format!("🎭 Harmonicity: {:.2}", analysis.harmonicity),
        format!("📡 Noisiness: {:.2}", analysis.noisiness),
    ];
    
    for (i, line) in hud_lines.iter().enumerate() {
        let text_y = hud_y + (i as f32 * line_height);
        draw.text(line)
            .xy(pt2(hud_x, text_y))
            .font_size(10)
            .color(rgba(0.7, 0.9, 1.0, 0.8));
    }
}

// =====================================================================
// 🛠️ UTILIDADES
// =====================================================================

/// Extrae un f32 de un argumento OSC, ya sea Int o Float.
fn get_float(arg: &osc::Type) -> Option<f32> {
    match arg {
        osc::Type::Float(f) => Some(*f),
        osc::Type::Int(i) => Some(*i as f32),
        _ => None,
    }
}

fn parse_osc_message(msg: &osc::Message, current_time: f64) -> Option<MusicalEvent> {
    if msg.addr != "/event" {
        return None;
    }

    let args = &msg.args;
    let event_name = args.get(0)?.clone().string()?;

    let event_type = match event_name.as_str() {
        "point" => EventType::Point {
            freq: get_float(args.get(1)?)?,
        },
        "gliss" => EventType::Glissando {
            start_freq: get_float(args.get(1)?)?,
            end_freq: get_float(args.get(2)?)?,
            curvature: get_float(args.get(4)?).unwrap_or(0.0),
        },
        "cluster" => EventType::Cluster {
            center_freq: get_float(args.get(1)?)?,
            spread: get_float(args.get(2)?)?,
            voices: args.get(3)?.clone().int()? as u32,
        },
        _ => return None, // Ignorar otros tipos de eventos por ahora
    };

    let amplitude = match event_name.as_str() {
        "point" => get_float(args.get(2)?)?,
        "gliss" => get_float(args.get(3)?)?,
        "cluster" => get_float(args.get(4)?)?,
        _ => return None,
    };

    let duration = match event_name.as_str() {
        "point" => get_float(args.get(3)?)? as f64,
        "gliss" => get_float(args.get(4)?)? as f64,
        "cluster" => get_float(args.get(5)?)? as f64,
        _ => return None,
    };

    let freq_for_color = match event_type {
        EventType::Point { freq } => freq,
        EventType::Glissando { start_freq, .. } => start_freq,
        EventType::Cluster { center_freq, .. } => center_freq,
        _ => 440.0,
    };

    Some(MusicalEvent {
        event_type,
        start_time: current_time,
        duration,
        amplitude,
        color: freq_to_color(freq_for_color),
        shape_seed: rand::random(),
    })
}

fn parse_realtime_audio(msg: &osc::Message, current_time: f64) -> Option<AudioAnalysis> {
    if msg.addr != "/realtime_audio" {
        return None;
    }

    let args = &msg.args;
    
    // Verificar que tenemos al menos 8 argumentos básicos
    if args.len() < 8 {
        return None;
    }

    // Función auxiliar para validar valores
    let validate_float = |val: f32| -> f32 {
        if val.is_finite() { val } else { 0.0 }
    };

    Some(AudioAnalysis {
        pitch: validate_float(get_float(args.get(0)?).unwrap_or(0.0)),
        amplitude: validate_float(get_float(args.get(1)?).unwrap_or(0.0)),
        onset: validate_float(get_float(args.get(2)?).unwrap_or(0.0)),
        has_freq: validate_float(get_float(args.get(3)?).unwrap_or(1.0)),  // Asumimos que hay frecuencia por defecto
        spectral_centroid: validate_float(get_float(args.get(4)?).unwrap_or(1000.0)),
        spectral_flux: validate_float(get_float(args.get(5)?).unwrap_or(0.0)),
        spectral_rolloff: validate_float(get_float(args.get(6)?).unwrap_or(2000.0)),
        spectral_flatness: validate_float(get_float(args.get(7)?).unwrap_or(0.5)),
        harmonicity: validate_float(args.get(8).map_or(0.5, |arg| get_float(arg).unwrap_or(0.5))),
        noisiness: validate_float(args.get(9).map_or(0.5, |arg| get_float(arg).unwrap_or(0.5))),
        spectral_slope: validate_float(args.get(10).map_or(0.0, |arg| get_float(arg).unwrap_or(0.0))),
        timestamp: current_time,
    })
}

fn freq_to_y(freq: f32, win: &Rect) -> f32 {
    let min_freq_log = 20.0f32.log2();
    let max_freq_log = 20000.0f32.log2();
    let freq_log = freq.clamp(20.0, 20000.0).log2();
    map_range(freq_log, min_freq_log, max_freq_log, win.bottom(), win.top())
}



fn freq_to_color(freq: f32) -> Hsv {
    let min_log = 20.0f32.log10();
    let max_log = 20000.0f32.log10();
    let freq_log = freq.clamp(20.0, 20000.0).log10();
    
    // Mapeo mejorado de frecuencia a color más musical y vibrante
    let normalized = (freq_log - min_log) / (max_log - min_log);
    
    // Paleta de colores musical: graves=rojos, medios=verdes/amarillos, agudos=azules/violetas
    let hue = match normalized {
        x if x < 0.2 => map_range(x, 0.0, 0.2, 0.0, 0.08),      // Graves: rojo-naranja
        x if x < 0.4 => map_range(x, 0.2, 0.4, 0.08, 0.17),    // Medios bajos: naranja-amarillo
        x if x < 0.6 => map_range(x, 0.4, 0.6, 0.17, 0.33),    // Medios: amarillo-verde
        x if x < 0.8 => map_range(x, 0.6, 0.8, 0.33, 0.55),    // Medios altos: verde-cyan
        _ => map_range(normalized, 0.8, 1.0, 0.55, 0.75),      // Agudos: cyan-azul-violeta
    };
    
    // Saturación y brillo altos para colores vibrantes
    let saturation = 0.85 + (freq_log * 0.1).sin() * 0.1; // Ligera variación
    let value = 0.9 + (freq_log * 0.05).cos() * 0.1;     // Ligera variación en brillo
    
    hsv(hue, saturation.clamp(0.7, 1.0), value.clamp(0.8, 1.0))
}

// =====================================================================
// 🧪 UNIT TESTS
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use nannou_osc::Type;

    #[test]
    fn test_get_float() {
        assert_eq!(get_float(&Type::Float(1.5)), Some(1.5));
        assert_eq!(get_float(&Type::Int(42)), Some(42.0));
        assert_eq!(get_float(&Type::String("test".to_string())), None);
    }

    #[test]
    fn test_freq_to_color() {
        let color = freq_to_color(440.0);
        assert!(color.hue.to_positive_degrees() >= 0.0);
        assert!(color.hue.to_positive_degrees() <= 360.0);
        assert!(color.saturation >= 0.7);
        assert!(color.value >= 0.8);
    }

    #[test]
    fn test_freq_to_y() {
        let win = nannou::geom::Rect::from_w_h(800.0, 600.0);
        let y1 = freq_to_y(440.0, &win);
        let y2 = freq_to_y(880.0, &win);
        assert!(y2 > y1); // Higher frequency should have higher Y position
    }

    #[test]
    fn test_parse_realtime_audio() {
        let args = vec![
            Type::Float(440.0),  // pitch
            Type::Float(0.5),    // amplitude
            Type::Float(0.0),    // onset
            Type::Float(0.9),    // has_freq
            Type::Float(1000.0), // spectral_centroid
            Type::Float(0.1),    // spectral_flux
            Type::Float(2000.0), // spectral_rolloff
            Type::Float(0.3),    // spectral_flatness
        ];
        
        let msg = nannou_osc::Message {
            addr: "/realtime_audio".to_string(),
            args,
        };
        
        let result = parse_realtime_audio(&msg, 0.0);
        assert!(result.is_some());
        
        let analysis = result.unwrap();
        assert_eq!(analysis.pitch, 440.0);
        assert_eq!(analysis.amplitude, 0.5);
        assert_eq!(analysis.onset, 0.0);
        assert_eq!(analysis.has_freq, 0.9);
    }

    #[test]
    fn test_parse_realtime_audio_insufficient_args() {
        let args = vec![
            Type::Float(440.0),  // Only 1 argument, need at least 8
        ];
        
        let msg = nannou_osc::Message {
            addr: "/realtime_audio".to_string(),
            args,
        };
        
        let result = parse_realtime_audio(&msg, 0.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_realtime_audio_invalid_values() {
        let args = vec![
            Type::Float(f32::NAN),    // Invalid pitch
            Type::Float(0.5),         // amplitude
            Type::Float(f32::INFINITY), // Invalid onset
            Type::Float(0.9),         // has_freq
            Type::Float(1000.0),      // spectral_centroid
            Type::Float(0.1),         // spectral_flux
            Type::Float(2000.0),      // spectral_rolloff
            Type::Float(0.3),         // spectral_flatness
        ];
        
        let msg = nannou_osc::Message {
            addr: "/realtime_audio".to_string(),
            args,
        };
        
        let result = parse_realtime_audio(&msg, 0.0);
        assert!(result.is_some());
        
        let analysis = result.unwrap();
        assert_eq!(analysis.pitch, 0.0);  // NaN should be replaced with 0.0
        assert_eq!(analysis.onset, 0.0);  // Infinity should be replaced with 0.0
    }
}
