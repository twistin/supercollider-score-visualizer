// src/audio/mod.rs - Audio processing module

pub mod osc;

// Re-export main functions for compatibility
pub use osc::process_osc_messages;

// Create the robust function expected by main.rs
pub fn process_osc_messages_robust(model: &mut crate::model::Model, app: &nannou::App) {
    process_osc_messages(model, app);
}
