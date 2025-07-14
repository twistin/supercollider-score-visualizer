// src/config/config.rs - Configuration structures for SC Score Visualizer

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub osc: OscConfig,
    pub window: WindowConfig,
    pub visual: VisualConfig,
    pub audio: AudioConfig,
    pub performance: PerformanceConfig,
    pub midi: MidiConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscConfig {
    pub listen_host: String,
    pub listen_port: u16,
    pub buffer_size: usize,
    pub timeout_ms: u64,
    pub max_messages_per_frame: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub vsync: bool,
    pub resizable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualConfig {
    pub quality: String,
    pub background_style: String,
    pub show_debug: bool,
    pub show_grid: bool,
    pub fps_target: u32,
    pub time_window: f32,
    pub max_events: usize,
    pub background_color: [u8; 3],
    pub event_fade_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub freq_min: f32,
    pub freq_max: f32,
    pub amp_min: f32,
    pub amp_max: f32,
    pub dur_min: f32,
    pub dur_max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub max_notes: usize,
    pub max_drones: usize,
    pub max_cluster_particles: usize,
    pub cleanup_interval_frames: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiConfig {
    pub enabled: bool,
    pub default_note_duration: f32,
    pub velocity_scaling: f32,
    pub channel_instruments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub show_osc_messages: bool,
    pub show_performance_stats: bool,
    pub stats_interval_frames: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            osc: OscConfig::default(),
            window: WindowConfig::default(),
            visual: VisualConfig::default(),
            audio: AudioConfig::default(),
            performance: PerformanceConfig::default(),
            midi: MidiConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for OscConfig {
    fn default() -> Self {
        Self {
            listen_host: "127.0.0.1".to_string(),
            listen_port: 7777,
            buffer_size: 1024,
            timeout_ms: 10,
            max_messages_per_frame: 50,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            title: "SC Score Visualizer v2.0".to_string(),
            vsync: true,
            resizable: true,
        }
    }
}

impl Default for VisualConfig {
    fn default() -> Self {
        Self {
            quality: "High".to_string(),
            background_style: "Modern".to_string(),
            show_debug: true,
            show_grid: true,
            fps_target: 60,
            time_window: 10.0,
            max_events: 200,
            background_color: [8, 15, 30],
            event_fade_time: 3.0,
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            freq_min: 20.0,
            freq_max: 20000.0,
            amp_min: 0.0,
            amp_max: 1.0,
            dur_min: 0.1,
            dur_max: 10.0,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_notes: 100,
            max_drones: 10,
            max_cluster_particles: 100,
            cleanup_interval_frames: 300,
        }
    }
}

impl Default for MidiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_note_duration: 1.0,
            velocity_scaling: 1.0,
            channel_instruments: vec![
                "piano".to_string(),
                "sine".to_string(),
                "triangle".to_string(),
                "square".to_string(),
                "sawtooth".to_string(),
                "bell".to_string(),
                "pad".to_string(),
                "lead".to_string(),
                "drums".to_string(),
                "sine".to_string(),
                "sine".to_string(),
                "sine".to_string(),
                "sine".to_string(),
                "sine".to_string(),
                "sine".to_string(),
                "sine".to_string(),
            ],
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "Info".to_string(),
            show_osc_messages: true,
            show_performance_stats: true,
            stats_interval_frames: 300,
        }
    }
}

impl AppConfig {
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                match toml::from_str::<AppConfig>(&content) {
                    Ok(config) => {
                        println!("✅ Configuración cargada desde archivo");
                        config
                    }
                    Err(e) => {
                        eprintln!("⚠️ Error parseando configuración: {}, usando defaults", e);
                        Self::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️ Error leyendo archivo de configuración: {}, usando defaults", e);
                Self::default()
            }
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // Validar puerto OSC
        if self.osc.listen_port == 0 {
            return Err("Puerto OSC no puede ser 0".to_string());
        }

        // Validar configuración de ventana
        if self.window.width == 0 || self.window.height == 0 {
            return Err("Dimensiones de ventana no pueden ser 0".to_string());
        }

        // Validar rangos de audio
        if self.audio.freq_min >= self.audio.freq_max {
            return Err("Rango de frecuencias inválido".to_string());
        }

        if self.audio.amp_min >= self.audio.amp_max {
            return Err("Rango de amplitudes inválido".to_string());
        }

        if self.audio.dur_min >= self.audio.dur_max {
            return Err("Rango de duraciones inválido".to_string());
        }

        // Validar configuración de rendimiento
        if self.performance.max_notes == 0 || self.performance.max_drones == 0 {
            return Err("Límites de rendimiento no pueden ser 0".to_string());
        }

        Ok(())
    }
}