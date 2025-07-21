// src/visual/audio_visual_mapping.rs - Reglas profesionales audio → imagen

use nannou::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

/// Estructura avanzada para notas visuales con propiedades expresivas
#[derive(Debug, Clone)]
pub struct VisualNote {
    // Propiedades de audio fundamentales
    pub frequency: f32,          // Frecuencia en Hz
    pub amplitude: f32,          // Amplitud normalizada (0.0-1.0)
    pub duration: f32,           // Duración en segundos
    pub instrument: String,      // Tipo de instrumento
    
    // Propiedades temporales
    pub timestamp: f64,          // Timestamp para scroll temporal
    pub age: f32,               // Edad en segundos desde creación
    pub lifetime_progress: f32,  // Progreso de vida (0.0-1.0)
    pub birth_time: f32,         // Tiempo de creación de la nota
    
    // Propiedades espaciales
    pub position: Vec2,         // Coordenadas x/y calculadas
    pub velocity: Vec2,         // Velocidad de movimiento
    pub target_position: Vec2,  // Posición objetivo para interpolación
    
    // Propiedades visuales
    pub color: Srgb<u8>,        // Color según instrumento/pitch
    pub secondary_color: Srgb<u8>, // Color secundario para gradientes
    pub size: f32,              // Tamaño base
    pub size_variation: f32,    // Variación de tamaño por amplitud
    
    // Propiedades de forma y estilo
    pub visual_style: VisualStyle,  // Estilo según tipo de evento
    pub energy_level: EnergyLevel,  // Nivel de energía visual
    pub texture_style: TextureStyle, // Estilo de textura
    
    // Propiedades de animación
    pub rotation: f32,          // Rotación actual
    pub rotation_speed: f32,    // Velocidad de rotación
    pub scale_pulse: f32,       // Pulsación de escala
    pub opacity: f32,           // Opacidad actual
    
    // Metadatos
    pub note_id: u64,           // ID único de la nota
    pub layer: VisualLayer,     // Capa de renderizado
    pub priority: u8,           // Prioridad de renderizado (0-255)
}

impl VisualNote {
    pub fn is_active(&self) -> bool {
        // Considera la nota activa si su edad es menor que su duración más un pequeño búfer.
        self.age < self.duration + 2.0
    }
}

/// Estilos visuales según tipo de evento musical
#[derive(Debug, Clone, PartialEq)]
pub enum VisualStyle {
    Circle,           // Círculos simples para notas puras
    Ring,             // Anillos para armónicos
    Polygon(u8),      // Polígonos (3-8 lados) para formas complejas
    Star(u8),         // Estrellas para eventos brillantes
    Wave,             // Ondas para sonidos continuos
    Particle,         // Partículas para percusión
    Beam,             // Rayos para eventos punzantes
    Blob,             // Formas orgánicas para pads
    Geometric,        // Formas geométricas complejas
    Custom(String),   // Estilo personalizado por nombre
}

/// Niveles de energía visual
#[derive(Debug, Clone, PartialEq)]
pub enum EnergyLevel {
    Minimal,     // 0.0-0.2 - Visuales sutiles
    Low,         // 0.2-0.4 - Visuales suaves
    Medium,      // 0.4-0.6 - Visuales normales
    High,        // 0.6-0.8 - Visuales intensos
    Extreme,     // 0.8-1.0 - Visuales explosivos
}

/// Estilos de textura
#[derive(Debug, Clone, PartialEq)]
pub enum TextureStyle {
    Solid,           // Color sólido
    Gradient,        // Gradiente radial
    LinearGradient,  // Gradiente lineal
    Noise,           // Textura de ruido
    Sparkle,         // Efecto brillante
    Smooth,          // Suavizado
    Sharp,           // Bordes definidos
    Glow,            // Efecto de brillo
}

/// Capas de renderizado para organizar elementos visuales
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisualLayer {
    Background = 0,   // Elementos de fondo
    Harmony = 1,      // Elementos armónicos
    Melody = 2,       // Melodías principales
    Percussion = 3,   // Elementos percusivos
    Effects = 4,      // Efectos especiales
    UI = 5,          // Elementos de interfaz
}

/// Configuración de mapeo audio-visual
#[derive(Debug, Clone)]
pub struct AudioVisualMappingConfig {
    // Rangos de frecuencia
    pub freq_min: f32,
    pub freq_max: f32,
    
    // Configuración espacial
    pub window_width: f32,
    pub window_height: f32,
    pub scroll_speed: f32,
    
