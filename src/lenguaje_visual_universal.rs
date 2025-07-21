// 🌐 Lenguaje Visual Generalizado - Implementación en Rust
// Sistema universal de mapeo audiovisual para cualquier tipo de música

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LenguajeVisualUniversal {
    pub tiempo_ritmo: TiempoRitmo,
    pub alturas_melodia: AlturasMelodia,
    pub armonia_capas: ArmoniaCapas,
    pub timbre_espacio: TimbreEspacio,
    pub color_sinestesia: ColorSinestesia,
    pub modos_especializados: ModosEspecializados,
}

// =====================================================================
// 🎵 TIEMPO Y RITMO
// =====================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TiempoRitmo {
    pub tempo_detection: bool,
    pub beat_emphasis: bool,
    pub grid_sync: bool,
    pub rhythmic_patterns: bool,
    pub beat_flash_intensity: f32,
    pub pattern_repetition_threshold: u32,
    pub grid_pulse_color: [u8; 4],
    pub tempo_range: [u32; 2],
}

impl TiempoRitmo {
    pub fn detectar_tempo(&self, onsets: &[f64]) -> Option<f32> {
        if !self.tempo_detection || onsets.len() < 4 {
            return None;
        }
        
        // Análisis de intervalos entre onsets
        let mut intervals: Vec<f64> = Vec::new();
        for i in 1..onsets.len() {
            intervals.push(onsets[i] - onsets[i-1]);
        }
        
        // Buscar patrón repetitivo (tempo)
        // Implementación simplificada - en producción usar FFT de intervalos
        intervals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_interval = intervals[intervals.len() / 2];
        let bpm = 60.0 / median_interval;
        
        if bpm >= self.tempo_range[0] as f64 && bpm <= self.tempo_range[1] as f64 {
            Some(bpm as f32)
        } else {
            None
        }
    }
    
    pub fn calcular_enfasis_beat(&self, tiempo_actual: f64, tempo: Option<f32>) -> f32 {
        if !self.beat_emphasis || tempo.is_none() {
            return 1.0;
        }
        
        let tempo = tempo.unwrap();
        let beat_duration = 60.0 / tempo as f64;
        let beat_phase = (tiempo_actual % beat_duration) / beat_duration;
        
        // Crear pulso visual sincronizado con beat
        let pulse = (beat_phase * 2.0 * std::f64::consts::PI).sin().abs();
        1.0 + (pulse * self.beat_flash_intensity as f64) as f32
    }
}

// =====================================================================
// 🎼 ALTURAS Y MELODÍA
// =====================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlturasMelodia {
    pub frequency_to_height: bool,
    pub melodic_tracing: bool,
    pub pitch_continuity_threshold: f32,
    pub chromatic_mapping: bool,
    pub microtonal_support: bool,
    pub frequency_resolution: String,
    pub glissando_smoothing: f32,
}

impl AlturasMelodia {
    pub fn mapear_frecuencia_a_altura(&self, frecuencia: f32, altura_pantalla: f32) -> f32 {
        if !self.frequency_to_height {
            return altura_pantalla * 0.5; // Centro por defecto
        }
        
        // Mapeo logarítmico de frecuencia a altura (como partitura musical)
        let freq_min = 20.0f32;
        let freq_max = 8000.0f32;
        let freq_log = frecuencia.log2();
        let min_log = freq_min.log2();
        let max_log = freq_max.log2();
        
        let posicion_normalizada = (freq_log - min_log) / (max_log - min_log);
        altura_pantalla * (1.0 - posicion_normalizada.clamp(0.0, 1.0))
    }
    
    pub fn detectar_melodia(&self, notas: &[(f64, f32)]) -> Vec<(f64, f32, bool)> {
        // notas: (tiempo, frecuencia)
        // retorna: (tiempo, frecuencia, es_conexion_melodica)
        
        if !self.melodic_tracing || notas.len() < 2 {
            return notas.iter().map(|(t, f)| (*t, *f, false)).collect();
        }
        
        let mut melodia = Vec::new();
        for i in 0..notas.len() {
            let (tiempo, freq) = notas[i];
            let es_conexion = if i > 0 {
                let tiempo_anterior = notas[i-1].0;
                tiempo - tiempo_anterior <= self.pitch_continuity_threshold as f64
            } else {
                false
            };
            melodia.push((tiempo, freq, es_conexion));
        }
        
        melodia
    }
    
