// src/visual/audio_visual_mapping_pro.rs
// Mapas de conversión profesionales para parámetros musicales
// Utiliza escalas perceptuales y paletas artísticas

use nannou::prelude::*;
use std::f32::consts::PI;

/// Configuración para mapeos profesionales
#[derive(Debug, Clone)]
pub struct ProMappingConfig {
    // Configuración temporal
    pub scroll_duration: f32,        // Duración del scroll completo en segundos
    pub time_window: f32,           // Ventana de tiempo visible
    
    // Configuración de frecuencia
    pub freq_min: f32,              // Frecuencia mínima (Hz)
    pub freq_max: f32,              // Frecuencia máxima (Hz)
    pub freq_reference: f32,        // Frecuencia de referencia (A4 = 440Hz)
    
    // Configuración de amplitud
    pub amp_min_db: f32,           // Amplitud mínima en dB
    pub amp_max_db: f32,           // Amplitud máxima en dB
    pub amp_reference_db: f32,     // Amplitud de referencia en dB
    
    // Configuración visual
    pub opacity_min: f32,          // Opacidad mínima
    pub opacity_max: f32,          // Opacidad máxima
    pub window_width: f32,         // Ancho de la ventana
    pub window_height: f32,        // Alto de la ventana
}

impl Default for ProMappingConfig {
    fn default() -> Self {
        Self {
            scroll_duration: 10.0,
            time_window: 5.0,
            freq_min: 20.0,
            freq_max: 20000.0,
            freq_reference: 440.0,
            amp_min_db: -60.0,
            amp_max_db: 0.0,
            amp_reference_db: -20.0,
            opacity_min: 0.1,
            opacity_max: 1.0,
            window_width: 1200.0,
            window_height: 800.0,
        }
    }
}

/// Paletas de colores artísticas para diferentes contextos musicales
#[derive(Debug, Clone)]
pub enum ColorPalette {
    /// Paleta clásica basada en círculo cromático musical
    Classical,
    /// Paleta moderna con colores vibrantes
    Modern,
    /// Paleta warm/cool basada en temperatura de color
    Thermal,
    /// Paleta espectral que sigue el espectro visible
    Spectral,
    /// Paleta calmante con colores suaves
    Ambient,
    /// Paleta energética para música electrónica
    Electronic,
    /// Paleta personalizada
    Custom(Vec<(f32, f32, f32)>), // Vec de (H, S, V)
}

/// Tipos de eventos musicales para mapeo de formas
#[derive(Debug, Clone, PartialEq)]
pub enum EventKind {
    /// Nota musical tradicional
    Note,
    /// Acorde o armonía
    Chord,
    /// Evento percusivo
    Percussion,
    /// Sonido continuo (drone, pad)
    Sustained,
    /// Transitorio o ataque
    Transient,
    /// Ruido o textura
    Noise,
    /// Evento de control (automation)
    Control,
    /// Evento personalizado
    Custom(String),
}

/// Formas visuales disponibles para representar eventos
#[derive(Debug, Clone)]
pub enum VisualShape {
    /// Círculo (notas puras, tonos senoidales)
    Circle { radius: f32 },
    /// Rectángulo (eventos percusivos, ataques)
    Rectangle { width: f32, height: f32 },
    /// Elipse (acordes, armonías)
    Ellipse { width: f32, height: f32 },
    /// Curva Bézier (glissandos, pitch bends)
    Curve { control_points: Vec<Vec2> },
    /// Polígono (timbres complejos)
    Polygon { sides: u8, radius: f32 },
    /// Estrella (eventos brillantes, metálicos)
    Star { points: u8, outer_radius: f32, inner_radius: f32 },
    /// Forma libre (ruido, texturas)
    Blob { points: Vec<Vec2> },
    /// Línea (eventos lineales, sweeps)
    Line { start: Vec2, end: Vec2, thickness: f32 },
}

/// Estructura principal para mapeos profesionales
pub struct ProAudioVisualMapper {
    config: ProMappingConfig,
    color_palette: ColorPalette,
    start_time: f32, // Tiempo de inicio para cálculos relativos
}

impl ProAudioVisualMapper {
    /// Crea un nuevo mapeador profesional
    pub fn new(config: ProMappingConfig, palette: ColorPalette) -> Self {
        Self {
            config,
            color_palette: palette,
            start_time: 0.0, // Se actualizará en el primer uso
        }
    }
    
    /// Crea un mapeador con configuración por defecto
    pub fn default() -> Self {
        Self::new(ProMappingConfig::default(), ColorPalette::Classical)
    }
    
