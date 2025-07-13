// src/visual.rs - Funciones de visualización, render, estilos y coordenadas

use nannou::prelude::*;
use crate::model::{Model, Note, DroneEvent, ClusterData, AnalysisData, BackgroundStyle, VisualQuality, DisplayMode, ScrollMode};
use crate::visual::audio_visual_mapping::{VisualNote, VisualStyle, TextureStyle};

pub mod audio_visual_mapping;
pub mod audio_visual_mapping_pro;
pub mod shader_manager;

// Re-exportar tipos importantes del shader_manager
pub use shader_manager::{
    ShaderManager, ShaderConfig, RenderQuality, AntiAliasingMode,
    string_to_event_kind, ShaderStats
};

/// Mapeo entre parámetros de audio y propiedades visuales
#[derive(Debug, Clone)]
pub struct AudioVisualMapping {
    pub freq_range: (f32, f32),
    pub amp_range: (f32, f32),
}

impl Default for AudioVisualMapping {
    fn default() -> Self {
        Self {
            freq_range: (100.0, 1200.0),
            amp_range: (0.0, 1.0),
        }
    }
}

impl AudioVisualMapping {
    /// Convierte frecuencia a posición Y en pantalla
    pub fn freq_to_y(&self, freq: f32, win: Rect) -> f32 {
        map_range(
            freq.clamp(self.freq_range.0, self.freq_range.1),
            self.freq_range.0, self.freq_range.1,
            win.bottom() + 50.0, win.top() - 50.0
        )
    }
    
    /// Convierte amplitud a tamaño visual
    pub fn amp_to_size(&self, amp: f32, min_size: f32, max_size: f32) -> f32 {
        map_range(amp, self.amp_range.0, self.amp_range.1, min_size, max_size)
    }
    
    /// Convierte frecuencia a matiz (hue) del color
    pub fn freq_to_hue(&self, freq: f32) -> f32 {
        map_range(
            freq.clamp(self.freq_range.0, self.freq_range.1),
            self.freq_range.0, self.freq_range.1,
            0.0, 0.8  // Evitar rojo (0.0) y violeta (1.0)
        )
    }
    
    /// Convierte frecuencia a color RGB
    pub fn freq_to_color(&self, freq: f32) -> Srgb<u8> {
        let hue = self.freq_to_hue(freq);
        Srgb::from_format(hsv(hue, 0.9, 1.0).into())
    }
    
    /// Convierte frecuencia a tamaño (inverso: frecuencias bajas = tamaño grande)
    pub fn freq_to_size(&self, freq: f32, min_size: f32, max_size: f32) -> f32 {
        map_range(freq, 200.0, 600.0, max_size, min_size).max(min_size)
    }
    
    /// Convierte amplitud a transparencia
    pub fn amp_to_alpha(&self, amp: f32) -> f32 {
        map_range(amp, 50.0, 800.0, 0.3, 1.0).clamp(0.3, 1.0)
    }
    
    /// Calcula fade basado en tiempo de vida
    pub fn time_to_fade(&self, time_alive: f32, duration: f32) -> f32 {
        map_range(time_alive, 0.0, duration, 1.0, 0.0).max(0.0)
    }
}

/// Reglas visuales específicas para tipos de eventos
pub struct EventVisualRules;

impl EventVisualRules {
    /// Configuración visual para percusión (tamaño, duración, estilo)
    pub fn percussion_config() -> (f32, f32, f32) {
        (8.0, 0.3, 1.5)  // size, duration, fade_speed
    }
    
    /// Configuración visual para drones (radio, persistencia, alfa)
    pub fn drone_config() -> (f32, f32, f32) {
        (80.0, 5.0, 0.7)  // radius, duration, alpha
    }
    
    /// Configuración visual para cluster (num_particles, spread, energy)
    pub fn cluster_config() -> (i32, f32, f32) {
        (50, 120.0, 0.8)  // particles, spread_radius, energy_factor
    }
}

/// Funciones auxiliares para efectos visuales
impl AudioVisualMapping {
    /// Genera posición aleatoria dentro de un círculo
    pub fn random_position_in_circle(center: Vec2, radius: f32) -> Vec2 {
        use nannou::prelude::*;
        let angle = random_range(0.0, TAU);
        let distance = random_range(0.0, radius);
        center + vec2(distance * angle.cos(), distance * angle.sin())
    }
    
    /// Genera variación de color basada en un color base
    pub fn color_variation(&self, base_hue: f32, variation: f32) -> f32 {
        let hue_shift = random_range(-variation, variation);
        (base_hue + hue_shift).rem_euclid(1.0)
    }
    
