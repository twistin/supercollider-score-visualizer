// src/events.rs

use std::time::Instant;
use crate::config::AudioConfig;
use nannou::geom::Rect;
use nannou::prelude::*; // Necesario para map_range
use nannou_osc as osc;

/// Evento musical genérico que puede representar distintos tipos de datos musicales
#[derive(Debug, Clone)]
pub enum MusicalEvent {
    /// Evento de nota musical discreta, con frecuencia, amplitud, duración, instrumento y tiempo de inicio.
    Note {
        frequency: f32,
        amplitude: f32,
        duration: f32,
        instrument: String,
        start_time: Instant,
    },
    /// Evento de nota musical con color personalizado (OSC: /note_colored)
    NoteColored {
        frequency: f32,
        amplitude: f32,
        duration: f32,
        r: f32,
        g: f32,
        b: f32,
        start_time: Instant,
    },
    /// Evento de drone (sonido sostenido), con parámetros similares a Note.
    Drone {
        frequency: f32,
        amplitude: f32,
        instrument: String, // Ahora incluye instrumento
        start_time: Instant,
        duration: f32,
    },
    /// Evento en tiempo real, utilizado para análisis continuo como pitch tracking.
    Realtime(RealtimeData),
    /// Datos de análisis específicos recibidos por OSC. Puede incluir brillo, ruido, etc.
    AnalysisData {
        amplitude: f32,
        brightness: f32,
        noisy: f32,
    },
    /// Representa agrupaciones de eventos o clústeres de frecuencia, usados para visualizaciones densas.
    Cluster {
        center_freq: f32,
        freq_width: f32,
        density: f32,
        amplitude: f32,
        duration: f32,
        start_time: Instant,
    },
}

impl MusicalEvent {
    /// Devuelve el `Instant` asociado al evento si aplica (Note, Drone, Realtime).
    pub fn timestamp(&self) -> Option<Instant> {
        match self {
            MusicalEvent::Note { start_time, .. } => Some(*start_time),
            MusicalEvent::Drone { start_time, .. } => Some(*start_time),
            MusicalEvent::Realtime(data) => Some(data.timestamp),
            _ => None,
        }
    }
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

/// Mensaje OSC procesado, con dirección, argumentos y marca de tiempo.
pub struct ProcessedOscMessage {
    pub addr: String,
    pub args: Vec<osc::Type>,
    pub timestamp: std::time::Instant,
}