    /// Establece el tiempo de inicio para cálculos relativos
    pub fn set_start_time(&mut self, time: f32) {
        self.start_time = time;
    }
    
    /// **freq_to_x_scroll()**: Asigna posición X relativa al tiempo
    /// 
    /// Esta función mapea la frecuencia a una posición X que se desplaza con el tiempo,
    /// creando un efecto de scroll horizontal donde las notas aparecen desde la derecha
    /// y se mueven hacia la izquierda conforme pasa el tiempo.
    /// 
    /// # Parámetros
    /// - `freq`: Frecuencia en Hz
    /// - `current_time`: Tiempo actual en segundos
    /// - `note_birth_time`: Momento en que nació la nota
    /// 
    /// # Retorna
    /// Posición X en coordenadas de pantalla
    pub fn freq_to_x_scroll(&self, freq: f32, current_time: f32, note_birth_time: f32) -> f32 {
        // Calcular edad de la nota
        let note_age = current_time - note_birth_time;
        
        // Mapeo logarítmico de frecuencia a velocidad de scroll
        // Las frecuencias más altas se mueven más rápido (más energía visual)
        let freq_log = freq.max(self.config.freq_min).log2();
        let freq_min_log = self.config.freq_min.log2();
        let freq_max_log = self.config.freq_max.log2();
        
        // Normalizar a rango [0, 1]
        let freq_normalized = (freq_log - freq_min_log) / (freq_max_log - freq_min_log);
        
        // Factor de velocidad: frecuencias altas = 1.5x velocidad, bajas = 0.7x velocidad
        let speed_factor = 0.7 + (freq_normalized * 0.8);
        
        // Velocidad base del scroll en pixels por segundo
        let base_scroll_speed = self.config.window_width / self.config.scroll_duration;
        let actual_speed = base_scroll_speed * speed_factor;
        
        // Posición inicial (lado derecho de la pantalla)
        let start_x = self.config.window_width * 0.6;
        
        // Calcular desplazamiento actual
        let displacement = actual_speed * note_age;
        
        // Posición final
        start_x - displacement
    }
    
    /// **amp_to_opacity()**: Asigna opacidad según amplitud usando escala logarítmica (dB)
    /// 
    /// Convierte amplitudes lineales a decibelios y mapea a opacidad de manera perceptual.
    /// El oído humano percibe el volumen logarítmicamente, por lo que este mapeo
    /// resulta más natural y expresivo.
    /// 
    /// # Parámetros
    /// - `amplitude`: Amplitud lineal normalizada (0.0 - 1.0)
    /// 
    /// # Retorna
    /// Opacidad en rango [opacity_min, opacity_max]
    pub fn amp_to_opacity(&self, amplitude: f32) -> f32 {
        // Evitar log(0) usando un mínimo muy pequeño
        let amp_safe = amplitude.max(1e-6);
        
        // Convertir a decibelios: dB = 20 * log10(amp)
        let amp_db = 20.0 * amp_safe.log10();
        
        // Mapear dB a rango de opacidad usando interpolación suave
        let db_normalized = (amp_db - self.config.amp_min_db) / 
                          (self.config.amp_max_db - self.config.amp_min_db);
        
        // Aplicar curva de respuesta perceptual (gamma ~2.2)
        let gamma = 2.2;
        let opacity_raw = db_normalized.clamp(0.0, 1.0).powf(1.0 / gamma);
        
        // Mapear al rango de opacidad configurado
        map_range(
            opacity_raw,
            0.0, 1.0,
            self.config.opacity_min,
            self.config.opacity_max
        )
    }
    