    /// Genera variación de transparencia
    pub fn alpha_variation(&self, base_alpha: f32, variation: f32) -> f32 {
        (base_alpha * random_range(1.0 - variation, 1.0 + variation)).clamp(0.0, 1.0)
    }
}

/// Función principal de renderizado - coordina todas las visualizaciones
pub fn draw_visualization(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    // Limpiar fondo según estilo configurado
    draw_background(&draw, win, &model.display_config.background_style);
    
    // Dibujar grid si está habilitado
    if model.display_config.show_grid {
        draw_grid(&draw, win);
    }
    
    // **Renderizar según modo de scroll**
    match model.scroll_mode {
        ScrollMode::Fixed => draw_fixed(&draw, model, win),
        ScrollMode::Scrolling => draw_scrolling(&draw, model, win),
    }
    
    // **Mostrar indicador de modo visual en pantalla**
    draw_visual_mode_indicator(&draw, model, win);
    
    // Mostrar información de debug si está habilitado
    if model.display_config.show_debug {
        draw_debug_info(&draw, model, win);
    }

    draw.to_frame(app, &frame).unwrap();
}

/// **Renderizado en modo fijo (sin scroll)**
fn draw_fixed(draw: &Draw, model: &Model, win: Rect) {
    // Renderizar según modo de display
    match model.display_mode {
        DisplayMode::Events => {
            // **Priorizar renderizado profesional con ShaderManager**
            draw_all_visual_notes(draw, model, win);
            // Mantener compatibilidad con notas legacy
            draw_notes_only(draw, &model.notes, win);
        }
        DisplayMode::Analysis => draw_analysis_only(draw, &model.analysis),
        DisplayMode::Drones => draw_drones_only(draw, &model.drone_events, &model.display_config.visual_quality),
        DisplayMode::Cluster => draw_cluster_only(draw, &model.cluster_data, &model.display_config.visual_quality),
        DisplayMode::Combined => {
            // Dibujar análisis de fondo
            draw_analysis(draw, &model.analysis);
            
            // Dibujar cluster
            draw_cluster(draw, &model.cluster_data, &model.display_config.visual_quality);
            
            // Dibujar drones
            for drone in &model.drone_events {
                draw_drone(draw, drone, &model.display_config.visual_quality);
            }
            
            // **Dibujar notas visuales usando renderizado profesional (prioridad alta)**
            draw_all_visual_notes(draw, model, win);
            
            // Dibujar notas legacy (para compatibilidad)
            for note in &model.notes {
                draw_note(draw, note, win);
            }
        }
    }
}

/// **Renderizado en modo scroll (con desplazamiento horizontal)**
fn draw_scrolling(draw: &Draw, model: &Model, win: Rect) {
    // En modo scroll, aplicamos las mismas reglas pero con consideraciones especiales
    // para elementos que salen de la pantalla
    
    // Área visible actual (con margen para elementos que entran/salen)
    let visible_margin = 100.0;
    let visible_left = win.left() - visible_margin;
    let visible_right = win.right() + visible_margin;
    
    match model.display_mode {
        DisplayMode::Events => {
            // Filtrar y dibujar solo notas visuales visibles
            draw_visible_visual_notes(draw, model, win, visible_left, visible_right);
            // Filtrar y dibujar solo notas legacy visibles
            draw_visible_notes_only(draw, &model.notes, win, visible_left, visible_right);
        }
        DisplayMode::Analysis => {
            // El análisis no se ve afectado por scroll
            draw_analysis_only(draw, &model.analysis);
        }
        DisplayMode::Drones => {
            // Filtrar drones visibles
            draw_visible_drones_only(draw, &model.drone_events, &model.display_config.visual_quality, visible_left, visible_right);
        }
        DisplayMode::Cluster => {
            // El cluster se mantiene fijo (no afectado por scroll)
            draw_cluster_only(draw, &model.cluster_data, &model.display_config.visual_quality);
        }
        DisplayMode::Combined => {
            // Análisis y cluster mantienen posición fija
            draw_analysis(draw, &model.analysis);
            draw_cluster(draw, &model.cluster_data, &model.display_config.visual_quality);
            
            // Solo elementos con posición específica se filtran por visibilidad
            draw_visible_drones(draw, &model.drone_events, &model.display_config.visual_quality, visible_left, visible_right);
            draw_visible_visual_notes(draw, model, win, visible_left, visible_right);
            draw_visible_notes(draw, &model.notes, win, visible_left, visible_right);
        }
    }
    
    // **Indicador visual de scroll** (línea de tiempo o indicador de posición)
    draw_scroll_indicator(draw, model, win);
}

