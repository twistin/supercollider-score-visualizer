use std::fs;
use std::path::Path;
use toml;
use serde::Deserialize;

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
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct OscConfig {
    pub listen_host: String,
    pub listen_port: u16,
    pub buffer_size: usize,
    pub timeout_ms: u64,
    pub max_messages_per_frame: usize,
}

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

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MidiConfig {
    pub enabled: bool,
    pub input_port_name: String,
    pub channel_instruments: Vec<String>,
    pub default_note_duration: f32,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct DebugConfig {
    pub enable_logs: bool,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub resizable: bool,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LoggingConfig {
    pub show_performance_stats: bool,
    pub stats_interval_frames: u32,
}

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
