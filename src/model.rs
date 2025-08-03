mod model_impl;


// Asegurar que los tipos sean públicos
pub use crate::events::MusicalEvent;

/// Representa una nota musical individual con su información acústica y temporal.
/// Usada para seguimiento de ejecución en tiempo real.
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
/// Define el modo de desplazamiento visual (scroll continuo o por páginas).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollMode {
    Continuous,
    Page,
}

/// Define el modo de visualización activa (eventos, análisis, drones, etc.).
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
use crate::visual::audio_visual_mapping::AirportVisualMapper;
use crate::events::RealtimeData;
use crate::osc_server::{OscServer, OscServerStats};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use crate::events::ProcessedOscMessage;
use crate::midi::MidiController;
use std::collections::HashMap;
use std::time::Instant;

/// Información de tiempo del sistema, usada para sincronización visual y lógica.
#[derive(Debug)]
pub struct TimeInfo {
    pub start_time: Instant,
    pub last_update_time: Instant,
    pub elapsed_time: f32,
    pub frame_counter: u64,
}

/// Modelo central de la aplicación.
/// Contiene el estado global: eventos visuales, datos OSC/MIDI, configuración, y estado de tiempo.
#[derive(Debug)]
pub struct Model {
    pub config: AppConfig,
    pub shader_manager: ShaderManager, // Asegúrate que ShaderManager derive Debug
    pub visual_notes: Vec<VisualNote>,
    pub musical_events: Vec<MusicalEvent>,
    pub time_info: TimeInfo,
    pub osc_rx: Receiver<ProcessedOscMessage>,
    pub osc_server_handle: Arc<Mutex<OscServer>>,
    pub osc_stats: OscServerStats,
    pub midi_controller: Option<MidiController>, // Asegúrate que MidiController derive Debug
    pub active_notes: HashMap<u32, VisualNote>,
    pub scroll_mode: ScrollMode,
    pub display_mode: DisplayMode,
    pub current_analysis_data: (f32, f32, f32),
    pub scroll_speed: f32,
    pub notes: Vec<Note>,
    pub drone_events: Vec<crate::events::MusicalEvent>,
    pub current_realtime_data: Option<RealtimeData>,
    pub audio_visual_mapping: AirportVisualMapper,
}

impl Model {
}
