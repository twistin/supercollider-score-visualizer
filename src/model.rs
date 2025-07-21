mod model_impl;


// Asegurar que los tipos sean públicos
pub use crate::events::MusicalEvent;

// Definir Note si no existe
#[derive(Debug, Clone)]
pub struct Note {
    pub instrument: String,
    pub frequency: f32,
    pub amplitude: f32,
    pub duration: f32,
    pub start_time: Instant,
    pub end_time: Instant,
    pub is_active: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollMode {
    Continuous,
    Page,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    Events,
    Analysis,
    Drones,
    Cluster,
    Combined,
}

use crate::config::AppConfig;
use crate::visual::shader_manager::ShaderManager;
use crate::visual::VisualNote;
use crate::events::RealtimeData;
use crate::osc_server::{OscServer, OscServerStats};
use crate::midi::MidiController;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
pub struct Model {
    pub config: AppConfig,
    pub shader_manager: ShaderManager, // Asegúrate que ShaderManager derive Debug
    pub visual_notes: Vec<VisualNote>,
    pub musical_events: Vec<MusicalEvent>,
    pub start_time: Instant,
    pub elapsed_time: f32,
    pub osc_server: OscServer, // Asegúrate que OscServer derive Debug
    pub osc_stats: OscServerStats,
    pub midi_controller: Option<MidiController>, // Asegúrate que MidiController derive Debug
    pub active_notes: HashMap<u32, VisualNote>,
    pub scroll_mode: ScrollMode,
    pub display_mode: DisplayMode,
    pub current_analysis_data: (f32, f32, f32),
    pub frame_counter: u64,
    pub scroll_speed: f32,
    pub notes: Vec<Note>,
    pub drone_events: Vec<()>,
    pub last_update_time: Instant,
    pub current_realtime_data: Option<RealtimeData>,
}

