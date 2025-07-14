// src/audio/mod.rs - Audio processing module

pub mod osc;
pub mod analyzer;
pub mod buffer;
pub mod iannix_osc;
pub mod osc_handler;

// Re-export main functions for compatibility
pub use osc::process_osc_messages;

// Create the robust function expected by main.rs
pub fn process_osc_messages_robust(model: &mut crate::model::Model, app: &nannou::App) {
    process_osc_messages(model, app);
}