    // Configuración de color
    pub hue_range: (f32, f32),        // Rango de matiz (0.0-360.0)
    pub saturation_base: f32,         // Saturación base
    pub brightness_base: f32,         // Brillo base
    
    // Configuración de tamaño
    pub size_min: f32,
    pub size_max: f32,
    pub size_amp_factor: f32,         // Factor de amplitud para tamaño
    
    // Configuración temporal
    pub fade_time: f32,               // Tiempo de desvanecimiento
    pub scroll_enabled: bool,         // Habilitar scroll temporal
}

impl Default for AudioVisualMappingConfig {
    fn default() -> Self {
        Self {
            freq_min: 20.0,
            freq_max: 20000.0,
            window_width: 1200.0,
            window_height: 800.0,
            scroll_speed: 100.0,
            hue_range: (0.0, 360.0),
            saturation_base: 0.8,
            brightness_base: 0.9,
            size_min: 5.0,
            size_max: 100.0,
            size_amp_factor: 2.0,
            fade_time: 3.0,
            scroll_enabled: true,
        }
    }
}

/// Mapeador principal de audio a visual
pub struct AudioVisualMapper {
    config: AudioVisualMappingConfig,
    note_counter: u64,
    creation_time: SystemTime,
}

impl AudioVisualMapper {
    /// Crea un nuevo mapeador audio-visual
    pub fn new(config: AudioVisualMappingConfig) -> Self {
        Self {
            config,
            note_counter: 0,
            creation_time: SystemTime::now(),
        }
    }
    
    /// Crea un mapeador con configuración por defecto
    pub fn default() -> Self {
        Self::new(AudioVisualMappingConfig::default())
    }
    
    /// Función principal: mapea parámetros de audio a VisualNote
    pub fn map_audio_to_visual(
        &mut self,
        frequency: f32,
        amplitude: f32,
        duration: f32,
        instrument: &str,
        window_rect: Rect,
    ) -> VisualNote {
        // Generar ID único
        self.note_counter += 1;
        let note_id = self.note_counter;
        
        // Calcular timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        
        // Mapear posición basada en frecuencia
        let position = self.map_frequency_to_position(frequency, window_rect);
        
        // Mapear color basado en instrumento y frecuencia
        let (color, secondary_color) = self.map_to_colors(frequency, instrument);
        
        // Mapear tamaño basado en amplitud
        let size = self.map_amplitude_to_size(amplitude);
        
        // Determinar estilo visual según instrumento
        let visual_style = self.map_instrument_to_style(instrument);
        
        // Calcular nivel de energía
        let energy_level = self.calculate_energy_level(amplitude, frequency);
        
        // Determinar estilo de textura
        let texture_style = self.map_energy_to_texture(&energy_level);
        
        // Calcular propiedades de animación
        let rotation_speed = self.calculate_rotation_speed(frequency, amplitude);
        let velocity = self.calculate_initial_velocity(frequency, amplitude);
        
        // Determinar capa de renderizado
        let layer = self.map_instrument_to_layer(instrument);
        
        // Calcular prioridad
        let priority = self.calculate_priority(amplitude, &layer);
        
        VisualNote {
            frequency,
            amplitude,
            duration,
            instrument: instrument.to_string(),
            timestamp,
            age: 0.0,
            lifetime_progress: 0.0,
            birth_time: timestamp as f32,
            position,
            velocity,
            target_position: position, // Inicialmente igual a posición
            color,
            secondary_color,
            size,
            size_variation: amplitude * self.config.size_amp_factor,
            visual_style,
            energy_level,
            texture_style,
            rotation: 0.0,
            rotation_speed,
            scale_pulse: 1.0,
            opacity: 1.0,
            note_id,
            layer,
            priority,
        }
    }
    
    /// Mapea frecuencia a posición espacial
    fn map_frequency_to_position(&self, frequency: f32, window_rect: Rect) -> Vec2 {
        // Mapeo logarítmico de frecuencia a posición Y
        let freq_log = frequency.max(self.config.freq_min).log2();
        let freq_min_log = self.config.freq_min.log2();
        let freq_max_log = self.config.freq_max.log2();
        
        let y_norm = (freq_log - freq_min_log) / (freq_max_log - freq_min_log);
        let y = map_range(y_norm, 0.0, 1.0, window_rect.bottom(), window_rect.top());
        
        // Posición X según scroll o fija
        let x = if self.config.scroll_enabled {
            window_rect.right() // Aparece desde la derecha
        } else {
            window_rect.left() + window_rect.w() * 0.1 // Posición fija
        };
        
        Vec2::new(x, y)
    }
    
