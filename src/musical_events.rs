// Parámetros visuales opcionales para enriquecer la representación
#[derive(Debug, Clone)]
pub struct VisualParams {
    pub timbre: f32,        // 0.0 a 1.0 - rugosidad/suavidad
    pub spatial_spread: f32, // dispersión visual
    pub color_hue: f32,     // matiz 0.0-360.0
    pub intensity: f32,     // energía visual
}

impl Default for VisualParams {
    fn default() -> Self {
        Self {
            timbre: 0.5,
            spatial_spread: 1.0,
            color_hue: 200.0, // azul por defecto
            intensity: 0.7,
        }
    }
}

// Enum para representar los diferentes tipos de eventos musicales.
#[derive(Debug, Clone)]
pub enum MusicalEvent {
    // Un evento discreto en el tiempo.
    Point {
        time: f32, // Tiempo de aparición en segundos.
        freq: f32, // Frecuencia en Hz.
        amp: f32,  // Amplitud (0.0 a 1.0).
        duration: f32, // Duración en segundos.
        visual_params: Option<VisualParams>, // Parámetros visuales opcionales
    },
    // Un glissando entre dos frecuencias.
    Glissando {
        time: f32,
        start_freq: f32,
        end_freq: f32,
        amp: f32,
        duration: f32,
        visual_params: Option<VisualParams>,
    },
    // Un cluster de notas.
    Cluster {
        time: f32,
        center_freq: f32,
        bandwidth: f32, // Ancho de banda del cluster en Hz.
        amp: f32,
        duration: f32,
        visual_params: Option<VisualParams>,
    },
    // Ruido en una banda de frecuencia.
    Noise {
        time: f32,
        center_freq: f32,
        bandwidth: f32,
        amp: f32,
        duration: f32,
        visual_params: Option<VisualParams>,
    },
}

impl MusicalEvent {
    // Devuelve el tiempo de inicio del evento.
    pub fn start_time(&self) -> f32 {
        match *self {
            MusicalEvent::Point { time, .. } => time,
            MusicalEvent::Glissando { time, .. } => time,
            MusicalEvent::Cluster { time, .. } => time,
            MusicalEvent::Noise { time, .. } => time,
        }
    }

    // Devuelve la duración del evento.
    pub fn duration(&self) -> f32 {
        match *self {
            MusicalEvent::Point { duration, .. } => duration,
            MusicalEvent::Glissando { duration, .. } => duration,
            MusicalEvent::Cluster { duration, .. } => duration,
            MusicalEvent::Noise { duration, .. } => duration,
        }
    }

    // Devuelve los parámetros visuales (o default si no existen)
    pub fn visual_params(&self) -> VisualParams {
        match self {
            MusicalEvent::Point { visual_params, .. } => visual_params.clone().unwrap_or_default(),
            MusicalEvent::Glissando { visual_params, .. } => visual_params.clone().unwrap_or_default(),
            MusicalEvent::Cluster { visual_params, .. } => visual_params.clone().unwrap_or_default(),
            MusicalEvent::Noise { visual_params, .. } => visual_params.clone().unwrap_or_default(),
        }
    }

    // Calcula el progreso del evento (0.0 = inicio, 1.0 = finalizado)
    pub fn progress(&self, current_time: f32) -> f32 {
        let elapsed = current_time - self.start_time();
        let duration = self.duration();
        if duration <= 0.0 {
            return 1.0;
        }
        (elapsed / duration).clamp(0.0, 1.0)
    }

    // Verifica si el evento está activo en el tiempo actual
    pub fn is_active(&self, current_time: f32) -> bool {
        let start = self.start_time();
        let end = start + self.duration();
        current_time >= start && current_time <= end
    }
}