/// Dibuja el fondo según el estilo configurado
fn draw_background(draw: &Draw, win: Rect, style: &BackgroundStyle) {
    match style {
        BackgroundStyle::Modern => {
            // Fondo degradado moderno
            draw.background().color(rgb(0.03, 0.06, 0.12));
            
            // Círculo central sutil
            let center_color = rgba(0.05, 0.08, 0.15, 0.8);
            draw.ellipse()
                .xy(pt2(0.0, 0.0))
                .radius(win.w().min(win.h()) * 0.3)
                .color(center_color);
        }
        BackgroundStyle::Simple => {
            draw.background().color(rgb(0.1, 0.1, 0.1));
        }
        BackgroundStyle::Gradient => {
            draw.background().color(rgb(0.02, 0.05, 0.1));
            // Círculos concéntricos para efecto degradado
            for i in 1..=5 {
                let radius = win.w().min(win.h()) * 0.1 * i as f32;
                let alpha = 0.1 / i as f32;
                draw.ellipse().xy(pt2(0.0, 0.0)).radius(radius).color(rgba(
                    0.1, 0.2, 0.4, alpha
                ));
            }
        }
        BackgroundStyle::None => {
            draw.background().color(BLACK);
        }
    }
}

/// Dibuja una grilla de referencia
fn draw_grid(draw: &Draw, win: Rect) {
    let main_line_color = rgba(0.4, 0.6, 0.8, 0.25);
    let spacing = 100.0;
    
    // Líneas verticales
    let mut x = win.left();
    while x <= win.right() {
        draw.line()
            .start(pt2(x, win.bottom()))
            .end(pt2(x, win.top()))
            .color(main_line_color);
        x += spacing;
    }
    
    // Líneas horizontales principales (cada 200Hz aprox)
    let freq_spacing = 200.0;
    let mapping = AudioVisualMapping::default();
    
    for i in 0..10 {
        let freq = 100.0 + i as f32 * freq_spacing;
        let y = mapping.freq_to_y(freq, win);
        
        if y >= win.bottom() && y <= win.top() {
            draw.line()
                .start(pt2(win.left(), y))
                .end(pt2(win.right(), y))
                .color(rgba(0.5, 0.7, 0.9, 0.3));
        }
    }
    
    // Línea central más visible
    draw.line()
        .start(pt2(win.left(), 0.0))
        .end(pt2(win.right(), 0.0))
        .color(rgba(0.8, 0.9, 1.0, 0.6));
}

/// Renderiza solo las notas musicales
fn draw_notes_only(draw: &Draw, notes: &[Note], win: Rect) {
    for note in notes {
        draw_note(draw, note, win);
    }
}

/// Renderiza solo el análisis continuo
fn draw_analysis_only(draw: &Draw, analysis: &AnalysisData) {
    draw_analysis(draw, analysis);
}

/// Renderiza solo los drones
fn draw_drones_only(draw: &Draw, drones: &[DroneEvent], quality: &VisualQuality) {
    for drone in drones {
        draw_drone(draw, drone, quality);
    }
}

/// Renderiza solo el cluster
fn draw_cluster_only(draw: &Draw, cluster: &ClusterData, quality: &VisualQuality) {
    draw_cluster(draw, cluster, quality);
}

/// Dibuja el cluster de partículas reactivas
fn draw_cluster(draw: &Draw, cluster: &ClusterData, quality: &VisualQuality) {
    let alpha = map_range(cluster.audio_level, 0.0, 0.3, 0.0, 0.8);
    let cluster_hue = map_range(cluster.frequency, 200.0, 800.0, 0.0, 0.8);
    
    let num_particles = match quality {
        VisualQuality::Low => 20,
        VisualQuality::Medium => 40,
        VisualQuality::High => 60,
        VisualQuality::Ultra => 100,
    };
    
    // Partículas orbitales
    for i in 0..num_particles {
        let angle = (i as f32 / num_particles as f32) * TAU;
        let radius_base = cluster.size * 2.0;
        let radius_variation = random_range(0.5, 1.0);
        
        let orbit_radius = radius_base * radius_variation;
        let x = orbit_radius * angle.cos();
        let y = orbit_radius * angle.sin();
        
        let hue_variation = cluster_hue + random_range(-0.1, 0.1);
        let particle_size = map_range(cluster.amplitude, 50.0, 800.0, 2.0, 8.0);
        let particle_alpha = alpha * random_range(0.5, 1.0);
        
        draw.ellipse()
            .xy(pt2(x, y))
            .radius(particle_size)
            .color(hsva(hue_variation, 0.8, 0.9, particle_alpha));
    }
    
    // Círculo central del cluster
    draw.ellipse()
        .xy(pt2(0.0, 0.0))
        .radius(cluster.size)
        .color(hsva(cluster_hue, 0.6, 0.7, alpha * 0.5));
    
    // Borde del cluster
    draw.ellipse()
        .xy(pt2(0.0, 0.0))
        .radius(cluster.size * 1.5)
        .stroke(hsva(cluster_hue, 0.9, 1.0, alpha * 0.3))
        .stroke_weight(2.0)
        .no_fill();
}

