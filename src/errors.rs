// src/errors.rs - Definiciones de errores personalizados

use thiserror::Error;

/// Errores específicos del SC Score Visualizer
#[derive(Error, Debug)]
pub enum VisualizerError {
    #[error("Error de conexión OSC: {message}")]
    OscConnectionError { message: String },

    #[error("Error de parsing OSC: dirección '{address}', argumentos inválidos: {details}")]
    OscParsingError { address: String, details: String },

    #[error("Error de renderizado: {message}")]
    RenderError { message: String },

    #[error("Error de configuración: {parameter} = {value}, {reason}")]
    ConfigurationError {
        parameter: String,
        value: String,
        reason: String,
    },

    #[error("Error de modelo: {message}")]
    ModelError { message: String },

    #[error("Error de audio: {message}")]
    AudioError { message: String },

    #[error("Error de ventana: {message}")]
    WindowError { message: String },

    #[error("Error de inicialización: {component} - {message}")]
    InitializationError { component: String, message: String },

    #[error("Error de validación: {field} debe ser {expected}, recibido: {actual}")]
    ValidationError {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("Error de recursos: {resource} no encontrado o inaccesible")]
    ResourceError { resource: String },
}

/// Resultado personalizado para la aplicación
pub type VisualizerResult<T> = Result<T, VisualizerError>;

/// Utilidades para manejo de errores
pub struct ErrorHandler;

impl ErrorHandler {
    /// Maneja errores de OSC con logging apropiado
    pub fn handle_osc_error(error: VisualizerError) {
        match &error {
            VisualizerError::OscConnectionError { message } => {
                crate::log_osc!(Error, "Conexión OSC falló: {}", message);
            }
            VisualizerError::OscParsingError { address, details } => {
                crate::log_osc!(Warning, "Parsing falló para '{}': {}", address, details);
            }
            _ => {
                crate::log_osc!(Error, "Error OSC inesperado: {}", error);
            }
        }
    }

    /// Maneja errores de renderizado
    pub fn handle_render_error(error: VisualizerError) {
        match &error {
            VisualizerError::RenderError { message } => {
                crate::log_visual!(Error, "Renderizado falló: {}", message);
            }
            _ => {
                crate::log_visual!(Error, "Error visual inesperado: {}", error);
            }
        }
    }

    /// Maneja errores de configuración
    pub fn handle_config_error(error: VisualizerError) {
        match &error {
            VisualizerError::ConfigurationError { parameter, value, reason } => {
                crate::log_system!(Error, "Configuración inválida - {}: '{}' ({})", 
                                 parameter, value, reason);
            }
            _ => {
                crate::log_system!(Error, "Error de configuración: {}", error);
            }
        }
    }

    /// Maneja errores de inicialización
    pub fn handle_init_error(error: VisualizerError) -> ! {
        match &error {
            VisualizerError::InitializationError { component, message } => {
                crate::log_system!(Error, "Inicialización falló en {}: {}", component, message);
            }
            _ => {
                crate::log_system!(Error, "Error crítico de inicialización: {}", error);
            }
        }
        
        eprintln!("❌ Error crítico: {}", error);
        eprintln!("🔴 La aplicación no puede continuar");
        std::process::exit(1);
    }

    /// Maneja errores de modelo/datos
    pub fn handle_model_error(error: VisualizerError) {
        match &error {
            VisualizerError::ModelError { message } => {
                crate::log_model!(Warning, "Error en modelo: {}", message);
            }
            VisualizerError::ValidationError { field, expected, actual } => {
                crate::log_model!(Warning, "Validación falló: {} debería ser {}, es {}", 
                                field, expected, actual);
            }
            _ => {
                crate::log_model!(Error, "Error de datos inesperado: {}", error);
            }
        }
    }
}

/// Macros para crear errores más fácilmente
#[macro_export]
macro_rules! osc_error {
    ($msg:expr) => {
        $crate::errors::VisualizerError::OscConnectionError {
            message: $msg.to_string(),
        }
    };
}

#[macro_export]
macro_rules! parse_error {
    ($addr:expr, $details:expr) => {
        $crate::errors::VisualizerError::OscParsingError {
            address: $addr.to_string(),
            details: $details.to_string(),
        }
    };
}

#[macro_export]
macro_rules! render_error {
    ($msg:expr) => {
        $crate::errors::VisualizerError::RenderError {
            message: $msg.to_string(),
        }
    };
}

#[macro_export]
macro_rules! config_error {
    ($param:expr, $value:expr, $reason:expr) => {
        $crate::errors::VisualizerError::ConfigurationError {
            parameter: $param.to_string(),
            value: $value.to_string(),
            reason: $reason.to_string(),
        }
    };
}

#[macro_export]
macro_rules! init_error {
    ($component:expr, $msg:expr) => {
        $crate::errors::VisualizerError::InitializationError {
            component: $component.to_string(),
            message: $msg.to_string(),
        }
    };
}

#[macro_export]
macro_rules! validation_error {
    ($field:expr, $expected:expr, $actual:expr) => {
        $crate::errors::VisualizerError::ValidationError {
            field: $field.to_string(),
            expected: $expected.to_string(),
            actual: $actual.to_string(),
        }
    };
}
