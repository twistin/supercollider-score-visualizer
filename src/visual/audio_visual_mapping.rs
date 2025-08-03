// src/visual/audio_visual_mapping.rs

use nannou::prelude::*;
use crate::config::AirportVisualConfig;
use crate::visual::VisualNote;

/// Mapeador avanzado para tu música generativa
#[derive(Debug)]
pub struct AirportVisualMapper {
    pub airport_config: AirportVisualConfig,
    note_counter: u64,
    last_plane_change: f64,
    current_plane: usize,
}

impl AirportVisualMapper {
    pub fn new(airport_config: AirportVisualConfig) -> Self {
        Self {
            airport_config,
            note_counter: 0,
            last_plane_change: 0.0,
            current_plane: 0,
        }
    }

    /// Mapeo especializado para tu synth de "Music for Airports"
    pub fn map_airport_sound(
        &mut self,
        freq: f32,
        amp: f32,
        dur: f32,
        note_type: &str,  // "airport" o "plane"
        seq_pos: usize,   // Posición en la secuencia
        window_rect: Rect,
        time: f64,
    ) -> VisualNote {
        self.note_counter += 1;
        // Determinar estilo basado en el tipo de sonido
        // Usar solo el constructor real de VisualNote
        // NOTA: Ajusta los argumentos según la definición real de VisualNote::new
        // Ejemplo: VisualNote::new(frequency, amplitude, duration, instrument, start_time)
        VisualNote::new(
            freq,
            amp,
            dur,
            note_type.to_string(),
            // Si VisualNote requiere un tiempo de inicio, usa time o Instant::now()
            std::time::Instant::now(),
        )
    }

    // ...
    // Si necesitas mapear otros elementos, agrégalos aquí
}