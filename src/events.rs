// src/events.rs

use std::time::Instant;
use crate::config::AudioConfig;
use nannou::geom::Rect;
use nannou::prelude::*; // Necesario para map_range

/// Evento musical genérico que puede representar distintos tipos de datos musicales
#[derive(Debug, Clone)]
pub enum MusicalEvent {
    Note {
        frequency: f32,
        amplitude: f32,
        duration: f32,
        instrument: String,
        start_time: Instant,
    },
    Drone {
        frequency: f32,
        amplitude: f32,
        instrument: String, // Ahora incluye instrumento
        start_time: Instant,
        duration: f32,
    },
    Realtime(RealtimeData),
    // Los modos AnalysisData y Cluster del DisplayMode anterior
    // se pueden mapear a RealtimeData o a nuevos enums específicos si se reciben como eventos.
    // Por ahora, mantendremos Realtime como el genérico para datos de análisis continuo.
    AnalysisData { // Re-añadido para compatibilidad con OSC directo
        amplitude: f32,
        brightness: f32,
        noisy: f32,
    },
    Cluster { // Re-añadido para compatibilidad con OSC directo
        center_freq: f32,
        freq_width: f32,
        density: f32,
        amplitude: f32,
        duration: f32,
    },
}

/// Datos musicales en tiempo real (por ejemplo, para análisis continuo)
#[derive(Debug, Clone)]
pub struct RealtimeData {
    pub pitch: f32,
    pub amplitude: f32,
    pub centroid: f32,
    pub timestamp: Instant,
}

/// Función auxiliar para mapear una frecuencia (Hz) a la posición vertical en la ventana
pub fn map_freq_to_y(freq: f32, audio_config: &AudioConfig, win: Rect) -> f32 {
    let log_min = audio_config.freq_min.log2();
    let log_max = audio_config.freq_max.log2();
    let log_freq = freq.max(audio_config.freq_min).log2(); // Evitar log(0)

    map_range(log_freq, log_min, log_max, win.bottom() + 50.0, win.top() - 50.0)
}
