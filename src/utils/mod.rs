// 🛠️ Módulo de utilidades
// Herramientas comunes para el visualizador

//! Funciones auxiliares para logging y manejo de errores.
//!
//! Este módulo proporciona:
//! - Un sistema de errores unificado mediante `VisualizerError` y `ResultExt`
//! - Inicialización flexible del sistema de logging con `init_logging`
//!
//! Se reexportan los componentes más usados para facilitar su inclusión en otros módulos.

pub mod error;
pub mod logger;

pub use error::{VisualizerError, VisualizerResult, ResultExt};
pub use logger::init_logging;