    /// **freq_to_color()**: Mapear pitch a color usando paletas artísticas
    /// 
    /// Utiliza el círculo de quintas y escalas musicales para crear mapeos
    /// de color que son tanto matemáticamente coherentes como estéticamente
    /// agradables. Incluye modulación por octavas y temperamento.
    /// 
    /// # Parámetros
    /// - `freq`: Frecuencia en Hz
    /// - `amplitude`: Amplitud para modular saturación/brillo
    /// 
    /// # Retorna
    /// Color en formato Srgb<u8>
    pub fn freq_to_color(&self, freq: f32, amplitude: f32) -> Srgb<u8> {
        // Calcular la nota musical (0-11) usando temperamento igual
        let freq_ratio = freq / self.config.freq_reference; // Relación con A4 (440Hz)
        let semitones_from_a4 = 12.0 * freq_ratio.log2();
        let note_class = ((semitones_from_a4 % 12.0) + 12.0) % 12.0; // 0-11.99
        
        // Calcular octava para modulación de brillo
        let octave = (semitones_from_a4 / 12.0).floor();
        let octave_normalized = ((octave + 5.0) / 10.0).clamp(0.0, 1.0); // Normalizar octavas
        
        // Seleccionar paleta de colores
        let (hue, saturation_base, value_base) = match &self.color_palette {
            ColorPalette::Classical => {
                // Círculo de quintas: C=0°, G=51°, D=102°, A=153°, E=204°, B=255°, F#=306°
                let fifth_cycle = [0.0, 51.4, 102.9, 154.3, 205.7, 257.1, 308.6, 0.0, 51.4, 102.9, 154.3, 205.7];
                let hue = fifth_cycle[note_class as usize % 12];
                (hue, 0.8, 0.9)
            },
            
            ColorPalette::Modern => {
                // Mapeo lineal moderno con colores vibrantes
                let hue = (note_class / 12.0) * 360.0;
                (hue, 0.9, 0.95)
            },
            
            ColorPalette::Thermal => {
                // Mapeo térmico: graves = fríos (azul), agudos = cálidos (rojo)
                let freq_log = freq.max(self.config.freq_min).log2();
                let freq_min_log = self.config.freq_min.log2();
                let freq_max_log = self.config.freq_max.log2();
                let thermal_ratio = (freq_log - freq_min_log) / (freq_max_log - freq_min_log);
                
                // Azul (240°) -> Cian (180°) -> Verde (120°) -> Amarillo (60°) -> Rojo (0°)
                let hue = 240.0 - (thermal_ratio * 240.0);
                (hue, 0.85, 0.9)
            },
            
            ColorPalette::Spectral => {
                // Espectro visible: 380nm (violeta) -> 700nm (rojo)
                // Mapeo logarítmico de frecuencia a longitud de onda
                let freq_log = freq.max(self.config.freq_min).log2();
                let freq_min_log = self.config.freq_min.log2();
                let freq_max_log = self.config.freq_max.log2();
                let spectral_ratio = (freq_log - freq_min_log) / (freq_max_log - freq_min_log);
                
                // Violeta (270°) -> Azul (240°) -> Verde (120°) -> Amarillo (60°) -> Rojo (0°)
                let hue = 270.0 - (spectral_ratio * 270.0);
                (hue, 0.9, 0.95)
            },
            
            ColorPalette::Ambient => {
                // Paleta suave para música ambient
                let hue_base = 200.0; // Azul-cian base
                let hue_variation = (note_class / 12.0) * 60.0 - 30.0; // ±30° de variación
                let hue = (hue_base + hue_variation) % 360.0;
                (hue, 0.6, 0.8)
            },
            
            ColorPalette::Electronic => {
                // Paleta energética para música electrónica
                let electronic_colors = [
                    300.0, 330.0, 0.0, 30.0, 60.0, 90.0, // Magenta -> Amarillo
                    120.0, 150.0, 180.0, 210.0, 240.0, 270.0 // Verde -> Violeta
                ];
                let hue = electronic_colors[note_class as usize % 12];
                (hue, 1.0, 1.0)
            },
            
            ColorPalette::Custom(colors) => {
                if colors.is_empty() {
                    (200.0, 0.8, 0.9) // Fallback
                } else {
                    let color_index = (note_class as usize) % colors.len();
                    colors[color_index]
                }
            }
        };
        
        // Modular saturación por amplitud (más fuerte = más saturado)
        let saturation = saturation_base * (0.3 + amplitude * 0.7);
        
        // Modular brillo por octava y amplitud
        let value = value_base * (0.4 + amplitude * 0.4 + octave_normalized * 0.2);
        
        // Convertir HSV a RGB
        hsv_to_srgb(hue, saturation, value)
    }
    
