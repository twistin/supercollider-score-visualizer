// ⏱️ Sistema de sincronización temporal
// Maneja la visualización de eventos en tiempo real según su timestamp y duración

use crate::musical_events::MusicalEvent;
use tracing::{debug, info};

/// Estados de sincronización de un evento
#[derive(Debug, Clone, PartialEq)]
pub enum EventTimeState {
    /// El evento no ha comenzado aún
    NotStarted,
    /// El evento está activo (progreso entre 0.0 y 1.0)
    Active { progress: f32 },
    /// El evento ha terminado
    Finished,
}

/// Calculadora de sincronización temporal
pub struct TimeSync {
    /// Tiempo de inicio del sistema
    pub system_start_time: f32,
    /// Offset para sincronización con SuperCollider
    pub time_offset: f32,
    /// Tolerancia para eventos "casi terminados"
    pub finish_tolerance: f32,
}

impl TimeSync {
    /// Crea un nuevo sistema de sincronización
    pub fn new() -> Self {
        Self {
            system_start_time: 0.0,
            time_offset: 0.0,
            finish_tolerance: 0.1, // 100ms de tolerancia
        }
    }

    /// Establece el tiempo de inicio del sistema
    pub fn set_start_time(&mut self, start_time: f32) {
        self.system_start_time = start_time;
        info!("⏱️ Tiempo de inicio establecido: {:.3}s", start_time);
    }

    /// Establece un offset de tiempo para sincronización
    pub fn set_time_offset(&mut self, offset: f32) {
        self.time_offset = offset;
        debug!("🔄 Offset de tiempo establecido: {:.3}s", offset);
    }

    /// Calcula el tiempo ajustado actual
    pub fn adjusted_time(&self, current_time: f32) -> f32 {
        (current_time - self.system_start_time) + self.time_offset
    }

    /// Determina el estado temporal de un evento
    pub fn get_event_state(&self, event: &MusicalEvent, current_time: f32) -> EventTimeState {
        let adjusted_time = self.adjusted_time(current_time);
        let start_time = event.start_time();
        let end_time = start_time + event.duration();

        if adjusted_time < start_time {
            EventTimeState::NotStarted
        } else if adjusted_time > end_time + self.finish_tolerance {
            EventTimeState::Finished
        } else {
            let progress = event.progress(adjusted_time);
            EventTimeState::Active { progress }
        }
    }

    /// Filtra eventos que deberían ser visibles en el tiempo actual
    pub fn filter_visible_events(&self, events: &[MusicalEvent], current_time: f32) -> Vec<(usize, EventTimeState)> {
        events.iter().enumerate()
            .filter_map(|(index, event)| {
                let state = self.get_event_state(event, current_time);
                match state {
                    EventTimeState::NotStarted => None,
                    _ => Some((index, state))
                }
            })
            .collect()
    }

    /// Calcula el progreso de renderizado para un glissando
    pub fn calculate_glissando_render_progress(&self, event: &MusicalEvent, current_time: f32) -> GlissandoRenderState {
        let state = self.get_event_state(event, current_time);
        
        match event {
            MusicalEvent::Glissando { start_freq, end_freq, .. } => {
                match state {
                    EventTimeState::NotStarted => {
                        GlissandoRenderState::NotVisible
                    },
                    EventTimeState::Active { progress } => {
                        // Calcular frecuencia actual basada en el progreso
                        let current_freq = start_freq + (end_freq - start_freq) * progress;
                        GlissandoRenderState::Partial {
                            progress,
                            current_freq,
                            show_start: true,
                            show_end: progress > 0.9, // Mostrar destino cuando esté cerca del final
                        }
                    },
                    EventTimeState::Finished => {
                        GlissandoRenderState::Complete {
                            start_freq: *start_freq,
                            end_freq: *end_freq,
                        }
                    }
                }
            },
            _ => GlissandoRenderState::NotVisible
        }
    }

    /// Calcula estadísticas de sincronización
    pub fn get_sync_stats(&self, events: &[MusicalEvent], current_time: f32) -> SyncStats {
        let mut stats = SyncStats::default();
        let adjusted_time = self.adjusted_time(current_time);

        for event in events {
            match self.get_event_state(event, current_time) {
                EventTimeState::NotStarted => stats.not_started += 1,
                EventTimeState::Active { .. } => stats.active += 1,
                EventTimeState::Finished => stats.finished += 1,
            }
        }

        stats.current_time = adjusted_time;
        stats.total_events = events.len();
        stats
    }
}

/// Estado de renderizado para glissandos
#[derive(Debug, Clone)]
pub enum GlissandoRenderState {
    /// No visible (no ha comenzado)
    NotVisible,
    /// Renderizado parcial (en progreso)
    Partial {
        progress: f32,
        current_freq: f32,
        show_start: bool,
        show_end: bool,
    },
    /// Renderizado completo (terminado)
    Complete {
        start_freq: f32,
        end_freq: f32,
    },
}

/// Estadísticas de sincronización
#[derive(Debug, Default, Clone)]
pub struct SyncStats {
    pub current_time: f32,
    pub total_events: usize,
    pub not_started: usize,
    pub active: usize,
    pub finished: usize,
}

impl SyncStats {
    /// Retorna el porcentaje de eventos activos
    pub fn active_percentage(&self) -> f32 {
        if self.total_events == 0 {
            0.0
        } else {
            (self.active as f32 / self.total_events as f32) * 100.0
        }
    }

    /// Retorna el porcentaje de eventos completados
    pub fn completion_percentage(&self) -> f32 {
        if self.total_events == 0 {
            0.0
        } else {
            (self.finished as f32 / self.total_events as f32) * 100.0
        }
    }
}
