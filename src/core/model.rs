// src/core/model.rs - Estructuras de datos principales

use nannou::prelude::*;
use nannou_osc as osc;

/// Evento musical discreto (Pbind, notas individuales)
#[derive(Debug, Clone)]
pub struct MusicalEvent {
    pub event_type: String,
    pub time_alive: f32,
    pub duration: f32,
    pub position: Vec2,
    pub size: f32,
    pub color: Srgb<u8>,
}

/// Datos de análisis continuo (FFT, análisis espectral)
#[derive(Debug, Clone, Default)]
pub struct AnalysisData {
    pub amplitude: f32,
    pub brightness: f32,
    pub noisiness: f32,
}

/// Evento de drone (tonos continuos)
#[derive(Debug, Clone)]
pub struct DroneEvent {
    pub frequency: f32,
    pub amplitude: f32,
    pub time_alive: f32,
    pub position: Vec2,
    pub radius: f32,
    pub color: Hsv,
}

/// Datos del cluster (masa de partículas reactiva)
#[derive(Debug, Clone)]
pub struct ClusterData {
    pub frequency: f32,
    pub amplitude: f32,
    pub audio_level: f32,
    pub size: f32,
    pub density: f32,
    pub time_alive: f32,
}

impl Default for ClusterData {
    fn default() -> Self {
        Self {
            frequency: 300.0,
            amplitude: 200.0,
            audio_level: 0.0,
            size: 50.0,
            density: 0.5,
            time_alive: 0.0,
        }
    }
}

/// Configuración de display/visualización
#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub show_debug: bool,
    pub show_grid: bool,
    pub background_style: BackgroundStyle,
    pub visual_quality: VisualQuality,
}

#[derive(Debug, Clone)]
pub enum BackgroundStyle {
    Modern,
    Simple,
    Gradient,
    None,
}

#[derive(Debug, Clone)]
pub enum VisualQuality {
    Low,
    Medium,
    High,
    Ultra,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_debug: true,
            show_grid: true,
            background_style: BackgroundStyle::Modern,
            visual_quality: VisualQuality::High,
        }
    }
}

/// Modelo principal de la aplicación
pub struct Model {
    pub osc_receiver: osc::Receiver,
    pub events: Vec<MusicalEvent>,
    pub analysis: AnalysisData,
    pub drone_events: Vec<DroneEvent>,
    pub cluster_data: ClusterData,
    pub display_config: DisplayConfig,
}

impl Model {
    /// Crea una nueva instancia del modelo
    pub fn new(receiver: osc::Receiver) -> Self {
        Model {
            osc_receiver: receiver,
            events: Vec::new(),
            analysis: AnalysisData::default(),
            drone_events: Vec::new(),
            cluster_data: ClusterData::default(),
            display_config: DisplayConfig::default(),
        }
    }

    /// Añade un nuevo evento musical discreto
    pub fn add_musical_event(&mut self, freq: f32, amp: f32, dur: f32, win: Rect) {
        use crate::visual::mapping::AudioVisualMapping;

        let mapping = AudioVisualMapping::default();
        let pos_y = mapping.freq_to_y(freq, win);
        let size = mapping.amp_to_size(amp, 2.0, 15.0);
        let color = mapping.freq_to_color(freq);

        let new_event = MusicalEvent {
            event_type: "pbind".to_string(),
            duration: dur,
            time_alive: 0.0,
            position: pt2(win.left() + 50.0, pos_y),
            size,
            color,
        };

        self.events.push(new_event);
    }

    /// Actualiza los datos de análisis continuo
    pub fn update_analysis_data(&mut self, amp: f32, bright: f32, noisy: f32) {
        self.analysis.amplitude = amp;
        self.analysis.brightness = bright;
        self.analysis.noisiness = noisy;
    }

    /// Añade un evento de drone
    pub fn add_drone_event(&mut self, freq: f32, amp: f32) {
        use crate::visual::mapping::AudioVisualMapping;

        let mapping = AudioVisualMapping::default();
        let radius = mapping.amp_to_size(amp, 50.0, 200.0).max(50.0);
        let hue = mapping.freq_to_hue(freq);

        let drone_event = DroneEvent {
            frequency: freq,
            amplitude: amp,
            time_alive: 0.0,
            position: pt2(0.0, 0.0), // Centro de la pantalla
            radius,
            color: hsv(hue, 0.9, 1.0),
        };

        println!(
            "🎵 Drone añadido: {}Hz {}amp radio:{}",
            freq, amp, drone_event.radius
        );

        // Mantener solo los eventos de drone más recientes
        if self.drone_events.len() > 3 {
            self.drone_events.remove(0);
        }
        self.drone_events.push(drone_event);
    }

    /// Actualiza los datos del cluster
    pub fn update_cluster_data(&mut self, freq: f32, amp: f32, level: f32) {
        use crate::visual::mapping::AudioVisualMapping;

        let mapping = AudioVisualMapping::default();

        self.cluster_data.frequency = freq;
        self.cluster_data.amplitude = amp;
        self.cluster_data.audio_level = level;
        self.cluster_data.time_alive = 0.0; // Resetear tiempo para que se vea activo

        // Calcular tamaño basado en frecuencia (inverso: frecuencias altas = cluster pequeño)
        self.cluster_data.size = mapping.freq_to_cluster_size(freq, 30.0, 150.0);

        // Calcular densidad basada en amplitud
        self.cluster_data.density = mapping.amp_to_density(amp);

        println!(
            "🌊 Cluster: freq={:.1}Hz amp={:.1} size={:.1} density={:.2}",
            freq, amp, self.cluster_data.size, self.cluster_data.density
        );
    }

    /// Actualiza el ciclo de vida de todos los eventos
    pub fn update_events(&mut self, dt: f32) {
        // Actualizar eventos musicales discretos
        for event in &mut self.events {
            event.time_alive += dt;
        }
        self.events
            .retain(|event| event.time_alive <= event.duration);

        // Actualizar eventos de drone
        for drone in &mut self.drone_events {
            drone.time_alive += dt;
        }
        // Los drones se mantienen por más tiempo (5 segundos)
        self.drone_events.retain(|drone| drone.time_alive <= 5.0);

        // Actualizar cluster data
        self.cluster_data.time_alive += dt;

        // Si no hay datos nuevos del cluster por más de 1 segundo, reducir la visualización
        if self.cluster_data.time_alive > 1.0 {
            self.cluster_data.audio_level *= 0.95; // Fade out gradual
        }
    }

    /// Configura el modo de visualización
    pub fn set_display_mode(&mut self, show_debug: bool, show_grid: bool) {
        self.display_config.show_debug = show_debug;
        self.display_config.show_grid = show_grid;
    }

    /// Cambia la calidad visual
    pub fn set_visual_quality(&mut self, quality: VisualQuality) {
        self.display_config.visual_quality = quality;
    }
}
