// 🛠️ Sistema de configuración avanzada
// Maneja la carga, validación y fusión de archivos de configuración

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::utils::{VisualizerError, VisualizerResult};
use tracing::{info, warn, debug};

/// Configuración completa de la aplicación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub visual: VisualConfig,
    pub audio: AudioConfig,
    pub performance: PerformanceConfig,
    pub development: DevelopmentConfig,
}

/// Configuración visual y de renderizado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
    pub fullscreen: bool,
    pub vsync: bool,
    pub background_color: Vec<u8>,
    pub max_fps: u32,
    pub enable_debug_info: bool,
    
    // Configuración de grilla
    pub grid_enabled: bool,
    pub grid_color: Vec<u8>,
    pub grid_frequency_lines: Vec<f32>,
    pub grid_time_divisions: u32,
    
    // Configuración de colores
    pub color_scheme: String,
    pub color_saturation: f32,
    pub color_brightness: f32,
    pub color_contrast: f32,
    
    // Configuración de shaders
    pub shader_hot_reload: bool,
    pub shader_recompile_on_error: bool,
    pub shader_validation_enabled: bool,
}

/// Configuración de audio y OSC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub osc_port: u16,
    pub osc_host: String,
    pub osc_timeout_ms: u32,
    pub osc_buffer_size: u32,
    
    // Configuración de audio
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub input_channels: u8,
    pub output_channels: u8,
    
    // Rangos de visualización
    pub freq_range_min: f32,
    pub freq_range_max: f32,
    pub amp_range_min: f32,
    pub amp_range_max: f32,
    
    // Configuración de eventos
    pub event_cleanup_interval_ms: u32,
    pub max_concurrent_events: u32,
}

/// Configuración de rendimiento y optimización
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub batching_enabled: bool,
    pub time_sync_enabled: bool,
    pub particle_system_enabled: bool,
    pub max_particles: u32,
    pub particle_pool_size: u32,
    
    // Configuración de memoria
    pub event_history_size: u32,
    pub render_cache_size: u32,
    pub gc_interval_ms: u32,
    
    // Configuración de CPU
    pub max_cpu_usage: f32,
    pub frame_skip_threshold: f32,
    pub adaptive_quality: bool,
    
    // Configuración de logging
    pub log_level: String,
    pub log_performance: bool,
    pub log_events: bool,
    pub log_to_file: bool,
    pub log_file_path: String,
    
    // Configuración de exportación
    pub export_format: String,
    pub export_quality: u8,
    pub export_path: String,
    pub auto_export_enabled: bool,
    pub auto_export_interval_s: f32,
}

/// Configuración de desarrollo y debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentConfig {
    pub hot_reload: bool,
    pub debug_mode: bool,
    pub profiling_enabled: bool,
    pub verbose_logging: bool,
    pub test_mode: bool,
    
    // Configuración de pruebas
    pub test_events_enabled: bool,
    pub test_event_interval_ms: u32,
    pub test_event_count: u32,
}

impl AppConfig {
    /// Carga la configuración desde archivos con prioridades
    pub fn load() -> VisualizerResult<Self> {
        info!("🛠️ Cargando configuración de la aplicación...");
        
        let mut settings = config::Config::default();
        
        // 1. Configuración por defecto (obligatoria)
        let default_path = "config/default.toml";
        if Path::new(default_path).exists() {
            info!("📄 Cargando configuración por defecto: {}", default_path);
            settings.merge(config::File::with_name("config/default"))
                .map_err(|e| VisualizerError::config(format!("Error cargando default.toml: {}", e)))?;
        } else {
            warn!("⚠️ No se encontró configuración por defecto, usando valores predeterminados");
            return Ok(Self::default());
        }
        
        // 2. Configuración local (opcional)
        let local_path = "config/local.toml";
        if Path::new(local_path).exists() {
            info!("📄 Cargando configuración local: {}", local_path);
            settings.merge(config::File::with_name("config/local").required(false))
                .map_err(|e| VisualizerError::config(format!("Error cargando local.toml: {}", e)))?;
        } else {
            debug!("📄 No se encontró configuración local, usando solo default.toml");
        }
        
        // 3. Variables de entorno (opcional)
        settings.merge(
            config::Environment::with_prefix("SC_VISUALIZER")
                .separator("__")
        ).ok(); // Ignorar errores de variables de entorno
        
        // Deserializar
        let app_config: AppConfig = settings.try_into()
            .map_err(|e| VisualizerError::config(format!("Error deserializando configuración: {}", e)))?;
        
        // Validar
        app_config.validate_basic()
            .map_err(|e| VisualizerError::config(format!("Error validando configuración: {}", e)))?;
        
        info!("✅ Configuración cargada y validada correctamente");
        debug!("🔧 Configuración final: {:#?}", app_config);
        
        Ok(app_config)
    }
    