    pub fn calcular_color_contorno(&self, freq_actual: f32, freq_anterior: Option<f32>) -> [f32; 3] {
        if let Some(freq_ant) = freq_anterior {
            let ratio = freq_actual / freq_ant;
            if ratio > 1.05 { // Ascendente
                [1.0, 0.8, 0.4] // Colores cálidos brillantes
            } else if ratio < 0.95 { // Descendente
                [0.4, 0.6, 1.0] // Colores fríos
            } else { // Estable
                [0.7, 0.7, 0.7] // Gris neutro
            }
        } else {
            [0.7, 0.7, 0.7] // Color neutro para primera nota
        }
    }
}

// =====================================================================
// 🎭 ARMONÍA Y CAPAS
// =====================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArmoniaCapas {
    pub polyphonic_analysis: bool,
    pub layer_separation: String,
    pub max_simultaneous_layers: u32,
    pub layer_blending: String,
    pub harmonic_resonance: bool,
    pub dissonance_emphasis: bool,
    pub chord_recognition: bool,
}

#[derive(Debug, Clone)]
pub struct CapaVisual {
    pub rango_freq: (f32, f32),
    pub color_palette: String,
    pub posicion: String,
    pub forma: String,
    pub eventos_activos: Vec<EventoVisual>,
}

#[derive(Debug, Clone)]
pub struct EventoVisual {
    pub frecuencia: f32,
    pub amplitud: f32,
    pub tiempo_inicio: f64,
    pub duracion: f64,
    pub tipo_forma: String,
    pub color: [f32; 4],
}

impl ArmoniaCapas {
    pub fn separar_en_capas(&self, eventos: &[EventoVisual]) -> Vec<CapaVisual> {
        if !self.polyphonic_analysis {
            // Una sola capa con todos los eventos
            return vec![CapaVisual {
                rango_freq: (20.0, 20000.0),
                color_palette: "universal".to_string(),
                posicion: "full".to_string(),
                forma: "adaptive".to_string(),
                eventos_activos: eventos.to_vec(),
            }];
        }
        
        // Definir capas frecuenciales
        let capas_config = vec![
            ((20.0, 200.0), "earth_tones", "bottom", "waves_thick"),
            ((200.0, 2000.0), "natural_greens", "center", "flowing_lines"),
            ((2000.0, 8000.0), "sky_blues", "top", "sparkles"),
            ((8000.0, 20000.0), "bright_whites", "top_edge", "dots_small"),
        ];
        
        let mut capas = Vec::new();
        
        for (rango, palette, pos, forma) in capas_config {
            let eventos_en_capa: Vec<EventoVisual> = eventos.iter()
                .filter(|e| e.frecuencia >= rango.0 && e.frecuencia < rango.1)
                .cloned()
                .collect();
                
            if !eventos_en_capa.is_empty() {
                capas.push(CapaVisual {
                    rango_freq: rango,
                    color_palette: palette.to_string(),
                    posicion: pos.to_string(),
                    forma: forma.to_string(),
                    eventos_activos: eventos_en_capa,
                });
            }
        }
        
        capas
    }
    
    pub fn detectar_acordes(&self, eventos_simultaneos: &[EventoVisual]) -> Option<String> {
        if !self.chord_recognition || eventos_simultaneos.len() < 3 {
            return None;
        }
        
        // Análisis armónico simplificado
        let mut frecuencias: Vec<f32> = eventos_simultaneos.iter()
            .map(|e| e.frecuencia)
            .collect();
        frecuencias.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Detectar intervalos básicos (implementación simplificada)
        // En producción: análisis de clases de altura, detección de tríadas, etc.
        
        Some("acorde_detectado".to_string()) // Placeholder
    }
}

// =====================================================================
// 🎨 TIMBRE Y ESPACIO SONORO
// =====================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimbreEspacio {
    pub spectral_centroid_mapping: bool,
    pub harmonicity_analysis: bool,
    pub roughness_detection: bool,
    pub brightness_mapping: bool,
    pub frequency_bands: u32,
}

