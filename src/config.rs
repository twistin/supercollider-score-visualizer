/// Configuración específica para el modo visual Airport
#[derive(Debug, Deserialize, Clone, Default)]
pub struct AirportVisualConfig {
    pub plane_change_interval: u64,
    pub scroll_speed: f32,
    pub default_size: f32,
}

use std::fs;
use std::path::Path;
use toml;
use serde::Deserialize;

/// Configuración global de la aplicación. Se carga desde `config.toml` e incluye todos los módulos de configuración.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct AppConfig {
    pub osc: OscConfig,
    pub audio: AudioConfig,
    pub visual: VisualConfig,
    pub midi: MidiConfig,
    pub debug: DebugConfig,
    pub window: WindowConfig,
    pub logging: LoggingConfig,
    pub performance: PerformanceConfig,
    pub airport_visual: AirportVisualConfig,
}

/// Configuración del servidor OSC, incluyendo dirección, puerto y control de buffer y tiempo de espera.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct OscConfig {
    pub listen_host: String,
    pub listen_port: u16,
    pub buffer_size: usize,
    pub timeout_ms: u64,
    pub max_messages_per_frame: usize,
}

/// Configuración del análisis de audio en tiempo real o simulado, incluyendo frecuencias, amplitudes y duración mínima y máxima.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct AudioConfig {
    pub freq_min: f32,
    pub freq_max: f32,
    pub amp_min: f32,
    pub amp_max: f32,
    pub dur_min: f32,
    pub dur_max: f32,
    pub enable_input_capture: bool,
}

/// Parámetros visuales del renderizador: calidad, estilo, opciones de depuración y líneas de rejilla.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct VisualConfig {
    pub quality: String,
    pub theme: String,
    pub debug_mode: bool,
    pub show_fps: bool,
    pub background_color: [f32; 4],
    pub show_grid: bool,
    pub grid_color: [f32; 4],
    pub grid_frequency_lines: Vec<f32>,
    pub timeline_divisions: usize,
    pub timeline_duration: f32,
    pub background_style: String,
    pub show_debug: bool,
}

/// Configuración para entrada MIDI, incluyendo puerto de entrada, duración por defecto y nombres de instrumentos por canal.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct MidiConfig {
    pub enabled: bool,
    pub input_port_name: String,
    pub channel_instruments: Vec<String>,
    pub default_note_duration: f32,
}

/// Parámetros de depuración general como nivel de logs y si deben mostrarse.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct DebugConfig {
    pub enable_logs: bool,
    pub log_level: String,
}

/// Tamaño y propiedades de la ventana de visualización, como título y posibilidad de redimensionar.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub resizable: bool,
}

/// Configuración para mostrar estadísticas de rendimiento y frecuencia de actualización.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct LoggingConfig {
    pub show_performance_stats: bool,
    pub stats_interval_frames: u32,
}

/// Ajustes relacionados con el rendimiento, como la frecuencia de limpieza de datos.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct PerformanceConfig {
    pub cleanup_interval_frames: u32,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new("config.toml");
        let content = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