    /// Mapea instrumento y frecuencia a colores
    fn map_to_colors(&self, frequency: f32, instrument: &str) -> (Srgb<u8>, Srgb<u8>) {
        // Color base según instrumento
        let base_hue = match instrument.to_lowercase().as_str() {
            "piano" | "keyboard" => 280.0,  // Púrpura
            "sine" | "oscillator" => 240.0, // Azul
            "triangle" => 120.0,            // Verde
            "square" => 0.0,                // Rojo
            "sawtooth" => 60.0,             // Amarillo
            "bell" | "metallic" => 180.0,   // Cian
            "pad" | "strings" => 300.0,     // Magenta
            "lead" | "synth" => 30.0,       // Naranja
            "drums" | "percussion" => 15.0, // Rojo-naranja
            "bass" => 200.0,                // Azul-cian
            _ => 200.0,                     // Azul por defecto
        };
        
        // Modulación de matiz por frecuencia
        let freq_factor = (frequency - self.config.freq_min) / 
                         (self.config.freq_max - self.config.freq_min);
        let hue_variation = freq_factor * 30.0 - 15.0; // ±15 grados
        let final_hue = (base_hue + hue_variation) % 360.0;
        
        // Color principal
        let primary_color = hsb_to_srgb(
            final_hue,
            self.config.saturation_base,
            self.config.brightness_base,
        );
        
        // Color secundario (complementario para gradientes)
        let secondary_hue = (final_hue + 30.0) % 360.0;
        let secondary_color = hsb_to_srgb(
            secondary_hue,
            self.config.saturation_base * 0.7,
            self.config.brightness_base * 0.8,
        );
        
        (primary_color, secondary_color)
    }
    
    /// Mapea amplitud a tamaño
    fn map_amplitude_to_size(&self, amplitude: f32) -> f32 {
        map_range(
            amplitude,
            0.0, 1.0,
            self.config.size_min,
            self.config.size_max
        )
    }
    
    /// Mapea instrumento a estilo visual
    fn map_instrument_to_style(&self, instrument: &str) -> VisualStyle {
        match instrument.to_lowercase().as_str() {
            "piano" | "keyboard" => VisualStyle::Circle,
            "sine" | "oscillator" => VisualStyle::Ring,
            "triangle" => VisualStyle::Polygon(3),
            "square" => VisualStyle::Polygon(4),
            "sawtooth" => VisualStyle::Star(5),
            "bell" | "metallic" => VisualStyle::Star(6),
            "pad" | "strings" => VisualStyle::Blob,
            "lead" | "synth" => VisualStyle::Beam,
            "drums" | "percussion" => VisualStyle::Particle,
            "bass" => VisualStyle::Wave,
            _ => VisualStyle::Circle,
        }
    }
    
    /// Calcula nivel de energía basado en amplitud y frecuencia
    fn calculate_energy_level(&self, amplitude: f32, frequency: f32) -> EnergyLevel {
        // Factor de energía combinando amplitud y presencia de altas frecuencias
        let freq_factor = if frequency > 2000.0 { 1.2 } else { 1.0 };
        let energy = amplitude * freq_factor;
        
        match energy {
            e if e < 0.2 => EnergyLevel::Minimal,
            e if e < 0.4 => EnergyLevel::Low,
            e if e < 0.6 => EnergyLevel::Medium,
            e if e < 0.8 => EnergyLevel::High,
            _ => EnergyLevel::Extreme,
        }
    }
    
    /// Mapea nivel de energía a estilo de textura
    fn map_energy_to_texture(&self, energy: &EnergyLevel) -> TextureStyle {
        match energy {
            EnergyLevel::Minimal => TextureStyle::Smooth,
            EnergyLevel::Low => TextureStyle::Solid,
            EnergyLevel::Medium => TextureStyle::Gradient,
            EnergyLevel::High => TextureStyle::Glow,
            EnergyLevel::Extreme => TextureStyle::Sparkle,
        }
    }
    
    /// Calcula velocidad de rotación
    fn calculate_rotation_speed(&self, frequency: f32, amplitude: f32) -> f32 {
        // Frecuencias altas rotan más rápido
        let freq_factor = (frequency / 1000.0).min(4.0);
        let amp_factor = amplitude;
        freq_factor * amp_factor * 2.0 // Radianes por segundo
    }
    