    /// **kind_to_shape()**: Elegir forma según tipo de evento
    /// 
    /// Mapea el tipo de evento musical a una forma visual apropiada,
    /// considerando las características perceptuales y culturales
    /// asociadas con cada tipo de sonido.
    /// 
    /// # Parámetros
    /// - `kind`: Tipo de evento musical
    /// - `freq`: Frecuencia para modular el tamaño
    /// - `amplitude`: Amplitud para modular la intensidad visual
    /// - `duration`: Duración para formas que cambian con el tiempo
    /// 
    /// # Retorna
    /// Forma visual apropiada para el evento
    pub fn kind_to_shape(&self, kind: &EventKind, freq: f32, amplitude: f32, duration: f32) -> VisualShape {
        // Calcular tamaño base basado en amplitud (escala logarítmica)
        let size_base = self.amp_to_size_scale(amplitude);
        
        // Modular tamaño por frecuencia (graves = más grandes, agudos = más pequeños)
        let freq_factor = self.freq_to_size_factor(freq);
        let adjusted_size = size_base * freq_factor;
        
        match kind {
            EventKind::Note => {
                // Círculos para notas puras - forma más simple y reconocible
                VisualShape::Circle { 
                    radius: adjusted_size 
                }
            },
            
            EventKind::Chord => {
                // Elipses para acordes - sugiere múltiples componentes armónicos
                let aspect_ratio = 1.0 + (duration * 0.2); // Más ancho para acordes largos
                VisualShape::Ellipse { 
                    width: adjusted_size * aspect_ratio, 
                    height: adjusted_size 
                }
            },
            
            EventKind::Percussion => {
                // Rectángulos para percusión - ataques definidos, forma angular
                let impact_factor = amplitude.powf(0.5); // Raíz cuadrada para suavizar
                VisualShape::Rectangle { 
                    width: adjusted_size * (0.8 + impact_factor * 0.4), 
                    height: adjusted_size * (1.2 + impact_factor * 0.3) 
                }
            },
            
            EventKind::Sustained => {
                // Polígonos suaves para sonidos sostenidos (pads, strings)
                let sides = 6 + ((duration * 2.0) as u8).min(6); // 6-12 lados según duración
                VisualShape::Polygon { 
                    sides, 
                    radius: adjusted_size 
                }
            },
            
            EventKind::Transient => {
                // Estrellas para transitorios - eventos brillantes y punzantes
                let points = 4 + ((amplitude * 4.0) as u8).min(4); // 4-8 puntas según amplitud
                VisualShape::Star { 
                    points,
                    outer_radius: adjusted_size,
                    inner_radius: adjusted_size * 0.4
                }
            },
            
            EventKind::Noise => {
                // Formas orgánicas (blobs) para ruido y texturas
                let complexity = 8 + ((amplitude * 8.0) as usize).min(8); // 8-16 puntos
                let points = self.generate_blob_points(adjusted_size, complexity, freq);
                VisualShape::Blob { points }
            },
            
            EventKind::Control => {
                // Líneas para eventos de control (automation, MIDI CC)
                let length = adjusted_size * (1.0 + duration * 0.5);
                let thickness = (amplitude * 4.0).max(1.0);
                VisualShape::Line { 
                    start: Vec2::ZERO, 
                    end: vec2(length, 0.0), 
                    thickness 
                }
            },
            
            EventKind::Custom(name) => {
                // Formas personalizadas según el nombre
                match name.to_lowercase().as_str() {
                    "glissando" | "slide" => {
                        // Curva Bézier para glissandos
                        let control_points = vec![
                            vec2(0.0, 0.0),
                            vec2(adjusted_size * 0.3, adjusted_size * 0.6),
                            vec2(adjusted_size * 0.7, -adjusted_size * 0.3),
                            vec2(adjusted_size, 0.0),
                        ];
                        VisualShape::Curve { control_points }
                    },
                    _ => {
                        // Fallback a círculo
                        VisualShape::Circle { radius: adjusted_size }
                    }
                }
            }
        }
    }
    
    /// Calcula factor de tamaño basado en amplitud (escala logarítmica)
    fn amp_to_size_scale(&self, amplitude: f32) -> f32 {
        // Usar escala logarítmica similar a la percepción de volumen
        let amp_safe = amplitude.max(1e-6);
        let amp_db = 20.0 * amp_safe.log10();
        let size_normalized = (amp_db - self.config.amp_min_db) / 
                            (self.config.amp_max_db - self.config.amp_min_db);
        
        // Mapear a rango de tamaño con mínimo visible
        let size_min = 5.0;
        let size_max = 50.0;
        size_min + (size_normalized.clamp(0.0, 1.0) * (size_max - size_min))
    }
    
    /// Calcula factor de modulación de tamaño por frecuencia
    fn freq_to_size_factor(&self, freq: f32) -> f32 {
        // Graves = más grandes (factor > 1), agudos = más pequeños (factor < 1)
        let freq_log = freq.max(self.config.freq_min).log2();
        let ref_log = self.config.freq_reference.log2();
        let octaves_from_ref = (freq_log - ref_log) / 12.0;
        
        // Factor decrece logarítmicamente con la frecuencia
        (2.0_f32).powf(-octaves_from_ref * 0.5).clamp(0.3, 3.0)
    }
    