/// Dibuja un evento de drone individual
fn draw_drone(draw: &Draw, drone: &DroneEvent, quality: &VisualQuality) {
    let alpha = map_range(drone.time_alive, 0.0, 3.0, 1.0, 0.2).max(0.2);
    
    let num_rings = match quality {
        VisualQuality::Low => 2,
        VisualQuality::Medium => 4,
        VisualQuality::High => 6,
        VisualQuality::Ultra => 10,
    };
    
    // Círculos concéntricos expansivos
    for i in 1..=num_rings {
        let ring_radius = drone.radius * (1.0 + drone.time_alive * 0.3) * i as f32 * 0.3;
        let ring_alpha = alpha / (i as f32 * 0.5 + 1.0);
        
        draw.ellipse()
            .xy(drone.position)
            .radius(ring_radius)
            .stroke_weight(2.0)
            .stroke(hsva(
                drone.color.hue.to_positive_radians() / TAU,
                drone.color.saturation,
                drone.color.value,
                ring_alpha
            ))
            .no_fill();
    }
}

/// Dibuja el análisis continuo como círculo de fondo
fn draw_analysis(draw: &Draw, analysis: &AnalysisData) {
    let circle_radius = map_range(analysis.amplitude, 0.0, 0.3, 0.0, 350.0);
    let circle_hue = map_range(analysis.brightness, 200.0, 1500.0, 0.0, 0.5);
    let circle_alpha = map_range(analysis.amplitude, 0.0, 0.3, 0.0, 0.3);
    let circle_color = hsva(circle_hue, 0.8, 0.9, circle_alpha);
    
    draw.ellipse()
        .xy(pt2(0.0, 0.0))
        .radius(circle_radius)
        .color(circle_color);
}

/// Dibuja una nota musical individual
fn draw_note(draw: &Draw, note: &Note, win: Rect) {
    let mapping = AudioVisualMapping::default();
    let fade = mapping.time_to_fade(note.time_alive, note.duration);
    
    // Longitud de línea basada en duración
    let line_length_factor = map_range(note.duration, 0.1, 2.0, 0.2, 0.8);
    let line_length = 
        map_range(note.duration, 0.1, 2.0, 30.0, win.w() * line_length_factor);
    
    // Línea principal de la nota
    draw.line()
        .start(note.position)
        .end(pt2(note.position.x + line_length, note.position.y))
        .weight(note.size)
        .color(srgba(
            note.color.red,
            note.color.green,
            note.color.blue,
            (255.0 * fade) as u8
        ));
    
    // Círculo inicial de la nota
    if note.time_alive < 0.1 {
        draw.ellipse()
            .xy(note.position)
            .radius(note.size * 2.0)
            .color(srgba(
                note.color.red,
                note.color.green,
                note.color.blue,
                (200.0 * fade) as u8
            ));
    }
    
    // Efecto de expansión final
    if note.time_alive > note.duration * 0.7 {
        let expansion_factor = map_range(note.time_alive, note.duration * 0.7, note.duration, 1.0, 3.0);
        draw.ellipse()
            .xy(pt2(note.position.x + line_length, note.position.y))
            .radius(note.size * expansion_factor)
            .stroke_weight(1.0)
            .stroke(srgba(
                note.color.red,
                note.color.green,
                note.color.blue,
                (100.0 * fade) as u8
            ))
            .no_fill();
    }
}

/// Dibuja información de debug
fn draw_debug_info(draw: &Draw, model: &Model, win: Rect) {
    let debug_y_start = win.top() - 30.0;
    let line_height = 20.0;
    
    // Información general
    let info_text = format!("Notas: {} | Drones: {} | Cluster: {:.1}Hz", 
        model.notes.len(),
        model.drone_events.len(),
        model.cluster_data.frequency
    );
    
    draw.text(&info_text)
        .xy(pt2(win.left() + 20.0, debug_y_start))
        .font_size(14)
        .color(WHITE);
    
    // Análisis actual
    if model.analysis.amplitude > 0.01 {
        let analysis_text = format!("Análisis - Amp: {:.3} | Bright: {:.1} | Noise: {:.3}",
            model.analysis.amplitude,
            model.analysis.brightness,
            model.analysis.noisiness
        );
        
        draw.text(&analysis_text)
            .xy(pt2(win.left() + 20.0, debug_y_start - line_height))
            .font_size(12)
            .color(YELLOW);
    }
    
    // Estado del cluster
    if model.cluster_data.audio_level > 0.01 {
        let cluster_text = format!("Cluster - Freq: {:.1}Hz | Amp: {:.1} | Level: {:.3}",
            model.cluster_data.frequency,
            model.cluster_data.amplitude,
            model.cluster_data.audio_level
        );
        
        draw.text(&cluster_text)
            .xy(pt2(win.left() + 20.0, debug_y_start - line_height * 2.0))
            .font_size(12)
            .color(LIGHTBLUE);
    }
}

