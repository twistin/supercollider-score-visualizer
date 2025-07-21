// src/visual/visual_note.rs

use nannou::prelude::*;
use std::time::Instant;
use crate::config::{AudioConfig, VisualConfig};

#[derive(Debug, Clone)]
pub struct VisualNote {
    pub frequency: f32,
    pub amplitude: f32,
    pub duration: f32,
    pub start_time: Instant,
    pub end_time: Instant,
    pub is_active: bool,
    pub position: Point2,
    pub size: f32,
    pub color: Rgba,
    pub instrument: String,
}

impl VisualNote {
    pub fn new(
        frequency: f32,
        amplitude: f32,
        duration: f32,
        instrument: String,
        start_time: Instant,
    ) -> Self {
        let end_time = start_time + std::time::Duration::from_secs_f32(duration);
        VisualNote {
            frequency,
            amplitude,
            duration,
            start_time,
            end_time,
            is_active: true,
            position: pt2(0.0, 0.0),
            size: 0.0,
            color: rgba(1.0, 1.0, 1.0, 1.0),
            instrument,
        }
    }

    pub fn update(
        &mut self,
        current_time: Instant,
        window_rect: Rect,
        audio_config: &AudioConfig,
        visual_config: &VisualConfig,
    ) {
        let elapsed_since_start = current_time.duration_since(self.start_time).as_secs_f32();
        let progress_through_note = elapsed_since_start / self.duration;

        // Y: frecuencia en escala logarítmica
        let y = map_range(
            self.frequency.log2(),
            audio_config.freq_min.log2(),
            audio_config.freq_max.log2(),
            window_rect.bottom() + 50.0,
            window_rect.top() - 50.0,
        );

        // X: desplazamiento horizontal basado en el tiempo
        let timeline_start = Instant::now()
            .checked_sub(std::time::Duration::from_secs_f32(visual_config.timeline_duration))
            .unwrap_or(Instant::now());

        let x_start_time = self.start_time.duration_since(timeline_start).as_secs_f32();

        let x = map_range(
            x_start_time - elapsed_since_start,
            0.0,
            visual_config.timeline_duration,
            window_rect.right() - 50.0,
            window_rect.left() + 50.0,
        );

        self.position = pt2(x, y);

        // Tamaño proporcional a la amplitud
        self.size = map_range(self.amplitude, audio_config.amp_min, audio_config.amp_max, 5.0, 50.0);

        // Color RGBA basado en características
        let alpha = (1.0 - progress_through_note).clamp(0.0, 1.0);

        self.color = rgba(
            map_range(self.frequency, audio_config.freq_min, audio_config.freq_max, 0.0, 1.0), // Rojo
            map_range(self.amplitude, audio_config.amp_min, audio_config.amp_max, 0.0, 1.0),     // Verde
            map_range(self.duration, audio_config.dur_min, audio_config.dur_max, 1.0, 0.0),      // Azul
            alpha,
        );

        // Desactiva la nota si terminó
        if current_time > self.end_time {
            self.is_active = false;
        }
    }
}
