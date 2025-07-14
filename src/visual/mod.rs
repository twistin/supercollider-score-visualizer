// 🎨 Módulo visual
// Maneja toda la lógica de renderizado y visualización

pub mod audio_visual_mapping;
pub mod audio_visual_mapping_pro;
pub mod shader_manager;
pub mod legacy;
pub mod mapping;

// Re-exportar tipos importantes
pub use audio_visual_mapping::{VisualNote, AudioVisualMapper};
pub use audio_visual_mapping_pro::{ColorPalette, ProAudioVisualMapper};
pub use shader_manager::ShaderManager;