/// Renderiza una VisualNote avanzada
pub fn draw_visual_note(draw: &Draw, note: &VisualNote, win: Rect) {
    let base_position = note.position;
    let effective_size = note.size * note.scale_pulse;
    let effective_opacity = note.opacity;
    
    // Aplicar transformaciones
    let drawing = draw
        .translate(vec3(base_position.x, base_position.y, 0.0))
        .rotate(note.rotation)
        .scale(note.scale_pulse);
    
    // Renderizar según estilo visual
    match &note.visual_style {
        VisualStyle::Circle => {
            draw_circle_note(&drawing, note, effective_size, effective_opacity);
        }
        VisualStyle::Ring => {
            draw_ring_note(&drawing, note, effective_size, effective_opacity);
        }
        VisualStyle::Polygon(sides) => {
            draw_polygon_note(&drawing, note, *sides, effective_size, effective_opacity);
        }
        VisualStyle::Star(points) => {
            draw_star_note(&drawing, note, *points, effective_size, effective_opacity);
        }
        VisualStyle::Wave => {
            draw_wave_note(&drawing, note, effective_size, effective_opacity);
        }
        VisualStyle::Particle => {
            draw_particle_note(&drawing, note, effective_size, effective_opacity);
        }
        VisualStyle::Beam => {
            draw_beam_note(&drawing, note, effective_size, effective_opacity);
        }
        VisualStyle::Blob => {
            draw_blob_note(&drawing, note, effective_size, effective_opacity);
        }
        VisualStyle::Geometric => {
            draw_geometric_note(&drawing, note, effective_size, effective_opacity);
        }
        VisualStyle::Custom(style_name) => {
            draw_custom_note(&drawing, note, style_name, effective_size, effective_opacity);
        }
    }
}

/// Renderiza todas las notas visuales del modelo
pub fn draw_all_visual_notes(draw: &Draw, model: &Model, win: Rect) {
    let sorted_notes = model.get_visual_notes_sorted();
    
    for note in sorted_notes {
        draw_visual_note(draw, note, win);
    }
}

/// Renderiza un círculo para notas simples
fn draw_circle_note(draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
    let color = rgba(
        note.color.red as f32 / 255.0,
        note.color.green as f32 / 255.0,
        note.color.blue as f32 / 255.0,
        opacity,
    );
    
    match note.texture_style {
        TextureStyle::Solid => {
            draw.ellipse()
                .w_h(size, size)
                .color(color);
        }
        TextureStyle::Gradient => {
            // Simular gradiente con círculos concéntricos
            let steps = 5;
            for i in 0..steps {
                let step_size = size * (1.0 - i as f32 / steps as f32);
                let step_alpha = opacity * (1.0 - i as f32 / steps as f32);
                let step_color = rgba(
                    note.color.red as f32 / 255.0,
                    note.color.green as f32 / 255.0,
                    note.color.blue as f32 / 255.0,
                    step_alpha,
                );
                draw.ellipse()
                    .w_h(step_size, step_size)
                    .color(step_color);
            }
        }
        TextureStyle::Glow => {
            // Efecto de brillo con múltiples círculos
            for i in 0..3 {
                let glow_size = size * (1.0 + i as f32 * 0.5);
                let glow_alpha = opacity * (0.3 / (i + 1) as f32);
                let glow_color = rgba(
                    note.color.red as f32 / 255.0,
                    note.color.green as f32 / 255.0,
                    note.color.blue as f32 / 255.0,
                    glow_alpha,
                );
                draw.ellipse()
                    .w_h(glow_size, glow_size)
                    .color(glow_color);
            }
        }
        _ => {
            // Renderizado por defecto
            draw.ellipse()
                .w_h(size, size)
                .color(color);
        }
    }
}

/// Renderiza un anillo para armónicos
fn draw_ring_note(draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
    let color = rgba(
        note.color.red as f32 / 255.0,
        note.color.green as f32 / 255.0,
        note.color.blue as f32 / 255.0,
        opacity,
    );
    
    let stroke_width = (size * 0.1).max(1.0);
    
    draw.ellipse()
        .w_h(size, size)
        .stroke(color)
        .stroke_weight(stroke_width)
        .no_fill();
}

