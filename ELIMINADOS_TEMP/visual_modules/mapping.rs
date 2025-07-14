// src/visual/mapping.rs - Reglas de mapeo audio → imagen

use nannou::prelude::*;

/// Configuración de mapeo audio-visual
pub struct AudioVisualMapping {
    pub freq_range: (f32, f32),
    pub amp_range: (f32, f32),
    pub color_range: (f32, f32),
}

impl Default for AudioVisualMapping {
    fn default() -> Self {
        Self {
            freq_range: (80.0, 2000.0),
            amp_range: (0.0, 1.0),
            color_range: (0.0, 0.7),
        }
    }
}

impl AudioVisualMapping {
    /// Convierte frecuencia a posición Y en la ventana
    pub fn freq_to_y(&self, freq: f32, win: Rect) -> f32 {
        map_range(
            freq.log2(),
            self.freq_range.0.log2(),
            self.freq_range.1.log2(),
            win.bottom() + 20.0,
            win.top() - 20.0,
        )
    }

    /// Convierte amplitud a tamaño visual
    pub fn amp_to_size(&self, amp: f32, min_size: f32, max_size: f32) -> f32 {
        map_range(amp, self.amp_range.0, self.amp_range.1, min_size, max_size)
    }

    /// Convierte frecuencia a color (matiz HSV)
    pub fn freq_to_hue(&self, freq: f32) -> f32 {
        map_range(
            freq,
            self.freq_range.0,
            self.freq_range.1,
            self.color_range.0,
            self.color_range.1,
        )
    }

    /// Convierte frecuencia a color RGB directo
    pub fn freq_to_color(&self, freq: f32) -> Srgb<u8> {
        let hue = self.freq_to_hue(freq);
        Srgb::from_format(hsv(hue, 0.9, 1.0).into())
    }

    /// Mapeo específico para clusters: frecuencia → tamaño (inverso)
    pub fn freq_to_cluster_size(&self, freq: f32, min_size: f32, max_size: f32) -> f32 {
        // Frecuencias altas → cluster pequeño, frecuencias bajas → cluster grande
        map_range(freq, 200.0, 600.0, max_size, min_size).max(min_size)
    }

    /// Mapeo específico para clusters: amplitud → densidad
    pub fn amp_to_density(&self, amp: f32) -> f32 {
        map_range(amp, 50.0, 800.0, 0.3, 1.0).clamp(0.3, 1.0)
    }

    /// Calcula alpha basado en tiempo de vida y duración
    pub fn time_to_alpha(&self, time_alive: f32, duration: f32) -> f32 {
        map_range(time_alive, 0.0, duration, 1.0, 0.0).max(0.0)
    }
}

/// Reglas específicas para diferentes tipos de eventos
pub struct EventVisualRules;

impl EventVisualRules {
    /// Configuración visual para eventos de Pbind/melodía
    pub fn pbind_config() -> (f32, f32, f32) {
        // (line_length_factor, size_factor, alpha_factor)
        (0.6, 15.0, 1.0)
    }

    /// Configuración visual para eventos de percusión
    pub fn percussion_config() -> (f32, f32, f32) {
        // (radius_factor, size_factor, alpha_factor)
        (1.0, 20.0, 1.2)
    }

    /// Configuración visual para drones
    pub fn drone_config() -> (f32, f32, f32) {
        // (expansion_factor, alpha_decay, duration)
        (0.3, 0.2, 5.0)
    }

    /// Configuración visual para clusters
    pub fn cluster_config() -> (i32, f32, f32) {
        // (max_particles, size_variation, alpha_variation)
        (50, 1.0, 1.0)
    }
}

/// Funciones de utilidad para cálculos visuales comunes
pub mod visual_utils {
    use nannou::prelude::*;

    /// Genera una posición aleatoria dentro de un círculo
    pub fn random_position_in_circle(center: Vec2, radius: f32) -> Vec2 {
        let angle = random_range(0.0, TAU);
        let distance = random_range(0.0, radius);
        center + vec2(distance * angle.cos(), distance * angle.sin())
    }

    /// Calcula el número de partículas basado en densidad
    pub fn particles_from_density(density: f32, max_particles: i32) -> i32 {
        (density * max_particles as f32) as i32
    }

    /// Genera variación de color para partículas
    pub fn hue_variation(base_hue: f32, variation: f32) -> f32 {
        base_hue + random_range(-variation, variation)
    }

    /// Calcula transparencia con variación aleatoria
    pub fn alpha_with_variation(base_alpha: f32, variation: f32) -> f32 {
        (base_alpha * random_range(1.0 - variation, 1.0 + variation)).clamp(0.0, 1.0)
    }
}
