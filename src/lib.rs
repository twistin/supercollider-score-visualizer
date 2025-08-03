/// Lógica de la aplicación principal (estructura Model, update, view, etc.)
pub mod app;
/// Configuración general de la app: audio, visual, MIDI
pub mod config;
/// Tipos de errores personalizados del visualizador
pub mod errors;
/// Inicialización y configuración del sistema de logging
pub mod logging;
/// Manejo de entrada MIDI y conversión a eventos internos
pub mod midi;
/// Estado del modelo y estructuras de datos compartidas
pub mod model;
/// Recepción de mensajes OSC y su interpretación
pub mod osc;
/// Representación visual de los eventos musicales
pub mod visual;
/// Definición de eventos musicales y datos de análisis
pub mod events;
/// Módulo de servidor OSC (legacy, actualmente desacoplado del modelo)
pub mod osc_server;