/// Renderiza un polígono
fn draw_polygon_note(draw: &Draw, note: &VisualNote, sides: u8, size: f32, opacity: f32) {
    let color = rgba(
        note.color.red as f32 / 255.0,
        note.color.green as f32 / 255.0,
        note.color.blue as f32 / 255.0,
        opacity,
    );
    
    let points: Vec<Point2> = (0..sides)
        .map(|i| {
            let angle = (i as f32 / sides as f32) * 2.0 * PI;
            let radius = size * 0.5;
            pt2(angle.cos() * radius, angle.sin() * radius)
        })
        .collect();
    
    draw.polygon()
        .points(points)
        .color(color);
}

/// Renderiza una estrella
fn draw_star_note(draw: &Draw, note: &VisualNote, points: u8, size: f32, opacity: f32) {
    let color = rgba(
        note.color.red as f32 / 255.0,
        note.color.green as f32 / 255.0,
        note.color.blue as f32 / 255.0,
        opacity,
    );
    
    let outer_radius = size * 0.5;
    let inner_radius = outer_radius * 0.4;
    
    let vertices: Vec<Point2> = (0..points * 2)
        .map(|i| {
            let angle = (i as f32 / (points * 2) as f32) * 2.0 * PI;
            let radius = if i % 2 == 0 { outer_radius } else { inner_radius };
            pt2(angle.cos() * radius, angle.sin() * radius)
        })
        .collect();
    
    draw.polygon()
        .points(vertices)
        .color(color);
}

/// Renderiza una onda
fn draw_wave_note(draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
    let color = rgba(
        note.color.red as f32 / 255.0,
        note.color.green as f32 / 255.0,
        note.color.blue as f32 / 255.0,
        opacity,
    );
    
    let wave_length = size;
    let wave_height = size * 0.3;
    let points = 20;
    
    // Dibujar onda usando líneas conectadas
    for i in 0..points - 1 {
        let x1 = (i as f32 / points as f32) * wave_length - wave_length * 0.5;
        let y1 = (x1 / wave_length * 4.0 * PI + note.rotation).sin() * wave_height;
        
        let x2 = ((i + 1) as f32 / points as f32) * wave_length - wave_length * 0.5;
        let y2 = (x2 / wave_length * 4.0 * PI + note.rotation).sin() * wave_height;
        
        draw.line()
            .start(pt2(x1, y1))
            .end(pt2(x2, y2))
            .color(color)
            .weight(2.0);
    }
}

/// Renderiza partículas para percusión
fn draw_particle_note(draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
    let color = rgba(
        note.color.red as f32 / 255.0,
        note.color.green as f32 / 255.0,
        note.color.blue as f32 / 255.0,
        opacity,
    );
    
    let particle_count = (note.amplitude * 10.0) as i32 + 5;
    let spread = size;
    
    for i in 0..particle_count {
        let angle = (i as f32 / particle_count as f32) * 2.0 * PI;
        let distance = random_range(0.0, spread * 0.5);
        let particle_size = random_range(1.0, 3.0);
        
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;
        
        draw.ellipse()
            .x_y(x, y)
            .w_h(particle_size, particle_size)
            .color(color);
    }
}

/// Renderiza un rayo
fn draw_beam_note(draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
    let color = rgba(
        note.color.red as f32 / 255.0,
        note.color.green as f32 / 255.0,
        note.color.blue as f32 / 255.0,
        opacity,
    );
    
    let beam_length = size;
    let beam_width = size * 0.1;
    
    draw.rect()
        .w_h(beam_length, beam_width)
        .color(color);
}

/// Renderiza una forma orgánica (blob)
fn draw_blob_note(draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
    let color = rgba(
        note.color.red as f32 / 255.0,
        note.color.green as f32 / 255.0,
        note.color.blue as f32 / 255.0,
        opacity,
    );
    
    // Crear forma orgánica con variaciones en el radio
    let points = 12;
    let base_radius = size * 0.5;
    
    let blob_points: Vec<Point2> = (0..points)
        .map(|i| {
            let angle = (i as f32 / points as f32) * 2.0 * PI;
            let radius_variation = (angle * 3.0 + note.rotation).sin() * 0.2 + 1.0;
            let radius = base_radius * radius_variation;
            pt2(angle.cos() * radius, angle.sin() * radius)
        })
        .collect();
    
    draw.polygon()
        .points(blob_points)
        .color(color);
}

