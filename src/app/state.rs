// 🏗️ Estado de la aplicación
// Mantiene el estado global de la aplicación incluyendo eventos musicales y tiempo

use std::sync::{Arc, Mutex};
/// Tipo para compartir una lista de notas entre hilos
pub type SharedNoteList = Arc<Mutex<Vec<Note>>>;
use crate::model::Note; // Usar la estructura Note del módulo model
use crate::errors::{VisualizerError, VisualizerResult, sync_error}; // Importar tipos de error y macro
use anyhow::Result;
use tracing::debug;

/// Estado principal de la aplicación
#[derive(Debug)]
pub struct AppState {
    /// Eventos musicales compartidos entre hilos
    pub events: SharedNoteList,
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

    /// Crea un nuevo AppState con una configuración dada (para consistencia)
    pub fn new_with_config(_config: crate::config::AppConfig) -> Self {
        // La configuración no se usa directamente aquí, pero se mantiene la firma para consistencia.
        Self::new()
    }

    /// Obtiene una copia del Arc para compartir entre hilos
    pub fn get_events_handle(&self) -> SharedNoteList {
        Arc::clone(&self.events)
    }

    /// Actualiza el tiempo de la aplicación
    pub fn update_time(&mut self, time: f32) {
        self.time = time;
    }

    // 🎵 Actualiza y limpia eventos musicales expirados con visibilidad extendida
    pub fn update_events(&mut self) -> Result<()> {
        let mut events_lock = self.events.lock()
            .map_err(|_| sync_error!("Error obteniendo lock de eventos en AppState"))?;
        
        // Eliminar eventos que han terminado - dejar eventos visibles por más tiempo
        let initial_count = events_lock.len();
        // `Note` tiene `time_alive` y `duration`. Los eventos se consideran "terminados"
        // cuando su `time_alive` excede su `duration` más una extensión de visibilidad.
        let visibility_extension = 5.0; // 5 segundos extra de visibilidad
        events_lock.retain(|event| event.time_alive <= event.duration + visibility_extension);
        
        let removed_count = initial_count - events_lock.len();
        
        if removed_count > 0 {
            debug!("🧹 Limpiados {} eventos terminados en AppState", removed_count);
        }
        
        Ok(())
    }

    // 📊 Cuenta los eventos actualmente activos
    pub fn get_active_events_count(&self) -> Result<usize> {
        let events_lock = self.events.lock()
            .map_err(|_| sync_error!("Error obteniendo lock para contar eventos activos en AppState"))?;
        // Un evento `Note` se considera "activo" si su `time_alive` no excede su `duration`.
        Ok(events_lock.iter().filter(|e| e.time_alive <= e.duration).count())
    }

    // 📈 Cuenta todos los eventos en memoria
    pub fn get_total_events_count(&self) -> Result<usize> {
        let events_lock = self.events.lock()
            .map_err(|_| sync_error!("Error obteniendo lock para contar eventos totales en AppState"))?;
        Ok(events_lock.len())
    }

    // 🖼️ Devuelve una copia de los eventos para renderizado visual
    pub fn get_events_for_render(&self) -> Result<Vec<Note>> {
        let events_lock = self.events.lock()
            .map_err(|_| sync_error!("Error obteniendo lock para renderizado en AppState"))?;
        Ok(events_lock.clone())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