    /// Genera puntos para formas orgánicas (blobs)
    fn generate_blob_points(&self, base_radius: f32, point_count: usize, freq: f32) -> Vec<Vec2> {
        let mut points = Vec::with_capacity(point_count);
        
        // Usar frecuencia como semilla para variación consistente
        let freq_seed = (freq * 1000.0) as u32;
        
        for i in 0..point_count {
            let angle = (i as f32 / point_count as f32) * 2.0 * PI;
            
            // Variación pseudoaleatoria basada en frecuencia y posición
            let variation = ((freq_seed.wrapping_add(i as u32) as f32).sin() * 0.5 + 0.5) * 0.4 + 0.8;
            let radius = base_radius * variation;
            
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            points.push(vec2(x, y));
        }
        
        points
    }
    
    /// Cambia la paleta de colores
    pub fn set_color_palette(&mut self, palette: ColorPalette) {
        self.color_palette = palette;
    }
    
    /// Obtiene información sobre la configuración actual
    pub fn get_config(&self) -> &ProMappingConfig {
        &self.config
    }
    
    /// Actualiza la configuración
    pub fn update_config(&mut self, config: ProMappingConfig) {
        self.config = config;
    }
}

/// Función auxiliar para conversión HSV a sRGB
fn hsv_to_srgb(hue: f32, saturation: f32, value: f32) -> Srgb<u8> {
    let h = (hue % 360.0) / 60.0;
    let s = saturation.clamp(0.0, 1.0);
    let v = value.clamp(0.0, 1.0);
    
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
    fn test_freq_to_x_scroll() {
        let mapper = ProAudioVisualMapper::default();
        
        // Test básico
        let x1 = mapper.freq_to_x_scroll(440.0, 1.0, 0.0);
        let x2 = mapper.freq_to_x_scroll(440.0, 2.0, 0.0);
        
        assert!(x2 < x1, "Las notas deben moverse hacia la izquierda con el tiempo");
        
        // Test velocidad por frecuencia
        let x_low = mapper.freq_to_x_scroll(100.0, 1.0, 0.0);
        let x_high = mapper.freq_to_x_scroll(2000.0, 1.0, 0.0);
        
        assert!(x_high < x_low, "Frecuencias altas deben moverse más rápido");
    }
    
    #[test]
    fn test_amp_to_opacity() {
        let mapper = ProAudioVisualMapper::default();
        
        // Test rango
        let opacity_min = mapper.amp_to_opacity(0.001);
        let opacity_max = mapper.amp_to_opacity(1.0);
        
        assert!(opacity_min >= 0.0 && opacity_min <= 1.0);
        assert!(opacity_max >= 0.0 && opacity_max <= 1.0);
        assert!(opacity_max > opacity_min);
        
        // Test monotonía
        let op1 = mapper.amp_to_opacity(0.1);
        let op2 = mapper.amp_to_opacity(0.5);
        let op3 = mapper.amp_to_opacity(0.9);
        
        assert!(op1 < op2 && op2 < op3, "La opacidad debe aumentar con la amplitud");
    }
    
    #[test]
    fn test_freq_to_color_consistency() {
        let mapper = ProAudioVisualMapper::default();
        
        // El mismo pitch debe dar el mismo color
        let color1 = mapper.freq_to_color(440.0, 0.5);
        let color2 = mapper.freq_to_color(440.0, 0.5);
        
        assert_eq!(color1, color2, "La misma frecuencia debe producir el mismo color");
        
        // Octavas deben tener colores relacionados pero distinguibles
        let c4 = mapper.freq_to_color(261.63, 0.5); // C4
        let c5 = mapper.freq_to_color(523.25, 0.5); // C5
        
        // Deben ser diferentes pero relacionados
        assert_ne!(c4, c5, "Octavas deben tener colores distinguibles");
    }
    
    #[test]
    fn test_kind_to_shape_consistency() {
        let mapper = ProAudioVisualMapper::default();
        
        // Diferentes tipos deben producir diferentes formas
        let note_shape = mapper.kind_to_shape(&EventKind::Note, 440.0, 0.5, 1.0);
        let perc_shape = mapper.kind_to_shape(&EventKind::Percussion, 440.0, 0.5, 0.1);
        
        match (&note_shape, &perc_shape) {
            (VisualShape::Circle { .. }, VisualShape::Rectangle { .. }) => {
                // Correcto: Note -> Circle, Percussion -> Rectangle
            },
            _ => panic!("Los tipos de evento deben mapear a formas diferentes"),
        }
    }
}