/// Renderiza forma geométrica compleja
fn draw_geometric_note(draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
    let color = rgba(
        note.color.red as f32 / 255.0,
        note.color.green as f32 / 255.0,
        note.color.blue as f32 / 255.0,
        opacity,
    );
    
    // Combinar múltiples formas geométricas
    let base_size = size * 0.6;
    
    // Triángulo base
    let triangle_points = vec![
        pt2(0.0, base_size * 0.5),
        pt2(-base_size * 0.5, -base_size * 0.5),
        pt2(base_size * 0.5, -base_size * 0.5),
    ];
    
    draw.polygon()
        .points(triangle_points)
        .color(color);
    
    // Círculo central
    draw.ellipse()
        .w_h(base_size * 0.3, base_size * 0.3)
        .color(color);
}

/// Renderiza estilo personalizado
fn draw_custom_note(draw: &Draw, note: &VisualNote, style_name: &str, size: f32, opacity: f32) {
    // Implementar estilos personalizados según el nombre
    match style_name.to_lowercase().as_str() {
        "sparkle" => {
            // Crear efecto de chispas
            let sparkle_count = 8;
            let color = rgba(
                note.color.red as f32 / 255.0,
                note.color.green as f32 / 255.0,
                note.color.blue as f32 / 255.0,
                opacity,
            );
            
            for i in 0..sparkle_count {
                let angle = (i as f32 / sparkle_count as f32) * 2.0 * PI;
                let distance = size * 0.3;
                let sparkle_size = random_range(1.0, 4.0);
                
                let x = angle.cos() * distance;
                let y = angle.sin() * distance;
                
                draw.ellipse()
                    .x_y(x, y)
                    .w_h(sparkle_size, sparkle_size)
                    .color(color);
            }
        }
        _ => {
            // Fallback a círculo
            draw_circle_note(draw, note, size, opacity);
        }
    }
}

/// **Renderizado profesional usando ShaderManager**
/// 
/// Esta función integra todos los mapeos profesionales de conversión:
/// - freq_to_x_scroll(): posición X con scroll temporal
/// - amp_to_opacity(): opacidad perceptual logarítmica  
/// - freq_to_color(): color basado en paletas artísticas
/// - kind_to_shape(): forma según tipo de evento musical
pub fn draw_with_professional_mapping(
    draw: &Draw, 
    model: &Model, 
    win: Rect,
    shader_manager: &mut ShaderManager
) {
    // Actualizar tiempo y tamaño de ventana en el shader manager
    shader_manager.update_time(model.elapsed_time);
    shader_manager.update_window_size(vec2(win.w(), win.h()));
    
    // Renderizar notas visuales usando el sistema profesional
    let visual_notes = model.get_visual_notes_sorted();
    
    for note in visual_notes {
        // Usar el shader manager para renderizado optimizado
        shader_manager.render_visual_note(draw, note);
    }
    
    // Estadísticas de rendimiento si están habilitadas
    if model.display_config.show_debug {
        draw_shader_stats(draw, shader_manager, win);
    }
}

/// **Función auxiliar para mostrar estadísticas del ShaderManager**
fn draw_shader_stats(draw: &Draw, shader_manager: &ShaderManager, win: Rect) {
    let stats = shader_manager.get_stats();
    
    let info_text = format!(
        "Shader Stats:\nCache: {} items\nQuality: {:?}\nTime: {:.2}s",
        stats.cache_size,
        stats.quality,
        stats.current_time
    );
    
    draw.text(&info_text)
        .xy(vec2(win.left() + 20.0, win.top() - 60.0))
        .font_size(14)
        .color(WHITE)
        .left_justify();
}

/// **Funciones auxiliares para renderizado con filtrado de visibilidad (modo scroll)**

/// Renderiza solo las notas visuales que están dentro del área visible
fn draw_visible_visual_notes(draw: &Draw, model: &Model, win: Rect, left: f32, right: f32) {
    let sorted_notes = model.get_visual_notes_sorted();
    
    for note in sorted_notes {
        if note.position.x >= left && note.position.x <= right {
            draw_visual_note(draw, note, win);
        }
    }
}

/// Renderiza solo las notas legacy que están dentro del área visible
fn draw_visible_notes_only(draw: &Draw, notes: &[Note], win: Rect, left: f32, right: f32) {
    for note in notes {
        if note.position.x >= left && note.position.x <= right {
            draw_note(draw, note, win);
        }
    }
}

/// Renderiza solo las notas legacy que están dentro del área visible (sin filtro de display mode)
fn draw_visible_notes(draw: &Draw, notes: &[Note], win: Rect, left: f32, right: f32) {
    for note in notes {
        if note.position.x >= left && note.position.x <= right {
            draw_note(draw, note, win);
        }
    }
}