    /// Validación básica sin dependencias externas
    pub fn validate_basic(&self) -> VisualizerResult<()> {
        // Validar tamaños de ventana
        if self.visual.window_width < 320 || self.visual.window_width > 7680 {
            return Err(VisualizerError::config(
                "Ancho de ventana debe estar entre 320 y 7680 pixels".to_string()
            ));
        }
        
        if self.visual.window_height < 240 || self.visual.window_height > 4320 {
            return Err(VisualizerError::config(
                "Alto de ventana debe estar entre 240 y 4320 pixels".to_string()
            ));
        }
        
        // Validar puerto OSC
        if self.audio.osc_port < 1024 || self.audio.osc_port > 65535 {
            return Err(VisualizerError::config(
                "Puerto OSC debe estar entre 1024 y 65535".to_string()
            ));
        }
        
        // Validar rangos de frecuencia
        if self.audio.freq_range_min >= self.audio.freq_range_max {
            return Err(VisualizerError::config(
                "La frecuencia mínima debe ser menor que la máxima".to_string()
            ));
        }
        
        // Validar colores
        if self.visual.background_color.len() != 3 {
            return Err(VisualizerError::config(
                "Color de fondo debe tener exactamente 3 valores RGB".to_string()
            ));
        }
        
        if self.visual.grid_color.len() != 4 {
            return Err(VisualizerError::config(
                "Color de grilla debe tener exactamente 4 valores RGBA".to_string()
            ));
        }
        
        // Validar partículas
        if self.performance.particle_pool_size > self.performance.max_particles {
            return Err(VisualizerError::config(
                "El pool de partículas no puede ser mayor que el máximo de partículas".to_string()
            ));
        }
        
        info!("✅ Validación básica completada");
        Ok(())
    }
    
    /// Guarda la configuración actual a un archivo
    pub fn save_to_file(&self, path: &str) -> VisualizerResult<()> {
        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| VisualizerError::config(format!("Error serializando configuración: {}", e)))?;
        
        // Crear directorio si no existe
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| VisualizerError::config(format!("Error creando directorio: {}", e)))?;
        }
        
        std::fs::write(path, toml_string)
            .map_err(|e| VisualizerError::config(format!("Error escribiendo archivo de configuración: {}", e)))?;
        
        info!("💾 Configuración guardada en: {}", path);
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            visual: VisualConfig::default(),
            audio: AudioConfig::default(),
            performance: PerformanceConfig::default(),
            development: DevelopmentConfig::default(),
        }
    }
}

impl Default for VisualConfig {
    fn default() -> Self {
        Self {
            window_width: 1024,
            window_height: 768,
            window_title: "SC Score Visualizer v2.0".to_string(),
            fullscreen: false,
            vsync: true,
            background_color: vec![15, 15, 20],
            max_fps: 60,
            enable_debug_info: false,
            grid_enabled: true,
            grid_color: vec![51, 76, 128, 204],
            grid_frequency_lines: vec![55.0, 110.0, 220.0, 440.0, 880.0, 1760.0],
            grid_time_divisions: 10,
            color_scheme: "default".to_string(),
            color_saturation: 0.8,
            color_brightness: 0.9,
            color_contrast: 1.0,
            shader_hot_reload: true,
            shader_recompile_on_error: true,
            shader_validation_enabled: true,
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            osc_port: 57124,
            osc_host: "127.0.0.1".to_string(),
            osc_timeout_ms: 5000,
            osc_buffer_size: 1024,
            sample_rate: 48000,
            buffer_size: 256,
            input_channels: 2,
            output_channels: 2,
            freq_range_min: 20.0,
            freq_range_max: 2000.0,
            amp_range_min: 0.0,
            amp_range_max: 1.0,
            event_cleanup_interval_ms: 1000,
            max_concurrent_events: 200,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            batching_enabled: true,
            time_sync_enabled: true,
            particle_system_enabled: true,
            max_particles: 1000,
            particle_pool_size: 500,
            event_history_size: 1000,
            render_cache_size: 100,
            gc_interval_ms: 30000,
            max_cpu_usage: 0.8,
            frame_skip_threshold: 50.0,
            adaptive_quality: true,
            log_level: "info".to_string(),
            log_performance: true,
            log_events: false,
            log_to_file: false,
            log_file_path: "logs/visualizer.log".to_string(),
            export_format: "png".to_string(),
            export_quality: 95,
            export_path: "exports/".to_string(),
            auto_export_enabled: false,
            auto_export_interval_s: 60.0,
        }
    }
}

impl Default for DevelopmentConfig {
    fn default() -> Self {
        Self {
            hot_reload: false,
            debug_mode: false,
            profiling_enabled: false,
            verbose_logging: false,
            test_mode: false,
            test_events_enabled: false,
            test_event_interval_ms: 1000,
            test_event_count: 10,
        }
    }
}
