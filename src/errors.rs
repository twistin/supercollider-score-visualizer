// src/errors.rs

use std::fmt;
use std::error::Error;

pub type VisualizerResult<T> = Result<T, VisualizerError>;

#[derive(Debug)]
pub enum VisualizerError {
    ConfigError { message: String },
    OscConnectionError { message: String },
    MidiError { message: String },
    ValidationError {
        field: String,
        expected: String,
        actual: String, // Cambiado de 'found' a 'actual' para coincidir con el uso
        details: String,
    },
    // Añadir este para manejar errores de IO (como los de nannou_osc)
    IoError(std::io::Error),
    // Otros errores...
    GenericError { message: String },
}

impl fmt::Display for VisualizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VisualizerError::ConfigError { message } => write!(f, "Error de configuración: {}", message),
            VisualizerError::OscConnectionError { message } => write!(f, "Error de conexión OSC: {}", message),
            VisualizerError::MidiError { message } => write!(f, "Error MIDI: {}", message),
            VisualizerError::ValidationError { field, expected, actual, details } => {
                write!(f, "Error de validación en '{}': Esperado '{}', Encontrado '{}'. Detalles: {}", field, expected, actual, details)
            },
            VisualizerError::IoError(e) => write!(f, "Error de E/S: {}", e),
            VisualizerError::GenericError { message } => write!(f, "Error genérico: {}", message),
        }
    }
}

impl Error for VisualizerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            VisualizerError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

// Implementar From para std::io::Error para usar el operador '?'
impl From<std::io::Error> for VisualizerError {
    fn from(err: std::io::Error) -> Self {
        VisualizerError::IoError(err)
    }
}

// Implementar From para nannou_osc::CommunicationError
impl From<nannou_osc::CommunicationError> for VisualizerError {
    fn from(err: nannou_osc::CommunicationError) -> Self {
        VisualizerError::OscConnectionError { message: format!("Error de comunicación OSC: {}", err) }
    }
}

// Puedes añadir más implementaciones From para otros tipos de errores si los usas.
// Por ejemplo, para toml::de::Error si cargas configuración TOML
impl From<toml::de::Error> for VisualizerError {
    fn from(err: toml::de::Error) -> Self {
        VisualizerError::ConfigError { message: format!("Error de parseo TOML: {}", err) }
    }
}

impl From<toml::ser::Error> for VisualizerError {
    fn from(err: toml::ser::Error) -> Self {
        VisualizerError::ConfigError { message: format!("Error de serialización TOML: {}", err) }
    }
}