/// Renderiza solo los drones que están dentro del área visible
fn draw_visible_drones_only(draw: &Draw, drones: &[DroneEvent], quality: &VisualQuality, left: f32, right: f32) {
    for drone in drones {
        if drone.position.x >= left && drone.position.x <= right {
            draw_drone(draw, drone, quality);
        }
    }
}

/// Renderiza solo los drones que están dentro del área visible (sin filtro de display mode)
fn draw_visible_drones(draw: &Draw, drones: &[DroneEvent], quality: &VisualQuality, left: f32, right: f32) {
    for drone in drones {
        if drone.position.x >= left && drone.position.x <= right {
            draw_drone(draw, drone, quality);
        }
    }
}

/// **Dibuja indicador de scroll y posición temporal**
fn draw_scroll_indicator(draw: &Draw, model: &Model, win: Rect) {
    let indicator_y = win.bottom() + 10.0;
    let indicator_width = win.w() * 0.8;
    let indicator_height = 4.0;
    let indicator_x = 0.0; // Centrado
    
    // Barra de fondo del indicador
    draw.rect()
        .xy(vec2(indicator_x, indicator_y))
        .w_h(indicator_width, indicator_height)
        .color(rgba(0.3, 0.3, 0.3, 0.5));
    
    // Calcular posición relativa basada en tiempo elapsed y velocidad de scroll
    let time_range = 60.0; // Mostrar 60 segundos de timeline
    let progress = (model.elapsed_time % time_range) / time_range;
    let indicator_pos_x = indicator_x - (indicator_width * 0.5) + (indicator_width * progress);
    
    // Indicador de posición actual
    draw.ellipse()
        .xy(vec2(indicator_pos_x, indicator_y))
        .w_h(8.0, 8.0)
        .color(rgba(1.0, 0.5, 0.0, 0.8)); // Color naranja
    
    // Texto informativo
    let scroll_info = format!(
        "Scroll: {} | Speed: {:.0} px/s | Time: {:.1}s | Presiona M para cambiar modo",
        match model.scroll_mode {
            ScrollMode::Fixed => "FIXED",
            ScrollMode::Scrolling => "SCROLLING",
        },
        model.scroll_speed,
        model.elapsed_time
    );
    
    draw.text(&scroll_info)
        .xy(vec2(0.0, indicator_y - 20.0))
        .font_size(12)
        .color(WHITE)
        .center_justify();
}

/// **Dibuja indicador del modo visual actual en la esquina superior**
fn draw_visual_mode_indicator(draw: &Draw, model: &Model, win: Rect) {
    let mode_text = match model.scroll_mode {
        ScrollMode::Fixed => "MODO: FIJO",
        ScrollMode::Scrolling => "MODO: SCROLL",
    };
    
    // Color del texto según el modo
    let text_color = match model.scroll_mode {
        ScrollMode::Fixed => rgba(0.7, 0.9, 1.0, 0.9),  // Azul claro para modo fijo
        ScrollMode::Scrolling => rgba(1.0, 0.8, 0.2, 0.9), // Amarillo/naranja para scroll
    };
    
    // Posición en esquina superior izquierda
    let text_x = win.left() + 20.0;
    let text_y = win.top() - 30.0;
    
    // Fondo semitransparente para mejor legibilidad
    let bg_padding = 8.0;
    let text_width = mode_text.len() as f32 * 8.0; // Aproximación del ancho del texto
    let text_height = 16.0;
    
    draw.rect()
        .xy(vec2(text_x + text_width * 0.5, text_y))
        .w_h(text_width + bg_padding * 2.0, text_height + bg_padding)
        .color(rgba(0.0, 0.0, 0.0, 0.6)); // Fondo negro semitransparente
    
    // Texto del modo
    draw.text(mode_text)
        .xy(vec2(text_x, text_y))
        .font_size(14)
        .color(text_color)
        .left_justify();
    
    // Información adicional si está en modo scroll
    if model.scroll_mode == ScrollMode::Scrolling {
        let speed_text = format!("Velocidad: {:.0} px/s", model.scroll_speed);
        let speed_y = text_y - 20.0;
        
        // Fondo para texto de velocidad
        let speed_width = speed_text.len() as f32 * 7.0;
        draw.rect()
            .xy(vec2(text_x + speed_width * 0.5, speed_y))
            .w_h(speed_width + bg_padding * 2.0, text_height)
            .color(rgba(0.0, 0.0, 0.0, 0.5));
        
        draw.text(&speed_text)
            .xy(vec2(text_x, speed_y))
            .font_size(12)
            .color(rgba(1.0, 1.0, 1.0, 0.8))
            .left_justify();
    }
}