impl TimbreEspacio {
    pub fn analizar_timbre(&self, espectro: &[f32]) -> CaracteristicasTimbricas {
        let mut caracteristicas = CaracteristicasTimbricas::default();
        
        if self.spectral_centroid_mapping {
            caracteristicas.centroide_espectral = self.calcular_centroide_espectral(espectro);
        }
        
        if self.harmonicity_analysis {
            caracteristicas.armonicidad = self.calcular_armonicidad(espectro);
        }
        
        if self.roughness_detection {
            caracteristicas.rugosidad = self.calcular_rugosidad(espectro);
        }
        
        if self.brightness_mapping {
            caracteristicas.brillo = self.calcular_brillo_espectral(espectro);
        }
        
        caracteristicas
    }
    
    fn calcular_centroide_espectral(&self, espectro: &[f32]) -> f32 {
        let mut suma_ponderada = 0.0;
        let mut suma_magnitudes = 0.0;
        let bin_freq = 22050.0 / espectro.len() as f32;
        for (i, magnitud) in espectro.iter().enumerate() {
            let frecuencia = i as f32 * bin_freq;
            suma_ponderada += frecuencia * magnitud;
            suma_magnitudes += magnitud;
        }
        if suma_magnitudes > 0.0 {
            suma_ponderada / suma_magnitudes
        } else {
            0.0
        }
    }
    
    fn calcular_armonicidad(&self, espectro: &[f32]) -> f32 {
        // Análisis de armonicidad vs inharmonicidad
        // Implementación simplificada - en producción usar HNR (Harmonic-to-Noise Ratio)

        let mut picos = Vec::new();
        for i in 1..espectro.len()-1 {
            if espectro[i] > espectro[i-1] && espectro[i] > espectro[i+1] && espectro[i] > 0.1 {
                picos.push((i, espectro[i]));
            }
        }
        let picos = picos.into_iter().take(50).collect::<Vec<_>>();
        if picos.len() < 2 {
            return 0.0;
        }
        // Buscar relaciones armónicas entre picos
        let mut relaciones_armonicas = 0;
        for i in 0..picos.len() {
            for j in i+1..picos.len() {
                let ratio = picos[j].0 as f32 / picos[i].0 as f32;
                if (ratio - ratio.round()).abs() < 0.05 { // Tolerancia para armónicos
                    relaciones_armonicas += 1;
                }
            }
        }
        relaciones_armonicas as f32 / (picos.len() * picos.len()) as f32
    }
    
    fn calcular_rugosidad(&self, espectro: &[f32]) -> f32 {
        // Detectar rugosidad/aspereza del sonido
        // Basado en diferencias rápidas en el espectro
        let rugosidad: f32 = espectro.windows(2).map(|w| (w[1] - w[0]).abs()).sum();
        rugosidad / espectro.len() as f32
    }
    
