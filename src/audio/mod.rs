// 🎵 Módulo audio
// Maneja toda la lógica de procesamiento de audio y OSC

pub mod analyzer;
pub mod buffer;
pub mod iannix_osc;
pub mod osc;
pub mod osc_handler;
pub mod legacy;

// Re-exportar tipos importantes
pub use analyzer::AudioAnalyzer;
pub use buffer::AudioBuffer;
pub use osc_handler::OscHandler;
pub use iannix_osc::IanniXOscProcessor;

// Re-exportar funciones legacy
pub use legacy::{process_osc_messages, process_osc_messages_robust};