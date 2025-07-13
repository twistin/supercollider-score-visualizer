// src/logging.rs - Sistema de logging centralizado para SC Score Visualizer

use log::{debug, error, info, warn};
use std::fmt;

/// Niveles de logging específicos para la aplicación
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Categorías de eventos para logging estructurado
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogCategory {
    System,    // Inicialización, configuración, shutdown
    OSC,       // Mensajes OSC, conexiones, errores de red
    Audio,     // Procesamiento de audio, análisis
    Visual,    // Renderizado, cambios visuales
    Model,     // Cambios de estado del modelo
    Performance, // Métricas de rendimiento
}

impl fmt::Display for LogCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = match self {
            LogCategory::System => "🔧",
            LogCategory::OSC => "📡",
            LogCategory::Audio => "🎵",
            LogCategory::Visual => "🎨",
            LogCategory::Model => "📊",
            LogCategory::Performance => "⚡",
        };
        write!(f, "{}", icon)
    }
}

/// Estructura para logging con contexto
pub struct Logger;

impl Logger {
    /// Inicializa el sistema de logging
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        // Configurar el logger con formato personalizado
        env_logger::Builder::from_default_env()
            .format(|buf, record| {
                use std::io::Write;
                writeln!(
                    buf,
                    "[{}] {} - {}",
                    chrono::Local::now().format("%H:%M:%S%.3f"),
                    record.level(),
                    record.args()
                )
            })
            .init();
        
        info!("🚀 Sistema de logging inicializado");
        Ok(())
    }

    /// Log de sistema (inicialización, configuración)
    pub fn system(level: LogLevel, message: &str) {
        let formatted = format!("{} {}", LogCategory::System, message);
        match level {
            LogLevel::Debug => debug!("{}", formatted),
            LogLevel::Info => info!("{}", formatted),
            LogLevel::Warning => warn!("{}", formatted),
            LogLevel::Error => error!("{}", formatted),
        }
    }

    /// Log de eventos OSC
    pub fn osc(level: LogLevel, message: &str) {
        let formatted = format!("{} {}", LogCategory::OSC, message);
        match level {
            LogLevel::Debug => debug!("{}", formatted),
            LogLevel::Info => info!("{}", formatted),
            LogLevel::Warning => warn!("{}", formatted),
            LogLevel::Error => error!("{}", formatted),
        }
    }

    /// Log de procesamiento de audio
    pub fn audio(level: LogLevel, message: &str) {
        let formatted = format!("{} {}", LogCategory::Audio, message);
        match level {
            LogLevel::Debug => debug!("{}", formatted),
            LogLevel::Info => info!("{}", formatted),
            LogLevel::Warning => warn!("{}", formatted),
            LogLevel::Error => error!("{}", formatted),
        }
    }

    /// Log de eventos visuales
    pub fn visual(level: LogLevel, message: &str) {
        let formatted = format!("{} {}", LogCategory::Visual, message);
        match level {
            LogLevel::Debug => debug!("{}", formatted),
            LogLevel::Info => info!("{}", formatted),
            LogLevel::Warning => warn!("{}", formatted),
            LogLevel::Error => error!("{}", formatted),
        }
    }

    /// Log de cambios en el modelo
    pub fn model(level: LogLevel, message: &str) {
        let formatted = format!("{} {}", LogCategory::Model, message);
        match level {
            LogLevel::Debug => debug!("{}", formatted),
            LogLevel::Info => info!("{}", formatted),
            LogLevel::Warning => warn!("{}", formatted),
            LogLevel::Error => error!("{}", formatted),
        }
    }

    /// Log de métricas de rendimiento
    pub fn performance(level: LogLevel, message: &str) {
        let formatted = format!("{} {}", LogCategory::Performance, message);
        match level {
            LogLevel::Debug => debug!("{}", formatted),
            LogLevel::Info => info!("{}", formatted),
            LogLevel::Warning => warn!("{}", formatted),
            LogLevel::Error => error!("{}", formatted),
        }
    }
}

/// Macros para logging más conveniente
#[macro_export]
macro_rules! log_system {
    ($level:ident, $($arg:tt)*) => {
        $crate::logging::Logger::system($crate::logging::LogLevel::$level, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_osc {
    ($level:ident, $($arg:tt)*) => {
        $crate::logging::Logger::osc($crate::logging::LogLevel::$level, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_audio {
    ($level:ident, $($arg:tt)*) => {
        $crate::logging::Logger::audio($crate::logging::LogLevel::$level, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_visual {
    ($level:ident, $($arg:tt)*) => {
        $crate::logging::Logger::visual($crate::logging::LogLevel::$level, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_model {
    ($level:ident, $($arg:tt)*) => {
        $crate::logging::Logger::model($crate::logging::LogLevel::$level, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_performance {
    ($level:ident, $($arg:tt)*) => {
        $crate::logging::Logger::performance($crate::logging::LogLevel::$level, &format!($($arg)*))
    };
}

/// Estructura para medir tiempos de ejecución
pub struct Timer {
    start: std::time::Instant,
    name: String,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        Self {
            start: std::time::Instant::now(),
            name: name.to_string(),
        }
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }

    pub fn log_if_slow(&self, threshold_ms: f64) {
        let elapsed = self.elapsed_ms();
        if elapsed > threshold_ms {
            log_performance!(Warning, "{} tardó {:.2}ms (límite: {:.2}ms)", 
                           self.name, elapsed, threshold_ms);
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.elapsed_ms();
        log_performance!(Debug, "{} completado en {:.2}ms", self.name, elapsed);
    }
}
