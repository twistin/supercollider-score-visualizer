// 🛠️ Módulo de utilidades
// Herramientas comunes para el visualizador

pub mod error;
pub mod logger;

pub use error::{VisualizerError, VisualizerResult, ResultExt};
pub use logger::{init_logging, init_simple_logging};
