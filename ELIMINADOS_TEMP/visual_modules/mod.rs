// 🎨 Módulo visual
// Maneja toda la lógica de renderizado y visualización

pub mod renderer;
pub mod primitives;
pub mod styles;
pub mod batching;
pub mod optimized_renderer;
pub mod time_sync;
pub mod shader_hot_reload;
pub mod shader_manager;
pub mod professional_flow;


pub use renderer::{render_frame};
pub use optimized_renderer::OptimizedRenderer;
pub use batching::{RenderBatch, BatchStats, BatchedRenderer};
pub use time_sync::{TimeSync, EventTimeState, GlissandoRenderState};
pub use shader_hot_reload::ShaderHotReloader;
pub use shader_manager::{ShaderManager, ShaderType, ShaderInfo, CompileStatus};
pub use professional_flow::{MusicalEventBuffer, VisualMapping};

