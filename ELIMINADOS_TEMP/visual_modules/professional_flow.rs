// 🔁 FLUJO PROFESIONAL SUPERCOLLIDER → NANNOU
// Sistema robusto de eventos musicales con buffer temporal y mapeos visuales automáticos

use std::collections::VecDeque;
use std::time::{Instant, Duration};
use nannou::prelude::*;
use tracing::{debug, warn, info};
use anyhow;

use crate::musical_events::{Mu    /// 🎵 RENDERIZADO DE PUNTO
    fn render_point(    /// 🎼 RENDERIZADO DE GLISSANDO
    fn render_glissan    /// 🌟 RENDERIZADO DE CLUSTER
    fn render_clust    /// 🌊 RENDERIZADO DE RUIDO
    fn render_noise(
        &self,
        draw: &Draw,
        win: &Rect,
        center_freq: f32,
        bandwidth: f32,
        amp: f32,
        time: f32,
        duration: f32,
        progress: f32,
        mapping: &VisualMapping,
        visual_params: &Option<VisualParams>,
    ) {
        let default_params = VisualParams::default();
        let params = visual_params.as_ref().unwrap_or(&default_params);lf,
        draw: &Draw,
        win: &Rect,
        center_freq: f32,
        bandwidth: f32,
        amp: f32,
        time: f32,
        duration: f32,
        progress: f32,
        mapping: &VisualMapping,
        visual_params: &Option<VisualParams>,
    ) {
        let default_params = VisualParams::default();
        let params = visual_params.as_ref().unwrap_or(&default_params);lf,
        draw: &Draw,
        win: &Rect,
        start_freq: f32,
        end_freq: f32,
        amp: f32,
        time: f32,
        duration: f32,
        progress: f32,
        mapping: &VisualMapping,
        visual_params: &Option<VisualParams>,
    ) {
        let default_params = VisualParams::default();
        let params = visual_params.as_ref().unwrap_or(&default_params);
        draw: &Draw,
        win: &Rect,
        freq: f32,
        amp: f32,
        time: f32,
        duration: f32,
        progress: f32,
        mapping: &VisualMapping,
        visual_params: &Option<VisualParams>,
    ) {
        let default_params = VisualParams::default();
        let params = visual_params.as_ref().unwrap_or(&default_params);sualParams};
use crate::config::AppConfig;

/// 🎯 BUFFER TEMPORAL DE EVENTOS MUSICALES
/// Estructura central que gestiona eventos musicales con timestamps precisos
#[derive(Debug)]
pub struct MusicalEventBuffer {
    /// Buffer temporal principal (eficiente para insertar/quitar por ambos extremos)
    events: VecDeque<TimestampedEvent>,
    /// Tiempo de inicio de la aplicación
    start_time: Instant,
    /// Configuración global
    config: AppConfig,
    /// Estadísticas de rendimiento
    stats: BufferStats,
}

/// Evento musical con timestamp de alta precisión
#[derive(Debug, Clone)]
struct TimestampedEvent {
    /// Evento musical original
    event: MusicalEvent,
    /// Timestamp real de recepción
    received_at: Instant,
    /// Tiempo absoluto calculado (desde inicio de app)
    absolute_time: f32,
    /// Estado de renderizado actual
    render_state: RenderState,
}

/// Estado actual del renderizado de un evento
#[derive(Debug, Clone)]
enum RenderState {
    /// Evento pendiente (aún no ha comenzado)
    Pending,
    /// Evento activo (renderizándose)
    Active { 
        progress: f32,
        visual_cache: Option<VisualCache> 
    },
    /// Evento completado
    Completed,
    /// Evento expirado (listo para limpieza)
    Expired,
}

/// Cache visual para optimizar renderizado
#[derive(Debug, Clone)]
struct VisualCache {
    /// Posiciones pre-calculadas
    positions: Vec<Point2>,
    /// Colores pre-calculados
    colors: Vec<Srgba<f32>>,
    /// Tamaños pre-calculados
    sizes: Vec<f32>,
    /// Último frame donde se calculó
    last_frame: u64,
}

/// Estadísticas del buffer
#[derive(Debug, Default)]
pub struct BufferStats {
    pub total_received: u64,
    pub active_events: usize,
    pub completed_events: u64,
    pub avg_latency_ms: f32,
    pub peak_concurrent: usize,
}

/// 🎨 SISTEMA DE MAPEOS VISUALES
/// Define cómo se traducen los parámetros musicales a elementos visuales
#[derive(Debug, Clone)]
pub struct VisualMapping {
    /// Configuración de presets visuales
    pub preset: VisualPreset,
    /// Escala de frecuencias para eje Y
    pub freq_range: (f32, f32),
    /// Escala temporal para eje X
    pub time_scale: f32,
    /// Factor de amplitud para tamaños
    pub amp_scale: f32,
    /// Paleta de colores activa
    pub color_palette: ColorPalette,
}

/// Presets visuales predefinidos
#[derive(Debug, Clone)]
pub enum VisualPreset {
    /// Estilo clásico: notación musical tradicional
    Classical {
        staff_lines: bool,
        note_heads: bool,
        stems: bool,
    },
    /// Estilo espectral: enfoque en frecuencias
    Spectral {
        waterfall: bool,
        harmonic_analysis: bool,
        color_by_pitch: bool,
    },
    /// Estilo experimental: formas libres
    Experimental {
        particle_systems: bool,
        organic_shapes: bool,
        procedural_growth: bool,
    },
    /// Estilo minimalista: formas geométricas simples
    Minimal {
        geometric_only: bool,
        monochrome: bool,
        grid_aligned: bool,
    },
    /// Estilo personalizado
    Custom {
        config_path: String,
    },
}

/// Paletas de colores
#[derive(Debug, Clone)]
pub enum ColorPalette {
    /// Monocromático
    Monochrome { base_hue: f32 },
    /// Espectral (frecuencia → color)
    Spectral { saturation: f32, brightness: f32 },
    /// Paleta personalizada
    Custom { colors: Vec<Srgb<u8>> },
    /// Paleta dinámica (cambia con el tiempo)
    Dynamic { speed: f32, range: (f32, f32) },
}

impl MusicalEventBuffer {
    /// Crea un nuevo buffer temporal
    pub fn new(config: AppConfig) -> Self {
        Self {
            events: VecDeque::with_capacity(1000), // Pre-allocar para performance
            start_time: Instant::now(),
            config,
            stats: BufferStats::default(),
        }
    }

    /// 📥 RECEPCIÓN DE EVENTOS
    /// Añade un nuevo evento musical al buffer con timestamp preciso
    pub fn add_event(&mut self, event: MusicalEvent) {
        let now = Instant::now();
        let absolute_time = now.duration_since(self.start_time).as_secs_f32();
        
        let timestamped = TimestampedEvent {
            event,
            received_at: now,
            absolute_time,
            render_state: RenderState::Pending,
        };

        // Insertar en orden temporal (binary search para eficiencia)
        let insert_pos = self.events
            .binary_search_by(|e| e.absolute_time.partial_cmp(&absolute_time).unwrap())
            .unwrap_or_else(|pos| pos);
        
        self.events.insert(insert_pos, timestamped);
        
        // Actualizar estadísticas
        self.stats.total_received += 1;
        self.stats.active_events = self.events.len();
        self.stats.peak_concurrent = self.stats.peak_concurrent.max(self.events.len());
        
        debug!("📥 Evento añadido: tiempo={:.3}s, buffer_size={}", 
               absolute_time, self.events.len());
    }

    /// ⏱️ ACTUALIZACIÓN TEMPORAL
    /// Actualiza el estado de todos los eventos basado en el tiempo actual
    pub fn update(&mut self, app_time: f32) -> anyhow::Result<()> {
        let mut completed_count = 0;
        let mut active_count = 0;

        // Calcular progreso de eventos primero
        let mut event_progress: Vec<f32> = Vec::new();
        for event in &self.events {
            event_progress.push(self.calculate_event_progress(&event.event, app_time));
        }

        // Actualizar estado de cada evento
        for (i, event) in self.events.iter_mut().enumerate() {
            let progress = event_progress[i];
            
            match progress {
                p if p < 0.0 => {
                    event.render_state = RenderState::Pending;
                }
                p if p >= 0.0 && p <= 1.0 => {
                    event.render_state = RenderState::Active { 
                        progress: p, 
                        visual_cache: None // Será calculado en render si es necesario
                    };
                    active_count += 1;
                }
                _ => {
                    if matches!(event.render_state, RenderState::Active { .. }) {
                        completed_count += 1;
                    }
                    event.render_state = RenderState::Completed;
                }
            }
        }

        // Limpiar eventos expirados (más antiguos que tiempo de gracia)
        let grace_period = 2.0; // 2 segundos de gracia
        let cleanup_threshold = app_time - grace_period;
        
        let initial_len = self.events.len();
        self.events.retain(|event| {
            let event_end = event.event.start_time() + event.event.duration();
            event_end > cleanup_threshold
        });
        
        let cleaned = initial_len - self.events.len();
        if cleaned > 0 {
            debug!("🧹 Limpiados {} eventos expirados", cleaned);
        }

        // Actualizar estadísticas
        self.stats.active_events = active_count;
        self.stats.completed_events += completed_count as u64;
        
        Ok(())
    }

    /// 📊 CÁLCULO DE PROGRESO
    /// Calcula el progreso de un evento (considerando timing y duración)
    fn calculate_event_progress(&self, event: &MusicalEvent, current_time: f32) -> f32 {
        let start_time = event.start_time();
        let duration = event.duration();
        
        if current_time < start_time {
            return -1.0; // Evento futuro
        }
        
        if duration <= 0.0 {
            return 1.0; // Evento instantáneo
        }
        
        let elapsed = current_time - start_time;
        elapsed / duration
    }

    /// 🎨 RENDERIZADO PROGRESIVO
    /// Renderiza todos los eventos activos con sus estados actuales
    pub fn render(&mut self, draw: &Draw, win: &Rect, mapping: &VisualMapping, frame_count: u64) {
        let mut rendered_count = 0;

        // Recopilar datos de renderizado primero
        let mut render_data = Vec::new();
        for event in &self.events {
            if let RenderState::Active { progress, visual_cache: _ } = &event.render_state {
                render_data.push((event.event.clone(), *progress));
            }
        }

        // Renderizar usando los datos recopilados
        for (event, progress) in render_data {
            self.render_single_event(draw, win, &event, progress, mapping, frame_count);
            rendered_count += 1;
        }

        if rendered_count > 0 {
            debug!("🎨 Renderizados {} eventos activos", rendered_count);
        }
    }

    /// 🎯 RENDERIZADO INDIVIDUAL
    /// Renderiza un evento específico basado en su tipo y progreso
    fn render_single_event(
        &self, 
        draw: &Draw, 
        win: &Rect, 
        event: &MusicalEvent, 
        progress: f32,
        mapping: &VisualMapping,
        _frame_count: u64
    ) {
        match event {
            MusicalEvent::Point { freq, amp, time, duration, visual_params, .. } => {
                self.render_point(draw, win, *freq, *amp, *time, *duration, progress, mapping, visual_params);
            }
            MusicalEvent::Glissando { start_freq, end_freq, amp, time, duration, visual_params, .. } => {
                self.render_glissando(draw, win, *start_freq, *end_freq, *amp, *time, *duration, progress, mapping, visual_params);
            }
            MusicalEvent::Cluster { center_freq, bandwidth, amp, time, duration, visual_params, .. } => {
                self.render_cluster(draw, win, *center_freq, *bandwidth, *amp, *time, *duration, progress, mapping, visual_params);
            }
            MusicalEvent::Noise { center_freq, bandwidth, amp, time, duration, visual_params, .. } => {
                self.render_noise(draw, win, *center_freq, *bandwidth, *amp, *time, *duration, progress, mapping, visual_params);
            }
        }
    }

    /// 🎵 RENDERIZADO DE PUNTO
    fn render_point(
        &self,
        draw: &Draw,
        win: &Rect,
        freq: f32,
        amp: f32,
        time: f32,
        duration: f32,
        progress: f32,
        mapping: &VisualMapping,
        visual_params: &Option<VisualParams>
    ) {
        let params = visual_params.as_ref().unwrap_or(&VisualParams::default());
        
        // Mapeo de parámetros a coordenadas visuales
        let x = self.map_time_to_x(time, win, mapping) + (progress * mapping.time_scale * duration);
        let y = self.map_freq_to_y(freq, win, mapping);
        let size = self.map_amp_to_size(amp, mapping) * (1.0 + params.intensity * 0.5);
        
        // Envelope visual: fade-in/out basado en progreso
        let envelope = self.calculate_visual_envelope(progress, 0.1, 0.7, 0.2);
        let alpha = envelope * amp;
        
        // Color basado en frecuencia y parámetros visuales
        let color = self.calculate_color(freq, params, mapping, alpha);
        
        // Renderizar con dispersión si está configurada
        if params.spatial_spread > 1.0 {
            self.render_dispersed_point(draw, pt2(x, y), size, color, params.spatial_spread);
        } else {
            draw.ellipse()
                .xy(pt2(x, y))
                .radius(size)
                .color(color);
        }
    }

    /// 🌊 RENDERIZADO DE GLISSANDO
    fn render_glissando(
        &self,
        draw: &Draw,
        win: &Rect,
        start_freq: f32,
        end_freq: f32,
        amp: f32,
        time: f32,
        duration: f32,
        progress: f32,
        mapping: &VisualMapping,
        visual_params: &Option<VisualParams>
    ) {
        let params = visual_params.as_ref().unwrap_or(&VisualParams::default());
        
        // Interpolación de frecuencia
        let current_freq = start_freq + (end_freq - start_freq) * progress;
        
        // Posiciones de inicio y actual
        let start_x = self.map_time_to_x(time, win, mapping);
        let current_x = start_x + (progress * mapping.time_scale * duration);
        let start_y = self.map_freq_to_y(start_freq, win, mapping);
        let current_y = self.map_freq_to_y(current_freq, win, mapping);
        
        let thickness = self.map_amp_to_size(amp, mapping);
        let envelope = self.calculate_visual_envelope(progress, 0.05, 0.9, 0.05);
        let alpha = envelope * amp;
        
        // Dibujar línea progresiva con curvatura si está especificada
        let curvature = params.timbre; // Usar timbre como curvatura
        if curvature > 0.1 {
            self.render_curved_line(draw, start_x, start_y, current_x, current_y, thickness, 
                                  self.calculate_color(current_freq, params, mapping, alpha), curvature);
        } else {
            draw.line()
                .start(pt2(start_x, start_y))
                .end(pt2(current_x, current_y))
                .weight(thickness)
                .color(self.calculate_color(current_freq, params, mapping, alpha));
        }
    }

    /// 🌟 RENDERIZADO DE CLUSTER
    fn render_cluster(
        &self,
        draw: &Draw,
        win: &Rect,
        center_freq: f32,
        bandwidth: f32,
        amp: f32,
        time: f32,
        duration: f32,
        progress: f32,
        mapping: &VisualMapping,
        visual_params: &Option<VisualParams>
    ) {
        let params = visual_params.as_ref().unwrap_or(&VisualParams::default());
        
        let x = self.map_time_to_x(time, win, mapping) + (progress * mapping.time_scale * duration);
        let center_y = self.map_freq_to_y(center_freq, win, mapping);
        let spread = bandwidth * mapping.amp_scale * 0.01; // Convertir Hz a pixels
        
        let envelope = self.calculate_visual_envelope(progress, 0.1, 0.8, 0.1);
        let alpha = envelope * amp;
        
        // Número de puntos basado en bandwidth y intensidad
        let num_points = ((bandwidth * 0.01) * (1.0 + params.intensity * 2.0)).max(3.0) as usize;
        
        for i in 0..num_points {
            let offset_y = (i as f32 / num_points as f32 - 0.5) * spread * 2.0;
            let point_freq = center_freq + (offset_y / mapping.amp_scale * 100.0); // Aproximación
            let point_size = self.map_amp_to_size(amp, mapping) * 
                           (0.5 + 0.5 * (1.0 - (offset_y.abs() / spread).min(1.0))); // Menor en bordes
            
            draw.ellipse()
                .xy(pt2(x, center_y + offset_y))
                .radius(point_size)
                .color(self.calculate_color(point_freq, params, mapping, alpha * 0.7));
        }
    }

    /// 🌪️ RENDERIZADO DE NOISE
    fn render_noise(
        &self,
        draw: &Draw,
        win: &Rect,
        center_freq: f32,
        bandwidth: f32,
        amp: f32,
        time: f32,
        duration: f32,
        progress: f32,
        mapping: &VisualMapping,
        visual_params: &Option<VisualParams>
    ) {
        let params = visual_params.as_ref().unwrap_or(&VisualParams::default());
        
        let x = self.map_time_to_x(time, win, mapping) + (progress * mapping.time_scale * duration);
        let center_y = self.map_freq_to_y(center_freq, win, mapping);
        let spread = bandwidth * mapping.amp_scale * 0.01;
        
        let envelope = self.calculate_visual_envelope(progress, 0.0, 1.0, 0.0); // Sin fade para noise
        let alpha = envelope * amp * 0.3; // Más transparente para noise
        
        // Textura ruidosa con partículas
        let density = (params.intensity * 20.0).max(5.0) as usize;
        
        for _ in 0..density {
            let noise_y = center_y + random_range(-spread, spread);
            let noise_x = x + random_range(-5.0, 5.0); // Pequeña dispersión horizontal
            let noise_size = random_range(0.5, 2.0) * self.map_amp_to_size(amp, mapping) * 0.3;
            
            draw.ellipse()
                .xy(pt2(noise_x, noise_y))
                .radius(noise_size)
                .color(self.calculate_color(center_freq, params, mapping, alpha));
        }
    }

    /// 📏 FUNCIONES DE MAPEO AUXILIARES
    fn map_time_to_x(&self, time: f32, win: &Rect, mapping: &VisualMapping) -> f32 {
        win.left() + (time * mapping.time_scale).min(win.w())
    }

    fn map_freq_to_y(&self, freq: f32, win: &Rect, mapping: &VisualMapping) -> f32 {
        let (min_freq, max_freq) = mapping.freq_range;
        let normalized = ((freq - min_freq) / (max_freq - min_freq)).clamp(0.0, 1.0);
        win.bottom() + normalized * win.h()
    }

    fn map_amp_to_size(&self, amp: f32, mapping: &VisualMapping) -> f32 {
        (amp * mapping.amp_scale).max(1.0)
    }

    /// 📊 CÁLCULOS VISUALES AUXILIARES
    fn calculate_visual_envelope(&self, progress: f32, attack: f32, sustain: f32, release: f32) -> f32 {
        if progress <= attack {
            progress / attack
        } else if progress <= attack + sustain {
            1.0
        } else {
            let release_progress = (progress - attack - sustain) / release;
            (1.0 - release_progress).max(0.0)
        }
    }

    fn calculate_color(&self, freq: f32, params: &VisualParams, mapping: &VisualMapping, alpha: f32) -> Srgba<f32> {
        match &mapping.color_palette {
            ColorPalette::Spectral { saturation, brightness } => {
                let hue = (freq / 1000.0 * 360.0) % 360.0; // 1kHz = 360°
                hsla(hue / 360.0, *saturation, *brightness, alpha).into()
            }
            ColorPalette::Monochrome { base_hue } => {
                let brightness = 0.3 + params.intensity * 0.7;
                hsla(*base_hue / 360.0, 0.2, brightness, alpha).into()
            }
            ColorPalette::Custom { colors } => {
                let idx = ((freq / 100.0) as usize) % colors.len();
                let base = colors[idx];
                srgba(base.red as f32 / 255.0, base.green as f32 / 255.0, base.blue as f32 / 255.0, alpha)
            }
            ColorPalette::Dynamic { speed: _, range: _ } => {
                // Implementar color dinámico basado en tiempo
                let time_hue = (params.color_hue + self.start_time.elapsed().as_secs_f32() * 30.0) % 360.0;
                hsla(time_hue / 360.0, 0.8, 0.6, alpha).into()
            }
        }
    }

    /// 🌟 EFECTOS VISUALES AUXILIARES
    fn render_dispersed_point(&self, draw: &Draw, center: Point2, size: f32, color: Srgba<f32>, spread: f32) {
        let num_particles = (spread * 3.0).min(10.0) as usize;
        for _ in 0..num_particles {
            let offset = vec2(
                random_range(-spread, spread),
                random_range(-spread, spread)
            );
            let particle_size = size * random_range(0.3, 1.0);
            let particle_alpha = color.alpha * random_range(0.3, 1.0);
            
            draw.ellipse()
                .xy(center + offset)
                .radius(particle_size)
                .color(srgba(color.red, color.green, color.blue, particle_alpha));
        }
    }

    fn render_curved_line(&self, draw: &Draw, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Srgba<f32>, curvature: f32) {
        let mid_x = (x1 + x2) * 0.5;
        let mid_y = (y1 + y2) * 0.5 + curvature * 50.0; // Offset curvo
        
        // Simular curva con segmentos
        let segments = 10;
        for i in 0..segments {
            let t1 = i as f32 / segments as f32;
            let t2 = (i + 1) as f32 / segments as f32;
            
            // Interpolación cuadrática simple
            let p1 = self.quadratic_bezier(pt2(x1, y1), pt2(mid_x, mid_y), pt2(x2, y2), t1);
            let p2 = self.quadratic_bezier(pt2(x1, y1), pt2(mid_x, mid_y), pt2(x2, y2), t2);
            
            draw.line()
                .start(p1)
                .end(p2)
                .weight(thickness)
                .color(color);
        }
    }

    fn quadratic_bezier(&self, p0: Point2, p1: Point2, p2: Point2, t: f32) -> Point2 {
        let inv_t = 1.0 - t;
        p0 * inv_t * inv_t + p1 * 2.0 * inv_t * t + p2 * t * t
    }

    /// 🔄 INTEGRACIÓN CON SISTEMA EXISTENTE
    /// Obtiene eventos activos como Vec para compatibilidad
    pub fn get_active_events(&self) -> Vec<MusicalEvent> {
        self.events
            .iter()
            .filter(|e| matches!(e.render_state, RenderState::Active { .. }))
            .map(|e| e.event.clone())
            .collect()
    }

    /// 📊 Obtiene estadísticas del buffer
    pub fn get_stats(&self) -> &BufferStats {
        &self.stats
    }

    /// ⏱️ Actualiza el tiempo interno
    pub fn update_time(&mut self, time: f32) -> anyhow::Result<()> {
        // El tiempo se maneja internamente con Instant, este método mantiene compatibilidad
        self.update(time)
    }
}

impl Default for VisualMapping {
    fn default() -> Self {
        Self {
            preset: VisualPreset::Spectral { 
                waterfall: false, 
                harmonic_analysis: false, 
                color_by_pitch: true 
            },
            freq_range: (20.0, 20000.0), // Rango audible completo
            time_scale: 100.0, // pixels por segundo
            amp_scale: 30.0, // factor de escalado para amplitudes
            color_palette: ColorPalette::Spectral { 
                saturation: 0.8, 
                brightness: 0.7 
            },
        }
    }
}

impl VisualMapping {
    /// Crea un mapeo visual desde configuración
    pub fn from_config(config: &AppConfig) -> Self {
        Self {
            preset: VisualPreset::Classical { 
                staff_lines: config.visual.grid_enabled, 
                note_heads: true, 
                stems: true 
            },
            freq_range: (config.audio.freq_range_min, config.audio.freq_range_max),
            time_scale: 1.0,
            amp_scale: 100.0,
            color_palette: ColorPalette::Spectral { 
                saturation: 0.8, 
                brightness: 0.7 
            },
        }
    }

    /// Crea un mapeo visual desde preset nombrado
    pub fn from_preset(preset_name: &str) -> anyhow::Result<Self> {
        match preset_name {
            "classical" => Ok(Self::classical()),
            "spectral" => Ok(Self::spectral()),
            "minimal" => Ok(Self::minimal()),
            "experimental" => Ok(Self::experimental()),
            _ => Err(anyhow::anyhow!("Preset visual desconocido: {}", preset_name))
        }
    }

    /// Preset experimental para formas libres
    pub fn experimental() -> Self {
        Self {
            preset: VisualPreset::Experimental {
                particle_systems: true,
                organic_shapes: true,
                procedural_growth: true,
            },
            freq_range: (20.0, 20000.0),
            time_scale: 1.5,
            amp_scale: 150.0,
            color_palette: ColorPalette::Dynamic { 
                speed: 0.5, 
                range: (0.0, 360.0) 
            },
        }
    }

    /// Preset minimal para formas geométricas
    pub fn minimal() -> Self {
        Self {
            preset: VisualPreset::Minimal {
                geometric_only: true,
                monochrome: false,
                grid_aligned: true,
            },
            freq_range: (80.0, 2000.0),
            time_scale: 1.0,
            amp_scale: 80.0,
            color_palette: ColorPalette::Monochrome { base_hue: 200.0 },
        }
    }

    /// Preset clásico para notación musical
    pub fn classical() -> Self {
        Self {
            preset: VisualPreset::Classical { 
                staff_lines: true, 
                note_heads: true, 
                stems: true 
            },
            freq_range: (80.0, 2000.0),
            time_scale: 1.0,
            amp_scale: 100.0,
            color_palette: ColorPalette::Spectral { 
                saturation: 0.6, 
                brightness: 0.8 
            },
        }
    }

    /// Preset espectral para análisis de frecuencias
    pub fn spectral() -> Self {
        Self {
            preset: VisualPreset::Spectral { 
                waterfall: false, 
                harmonic_analysis: false, 
                color_by_pitch: true 
            },
            freq_range: (20.0, 20000.0),
            time_scale: 1.0,
            amp_scale: 120.0,
            color_palette: ColorPalette::Spectral { 
                saturation: 0.9, 
                brightness: 0.7 
            },
        }
    }
}
