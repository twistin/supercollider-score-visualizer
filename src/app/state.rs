// 🏗️ Estado de la aplicación
// Mantiene el estado global de la aplicación incluyendo eventos musicales y tiempo

use std::sync::{Arc, Mutex};
use crate::musical_events::MusicalEvent;
use crate::utils::VisualizerError;
use anyhow::Result;
use tracing::debug;

/// Estado principal de la aplicación
#[derive(Debug)]
pub struct AppState {
    /// Eventos musicales compartidos entre hilos
    pub events: Arc<Mutex<Vec<MusicalEvent>>>,
    /// Tiempo actual de la aplicación
    pub time: f32,
}

impl AppState {
    /// Crea un nuevo estado de aplicación
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            time: 0.0,
        }
    }

    /// Obtiene una copia del Arc para compartir entre hilos
    pub fn get_events_handle(&self) -> Arc<Mutex<Vec<MusicalEvent>>> {
        Arc::clone(&self.events)
    }

    /// Actualiza el tiempo de la aplicación
    pub fn update_time(&mut self, time: f32) {
        self.time = time;
    }

    /// Actualiza los tiempos de eventos nuevos y elimina eventos terminados
    pub fn update_events(&mut self) -> Result<()> {
        let mut events_lock = self.events.lock()
            .map_err(|_| VisualizerError::sync("Error obteniendo lock de eventos"))?;
        
        // Actualizar el tiempo de eventos que aún no han comenzado
        for event in events_lock.iter_mut() {
            if event.start_time() == 0.0 {
                match event {
                    MusicalEvent::Point { time, .. } => time = self.time,
                    MusicalEvent::Glissando { time, .. } => time = self.time,
                    MusicalEvent::Cluster { time, .. } => time = self.time,
                    MusicalEvent::Noise { time, .. } => time = self.time,
                }
            }
        }

        // Eliminar eventos que han terminado - dejar eventos visibles por más tiempo
        let initial_count = events_lock.len();
        // Añadir 5 segundos extra de visibilidad después de que termine el evento
        let visibility_extension = 5.0;
        events_lock.retain(|event| self.time < event.start_time() + event.duration() + visibility_extension);
        let removed_count = initial_count - events_lock.len();
        
        if removed_count > 0 {
            debug!("🧹 Limpiados {} eventos terminados", removed_count);
        }
        
        Ok(())
    }

    /// Obtiene el número de eventos activos
    pub fn get_active_events_count(&self) -> Result<usize> {
        let events_lock = self.events.lock()
            .map_err(|_| VisualizerError::sync("Error obteniendo lock para contar eventos activos"))?;
        Ok(events_lock.iter().filter(|e| e.is_active(self.time)).count())
    }

    /// Obtiene el número total de eventos
    pub fn get_total_events_count(&self) -> Result<usize> {
        let events_lock = self.events.lock()
            .map_err(|_| VisualizerError::sync("Error obteniendo lock para contar eventos totales"))?;
        Ok(events_lock.len())
    }

    /// Obtiene una copia de todos los eventos para renderizado
    pub fn get_events_for_render(&self) -> Result<Vec<MusicalEvent>> {
        let events_lock = self.events.lock()
            .map_err(|_| VisualizerError::sync("Error obteniendo lock para renderizado"))?;
        Ok(events_lock.clone())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}