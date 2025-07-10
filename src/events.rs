// =================================================================
// 🎵 SC SCORE VISUALIZER - EVENTOS
// =================================================================
// Estructuras de datos para los eventos musicales

use nannou::prelude::*;
use std::time::Instant;

// =================================================================
// TIPOS DE EVENTOS MUSICALES
// =================================================================

#[derive(Debug, Clone)]
pub enum EventType {
    Point,
    Glissando,
    Texture,
    Rhythm,
    Harmony,
}

// =================================================================
// ESTRUCTURA PRINCIPAL DE EVENTO
// =================================================================

#[derive(Debug, Clone)]
pub struct MusicalEvent {
    pub event_type: EventType,
    pub position: Vec2,
    pub frequency: f32,
    pub amplitude: f32,
    pub duration: f32,
    pub timbre: f32,
    pub curvature: f32,
    pub cf: f32,
    pub created_at: Instant,
    pub fade_start: Option<Instant>,
    pub velocity: Vec2,
    pub color: Hsv,
    pub size: f32,
    pub trail: Vec<Vec2>,
}

impl MusicalEvent {
    pub fn new(
        event_type: EventType,
        frequency: f32,
        amplitude: f32,
        duration: f32,
        timbre: f32,
        curvature: f32,
        cf: f32,
        window_rect: Rect,
    ) -> Self {
        let position = Self::frequency_to_position(frequency, window_rect);
        let color = Self::frequency_to_color(frequency, timbre);
        let size = amplitude * 20.0 + 5.0;
        
        Self {
            event_type,
            position,
            frequency,
            amplitude,
            duration,
            timbre,
            curvature,
            cf,
            created_at: Instant::now(),
            fade_start: None,
            velocity: vec2(0.0, 0.0),
            color,
            size,
            trail: Vec::new(),
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        // Actualizar posición con velocidad
        self.position += self.velocity * dt;
        
        // Actualizar trail
        self.trail.push(self.position);
        if self.trail.len() > 20 {
            self.trail.remove(0);
        }
        
        // Aplicar curvatura como modificador de velocidad
        if self.curvature != 0.0 {
            let curve_factor = (self.created_at.elapsed().as_secs_f32() * 2.0).sin() * self.curvature;
            self.velocity.x += curve_factor * 10.0;
        }
    }
    
    pub fn is_expired(&self, fade_time: f64) -> bool {
        if let Some(fade_start) = self.fade_start {
            fade_start.elapsed().as_secs_f64() > fade_time
        } else {
            false
        }
    }
    
    pub fn start_fade(&mut self) {
        if self.fade_start.is_none() {
            self.fade_start = Some(Instant::now());
        }
    }
    
    pub fn get_alpha(&self, fade_time: f64) -> f32 {
        if let Some(fade_start) = self.fade_start {
            let elapsed = fade_start.elapsed().as_secs_f64();
            (1.0 - (elapsed / fade_time)).max(0.0) as f32
        } else {
            1.0
        }
    }
    
    fn frequency_to_position(frequency: f32, window_rect: Rect) -> Vec2 {
        let log_freq = frequency.log2();
        let normalized_freq = (log_freq - 5.0) / 6.0; // Mapear ~32Hz a ~2000Hz
        let x = map_range(normalized_freq, 0.0, 1.0, window_rect.left(), window_rect.right());
        let y = random_range(window_rect.bottom() * 0.8, window_rect.top() * 0.8);
        vec2(x.clamp(window_rect.left(), window_rect.right()), y)
    }
    
    fn frequency_to_color(frequency: f32, timbre: f32) -> Hsv {
        let log_freq = frequency.log2();
        let hue = map_range(log_freq, 5.0, 11.0, 0.0, 360.0);
        let saturation = map_range(timbre, 0.0, 1.0, 0.3, 1.0);
        let value = 0.9;
        hsv(hue, saturation, value)
    }
}

// =================================================================
// DATOS DE ANÁLISIS EN TIEMPO REAL
// =================================================================

#[derive(Debug, Clone)]
pub struct RealtimeData {
    pub frequency: f32,
    pub amplitude: f32,
    pub spectral_centroid: f32,
    pub spectral_flatness: f32,
    pub onset_detected: bool,
    pub received_at: Instant,
}

impl RealtimeData {
    pub fn new(frequency: f32, amplitude: f32, centroid: f32, flatness: f32, onset: bool) -> Self {
        Self {
            frequency,
            amplitude,
            spectral_centroid: centroid,
            spectral_flatness: flatness,
            onset_detected: onset,
            received_at: Instant::now(),
        }
    }
    
    pub fn is_recent(&self, max_age_ms: u64) -> bool {
        self.received_at.elapsed().as_millis() < max_age_ms as u128
    }
}