    fn calcular_brillo_espectral(&self, espectro: &[f32]) -> f32 {
        // Brillo = energía en frecuencias altas vs bajas
        let punto_medio = espectro.len() / 2;
        
        let energia_bajas: f32 = espectro[..punto_medio].iter().sum();
        let energia_altas: f32 = espectro[punto_medio..].iter().sum();
        
        if energia_bajas + energia_altas > 0.0 {
            energia_altas / (energia_bajas + energia_altas)
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaracteristicasTimbricas {
    pub centroide_espectral: f32,
    pub armonicidad: f32,
    pub rugosidad: f32,
    pub brillo: f32,
}

// =====================================================================
// 🌈 COLOR Y SINESTESIA
// =====================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColorSinestesia {
    pub synesthetic_mapping: bool,
    pub color_consistency: bool,
    pub personal_color_profile: String,
    pub frequency_color_mapping: String,
    pub amplitude_color_intensity: bool,
    pub frequency_color_hue: bool,
    pub timbre_color_saturation: bool,
    pub temporal_color_evolution: bool,
}

impl ColorSinestesia {
    pub fn mapear_frecuencia_a_color(&self, frecuencia: f32) -> [f32; 3] {
        if !self.synesthetic_mapping {
            return [0.7, 0.7, 0.7]; // Gris neutro
        }
        
        match self.frequency_color_mapping.as_str() {
            "spectral_rainbow" => self.mapeo_espectral_arcoiris(frecuencia),
            "warm_cool_gradient" => self.mapeo_calido_frio(frecuencia),
            "chromatic_circle" => self.mapeo_circulo_cromatico(frecuencia),
            _ => self.mapeo_espectral_arcoiris(frecuencia),
        }
    }
    
    fn mapeo_espectral_arcoiris(&self, frecuencia: f32) -> [f32; 3] {
        // Mapeo inspirado en el espectro visible de luz
        let freq_normalizada = ((frecuencia.log2() - 20.0f32.log2()) / 
                               (8000.0f32.log2() - 20.0f32.log2())).clamp(0.0, 1.0);
        
        // Mapeo a colores del arcoíris
        let hue = freq_normalizada * 300.0; // 0° (rojo) a 300° (magenta)
        self.hsv_a_rgb(hue, 0.8, 0.9)
    }
    
    fn mapeo_calido_frio(&self, frecuencia: f33) -> [f32; 3] {
        let freq_normalizada = ((frecuencia.log2() - 20.0f32.log2()) / 
                               (8000.0f32.log2() - 20.0f32.log2())).clamp(0.0, 1.0);
        
        // Graves = azules fríos, Agudos = rojos cálidos
        let r = freq_normalizada;
        let g = 0.3 + 0.4 * (1.0 - (freq_normalizada - 0.5).abs() * 2.0);
        let b = 1.0 - freq_normalizada;
        
        [r, g, b]
    }
    
    fn mapeo_circulo_cromatico(&self, frecuencia: f32) -> [f32; 3] {
        // Mapeo basado en el círculo de quintas musical
        let nota = self.frecuencia_a_nota(frecuencia);
        let hue = (nota % 12) as f32 * 30.0; // 12 semitonos = 360°
        self.hsv_a_rgb(hue, 0.7, 0.8)
    }
    
    fn frecuencia_a_nota(&self, frecuencia: f32) -> u32 {
        // Convertir frecuencia a número de semitono (A4 = 440Hz = nota 57)
        let a4 = 440.0;
        let nota_a4 = 57;
        let nota = nota_a4 as f32 + 12.0 * (frecuencia / a4).log2();
        nota.round() as u32
    }
    
    fn hsv_a_rgb(&self, h: f32, s: f32, v: f32) -> [f32; 3] {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        
        let (r_prime, g_prime, b_prime) = match h {
            h if h < 60.0 => (c, x, 0.0),
            h if h < 120.0 => (x, c, 0.0),
            h if h < 180.0 => (0.0, c, x),
            h if h < 240.0 => (0.0, x, c),
            h if h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        
        [r_prime + m, g_prime + m, b_prime + m]
    }
    
    pub fn aplicar_perfil_sinestesico(&self, color_base: [f32; 3], instrumento: &str) -> [f32; 3] {
        if !self.color_consistency {
            return color_base;
        }
        
        // Inspirado en los mapeos sinestésicos de Melissa McCracken
        match instrumento {
            "guitar_electric" => [1.0, 0.84, 0.0], // Dorado
            "piano" => [0.78, 0.78, 1.0],          // Azul marmoleado
            "strings" => [0.59, 1.0, 0.59],        // Verde fluido
            "voice" => [1.0, 0.71, 0.71],          // Rosa cálido
            "percussion" => [1.0, 0.39, 0.20],     // Naranja explosivo
            "synthesizer" => [0.39, 1.0, 1.0],     // Cian geométrico
            _ => color_base, // Color base si no hay mapeo específico
        }
    }
}

// =====================================================================
// 🎛️ MODOS ESPECIALIZADOS
// =====================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModosEspecializados {
    pub espectrograma_artistico: ModoEspectrograma,
    pub partitura: ModoPartitura,
    pub ambient: ModoAmbient,
    pub ritmico: ModoRitmico,
    pub experimental: ModoExperimental,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModoEspectrograma {
    pub enabled: bool,
    pub frequency_resolution: String,
    pub color_mapping: String,
    pub temporal_averaging: f32,
    pub artistic_distortion: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModoPartitura {
    pub enabled: bool,
    pub note_detection: String,
    pub staff_lines: bool,
    pub key_signature_detection: bool,
    pub tempo_grid: bool,
    pub traditional_notation_elements: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModoAmbient {
    pub enabled: bool,
    pub slow_evolution: bool,
    pub atmospheric_effects: bool,
    pub minimal_transients: bool,
    pub color_flow_emphasis: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModoRitmico {
    pub enabled: bool,
    pub beat_detection_enhanced: bool,
    pub pattern_recognition: bool,
    pub rhythmic_geometry: bool,
    pub dance_visualization_elements: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModoExperimental {
    pub enabled: bool,
    pub chaos_mathematics: bool,
    pub fractal_patterns: bool,
    pub non_linear_mappings: bool,
    pub avant_garde_aesthetics: bool,
}

// =====================================================================
// 🔄 SISTEMA DE ADAPTACIÓN AUTOMÁTICA
// =====================================================================

pub struct AdaptadorAutomatico {
    pub modo_actual: String,
    pub confianza_modo: f32,
    pub historial_caracteristicas: Vec<CaracteristicasAudio>,
}

#[derive(Debug, Clone)]
pub struct CaracteristicasAudio {
    pub densidad_eventos: f32,
    pub regularidad_ritmica: f32,
    pub complejidad_armonica: f32,
    pub rango_dinamico: f32,
    pub contenido_frecuencial: f32,
}

impl AdaptadorAutomatico {
    pub fn new() -> Self {
        Self {
            modo_actual: "universal".to_string(),
            confianza_modo: 1.0,
            historial_caracteristicas: Vec::new(),
        }
    }
    
    pub fn analizar_y_adaptar(&mut self, caracteristicas: CaracteristicasAudio) -> String {
        self.historial_caracteristicas.push(caracteristicas.clone());
        
        // Mantener solo últimas N mediciones
        if self.historial_caracteristicas.len() > 100 {
            self.historial_caracteristicas.remove(0);
        }
        
        // Determinar modo óptimo basado en características
        let modo_sugerido = self.determinar_modo_optimo(&caracteristicas);
        
        if modo_sugerido != self.modo_actual {
            self.confianza_modo -= 0.1;
            if self.confianza_modo < 0.3 {
                self.modo_actual = modo_sugerido;
                self.confianza_modo = 0.8;
            }
        } else {
            self.confianza_modo = (self.confianza_modo + 0.05).min(1.0);
        }
        
        self.modo_actual.clone()
    }
    
    fn determinar_modo_optimo(&self, caracteristicas: &CaracteristicasAudio) -> String {
        // Lógica de decisión para modo automático
        
        if caracteristicas.regularidad_ritmica > 0.8 && caracteristicas.densidad_eventos > 0.6 {
            return "ritmico".to_string();
        }
        
        if caracteristicas.complejidad_armonica > 0.7 && caracteristicas.densidad_eventos > 0.5 {
            return "partitura".to_string();
        }
        
        if caracteristicas.densidad_eventos < 0.3 && caracteristicas.rango_dinamico < 0.4 {
            return "ambient".to_string();
        }
        
        if caracteristicas.contenido_frecuencial > 0.8 || caracteristicas.complejidad_armonica > 0.9 {
            return "espectrograma_artistico".to_string();
        }
        
        if caracteristicas.rango_dinamico > 0.8 && caracteristicas.complejidad_armonica > 0.8 {
            return "experimental".to_string();
        }
        
        "universal".to_string()
    }
}

// =====================================================================
// 🔧 IMPLEMENTACIONES DEFAULT
// =====================================================================

impl Default for LenguajeVisualUniversal {
    fn default() -> Self {
        Self {
            tiempo_ritmo: TiempoRitmo {
                tempo_detection: true,
                beat_emphasis: true,
                grid_sync: true,
                rhythmic_patterns: true,
                beat_flash_intensity: 0.8,
                pattern_repetition_threshold: 3,
                grid_pulse_color: [255, 255, 255, 100],
                tempo_range: [60, 200],
            },
            alturas_melodia: AlturasMelodia {
                frequency_to_height: true,
                melodic_tracing: true,
                pitch_continuity_threshold: 0.1,
                chromatic_mapping: true,
                microtonal_support: true,
                frequency_resolution: "high".to_string(),
                glissando_smoothing: 0.3,
            },
            armonia_capas: ArmoniaCapas {
                polyphonic_analysis: true,
                layer_separation: "frequency_bands".to_string(),
                max_simultaneous_layers: 4,
                layer_blending: "additive".to_string(),
                harmonic_resonance: true,
                dissonance_emphasis: true,
                chord_recognition: true,
            },
            timbre_espacio: TimbreEspacio {
                spectral_centroid_mapping: true,
                harmonicity_analysis: true,
                roughness_detection: true,
                brightness_mapping: true,
                frequency_bands: 32,
            },
            color_sinestesia: ColorSinestesia {
                synesthetic_mapping: true,
                color_consistency: true,
                personal_color_profile: "default".to_string(),
                frequency_color_mapping: "spectral_rainbow".to_string(),
                amplitude_color_intensity: true,
                frequency_color_hue: true,
                timbre_color_saturation: true,
                temporal_color_evolution: true,
            },
            modos_especializados: ModosEspecializados {
                espectrograma_artistico: ModoEspectrograma {
                    enabled: true,
                    frequency_resolution: "high".to_string(),
                    color_mapping: "default".to_string(),
                    temporal_averaging: 1.0,
                    artistic_distortion: 0.5,
                },
                partitura: ModoPartitura {
                    enabled: true,
                    note_detection: "onset".to_string(),
                    staff_lines: true,
                    key_signature_detection: true,
                    tempo_grid: true,
                    traditional_notation_elements: true,
                },
                ambient: ModoAmbient {
                    enabled: true,
                    slow_evolution: true,
                    atmospheric_effects: true,
                    minimal_transients: true,
                    color_flow_emphasis: true,
                },
                ritmico: ModoRitmico {
                    enabled: true,
                    beat_detection_enhanced: true,
                    pattern_recognition: true,
                    rhythmic_geometry: true,
                    dance_visualization_elements: true,
                },
                experimental: ModoExperimental {
                    enabled: true,
                    chaos_mathematics: true,
                    fractal_patterns: true,
                    non_linear_mappings: true,
                    avant_garde_aesthetics: true,
                },
            },
        }
    }
}

impl Default for TiempoRitmo {
    fn default() -> Self {
        Self {
            tempo_detection: true,
            beat_emphasis: true,
            grid_sync: true,
            rhythmic_patterns: true,
            beat_flash_intensity: 0.8,
            pattern_repetition_threshold: 3,
            grid_pulse_color: [255, 255, 255, 100],
            tempo_range: [60, 200],
        }
    }
}

impl Default for AlturasMelodia {
    fn default() -> Self {
        Self {
            frequency_to_height: true,
            melodic_tracing: true,
            pitch_continuity_threshold: 0.1,
            chromatic_mapping: true,
            microtonal_support: true,
            frequency_resolution: "high".to_string(),
            glissando_smoothing: 0.3,
        }
    }
}

impl Default for ArmoniaCapas {
    fn default() -> Self {
        Self {
            polyphonic_analysis: true,
            layer_separation: "frequency_bands".to_string(),
            max_simultaneous_layers: 4,
            layer_blending: "additive".to_string(),
            harmonic_resonance: true,
            dissonance_emphasis: true,
            chord_recognition: true,
        }
    }
}

impl Default for TimbreEspacio {
    fn default() -> Self {
        Self {
            spectral_centroid_mapping: true,
            harmonicity_analysis: true,
            roughness_detection: true,
            brightness_mapping: true,
            frequency_bands: 32,
        }
    }
}

impl Default for ColorSinestesia {
    fn default() -> Self {
        Self {
            synesthetic_mapping: true,
            color_consistency: true,
            personal_color_profile: "default".to_string(),
            frequency_color_mapping: "spectral_rainbow".to_string(),
            amplitude_color_intensity: true,
            frequency_color_hue: true,
            timbre_color_saturation: true,
            temporal_color_evolution: true,
        }
    }
}

impl Default for ModosEspecializados {
    fn default() -> Self {
        Self {
            espectrograma_artistico: ModoEspectrograma {
                enabled: true,
                frequency_resolution: "high".to_string(),
                color_mapping: "default".to_string(),
                temporal_averaging: 1.0,
                artistic_distortion: 0.5,
            },
            partitura: ModoPartitura {
                enabled: true,
                note_detection: "onset".to_string(),
                staff_lines: true,
                key_signature_detection: true,
                tempo_grid: true,
                traditional_notation_elements: true,
            },
            ambient: ModoAmbient {
                enabled: true,
                slow_evolution: true,
                atmospheric_effects: true,
                minimal_transients: true,
                color_flow_emphasis: true,
            },
            ritmico: ModoRitmico {
                enabled: true,
                beat_detection_enhanced: true,
                pattern_recognition: true,
                rhythmic_geometry: true,
                dance_visualization_elements: true,
            },
            experimental: ModoExperimental {
                enabled: true,
                chaos_mathematics: true,
                fractal_patterns: true,
                non_linear_mappings: true,
                avant_garde_aesthetics: true,
            },
        }
    }
}
