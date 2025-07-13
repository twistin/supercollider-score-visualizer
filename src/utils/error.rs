// 🛠️ Manejo de errores profesional
// Definición de errores específicos del visualizador usando thiserror

use thiserror::Error;

/// Errores específicos del visualizador
#[derive(Error, Debug)]
pub enum VisualizerError {
    /// Errores relacionados con OSC
    #[error("Error OSC: {message}")]
    Osc {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Errores de renderizado
    #[error("Error de renderizado: {message}")]
    Render {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Errores de audio
    #[error("Error de audio: {message}")]
    Audio {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Errores de configuración
    #[error("Error de configuración: {message}")]
    Config {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Errores de inicialización
    #[error("Error de inicialización: {message}")]
    Initialization {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Errores de parseo de eventos
    #[error("Error parseando evento musical: {event_type} - {message}")]
    EventParsing {
        event_type: String,
        message: String,
    },

    /// Errores de estado de la aplicación
    #[error("Error de estado: {message}")]
    State {
        message: String,
    },

    /// Errores de exportación
    #[error("Error exportando: {format} - {message}")]
    Export {
        format: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Errores de ventana/UI
    #[error("Error de ventana: {message}")]
    Window {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Errores de sincronización
    #[error("Error de sincronización: {message}")]
    Sync {
        message: String,
    },
}

impl VisualizerError {
    /// Crear un error OSC con contexto
    pub fn osc(message: impl Into<String>) -> Self {
        Self::Osc {
            message: message.into(),
            source: None,
        }
    }

    /// Crear un error OSC con fuente
    pub fn osc_with_source(
        message: impl Into<String>, 
        source: Box<dyn std::error::Error + Send + Sync>
    ) -> Self {
        Self::Osc {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Crear un error de renderizado
    pub fn render(message: impl Into<String>) -> Self {
        Self::Render {
            message: message.into(),
            source: None,
        }
    }

    /// Crear un error de renderizado con fuente
    pub fn render_with_source(
        message: impl Into<String>, 
        source: Box<dyn std::error::Error + Send + Sync>
    ) -> Self {
        Self::Render {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Crear un error de audio
    pub fn audio(message: impl Into<String>) -> Self {
        Self::Audio {
            message: message.into(),
            source: None,
        }
    }

    /// Crear un error de configuración
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            source: None,
        }
    }

    /// Crear un error de inicialización
    pub fn initialization(message: impl Into<String>) -> Self {
        Self::Initialization {
            message: message.into(),
            source: None,
        }
    }

    /// Crear un error de parseo de eventos
    pub fn event_parsing(event_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::EventParsing {
            event_type: event_type.into(),
            message: message.into(),
        }
    }

    /// Crear un error de estado
    pub fn state(message: impl Into<String>) -> Self {
        Self::State {
            message: message.into(),
        }
    }

    /// Crear un error de exportación
    pub fn export(format: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Export {
            format: format.into(),
            message: message.into(),
            source: None,
        }
    }

    /// Crear un error de ventana
    pub fn window(message: impl Into<String>) -> Self {
        Self::Window {
            message: message.into(),
            source: None,
        }
    }

    /// Crear un error de sincronización
    pub fn sync(message: impl Into<String>) -> Self {
        Self::Sync {
            message: message.into(),
        }
    }
}

/// Resultado con error del visualizador
pub type VisualizerResult<T> = Result<T, VisualizerError>;

/// Trait para convertir errores a VisualizerError con contexto
pub trait ResultExt<T> {
    fn with_osc_context(self, message: impl Into<String>) -> VisualizerResult<T>;
    fn with_render_context(self, message: impl Into<String>) -> VisualizerResult<T>;
    fn with_audio_context(self, message: impl Into<String>) -> VisualizerResult<T>;
    fn with_config_context(self, message: impl Into<String>) -> VisualizerResult<T>;
    fn with_init_context(self, message: impl Into<String>) -> VisualizerResult<T>;
}

impl<T, E> ResultExt<T> for Result<T, E> 
where 
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_osc_context(self, message: impl Into<String>) -> VisualizerResult<T> {
        self.map_err(|e| VisualizerError::osc_with_source(message, Box::new(e)))
    }

    fn with_render_context(self, message: impl Into<String>) -> VisualizerResult<T> {
        self.map_err(|e| VisualizerError::render_with_source(message, Box::new(e)))
    }

    fn with_audio_context(self, message: impl Into<String>) -> VisualizerResult<T> {
        self.map_err(|e| VisualizerError::Audio {
            message: message.into(),
            source: Some(Box::new(e)),
        })
    }

    fn with_config_context(self, message: impl Into<String>) -> VisualizerResult<T> {
        self.map_err(|e| VisualizerError::Config {
            message: message.into(),
            source: Some(Box::new(e)),
        })
    }

    fn with_init_context(self, message: impl Into<String>) -> VisualizerResult<T> {
        self.map_err(|e| VisualizerError::Initialization {
            message: message.into(),
            source: Some(Box::new(e)),
        })
    }
}