    /// Calcula velocidad inicial de movimiento
    fn calculate_initial_velocity(&self, frequency: f32, amplitude: f32) -> Vec2 {
        if !self.config.scroll_enabled {
            return Vec2::ZERO;
        }
        
        // Velocidad base de scroll hacia la izquierda
        let base_velocity = -self.config.scroll_speed;
        
        // Modulación por frecuencia (frecuencias altas van más rápido)
        let freq_factor = 1.0 + (frequency / 2000.0).min(1.0);
        
        // Modulación por amplitud
        let amp_factor = 0.5 + amplitude * 0.5;
        
        Vec2::new(base_velocity * freq_factor * amp_factor, 0.0)
    }
    
    /// Mapea instrumento a capa de renderizado
    fn map_instrument_to_layer(&self, instrument: &str) -> VisualLayer {
        match instrument.to_lowercase().as_str() {
            "drums" | "percussion" => VisualLayer::Percussion,
            "piano" | "lead" | "synth" => VisualLayer::Melody,
            "pad" | "strings" | "bass" => VisualLayer::Harmony,
            _ => VisualLayer::Melody,
        }
    }
    
    /// Calcula prioridad de renderizado
    fn calculate_priority(&self, amplitude: f32, layer: &VisualLayer) -> u8 {
        let base_priority = match layer {
            VisualLayer::Background => 50,
            VisualLayer::Harmony => 100,
            VisualLayer::Melody => 150,
            VisualLayer::Percussion => 200,
            VisualLayer::Effects => 220,
            VisualLayer::UI => 250,
        };
        
        // Ajustar por amplitud
        let amp_bonus = (amplitude * 50.0) as u8;
        (base_priority + amp_bonus).min(255)
    }
    
    /// Actualiza una VisualNote existente (para animación)
    pub fn update_visual_note(&self, note: &mut VisualNote, delta_time: f32) {
        // Actualizar edad
        note.age += delta_time;
        note.lifetime_progress = (note.age / note.duration).min(1.0);
        
        // Actualizar posición
        note.position += note.velocity * delta_time;
        
        // Actualizar rotación
        note.rotation += note.rotation_speed * delta_time;
        
        // Actualizar pulsación de escala
        note.scale_pulse = 1.0 + (note.age * 4.0).sin() * 0.1 * note.amplitude;
        
        // Actualizar opacidad (fade out)
        if note.lifetime_progress > 0.7 {
            let fade_progress = (note.lifetime_progress - 0.7) / 0.3;
            note.opacity = 1.0 - fade_progress;
        }
    }
    
    /// Verifica si una nota debe ser eliminada
    pub fn should_remove_note(&self, note: &VisualNote, window_rect: Rect) -> bool {
        // Eliminar si ha expirado
        if note.age >= note.duration {
            return true;
        }
        
        // Eliminar si salió de la pantalla (scroll)
        if self.config.scroll_enabled && note.position.x < window_rect.left() - 100.0 {
            return true;
        }
        
        // Eliminar si la opacidad es muy baja
        if note.opacity < 0.01 {
            return true;
        }
        
        false
    }
}

/// Función auxiliar para conversión HSB a RGB
fn hsb_to_srgb(hue: f32, saturation: f32, brightness: f32) -> Srgb<u8> {
    let h = hue / 60.0;
    let s = saturation;
    let v = brightness;
    
    let c = v * s;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = v - c;
    
    let (r_prime, g_prime, b_prime) = match h as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    
    let r = ((r_prime + m) * 255.0) as u8;
    let g = ((g_prime + m) * 255.0) as u8;
    let b = ((b_prime + m) * 255.0) as u8;
    
    srgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_visual_mapping() {
        let mut mapper = AudioVisualMapper::default();
        let window_rect = Rect::from_w_h(1200.0, 800.0);
        
        let note = mapper.map_audio_to_visual(
            440.0,      // A4
            0.8,        // Amplitud alta
            2.0,        // 2 segundos
            "piano",    // Piano
            window_rect
        );
        
        assert_eq!(note.frequency, 440.0);
        assert_eq!(note.amplitude, 0.8);
        assert_eq!(note.instrument, "piano");
        assert!(matches!(note.visual_style, VisualStyle::Circle));
        assert!(matches!(note.layer, VisualLayer::Melody));
    }
    
    #[test]
    fn test_energy_level_calculation() {
        let mapper = AudioVisualMapper::default();
        
        assert_eq!(mapper.calculate_energy_level(0.1, 440.0), EnergyLevel::Minimal);
        assert_eq!(mapper.calculate_energy_level(0.5, 440.0), EnergyLevel::Medium);
        assert_eq!(mapper.calculate_energy_level(0.9, 440.0), EnergyLevel::Extreme);
    }
}
