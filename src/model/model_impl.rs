use super::{DisplayMode, AppConfig};
use std::sync::{mpsc::Receiver, Arc, Mutex};
use crate::osc_server::OscServer;
use crate::events::ProcessedOscMessage;

impl Model {
    pub fn new_with_receiver(
        config: AppConfig,
        osc_rx: Receiver<ProcessedOscMessage>,
        osc_server_handle: Arc<Mutex<OscServer>>,
    ) -> Self {
        use crate::visual::shader_manager::ShaderManager;
        use crate::osc_server::OscServerStats;
        use std::collections::HashMap;
        use std::time::Instant;

        Self {
            config: config.clone(),
            shader_manager: ShaderManager::dummy(),
            visual_notes: Vec::new(),
            musical_events: Vec::new(),
            time_info: crate::model::TimeInfo {
                start_time: Instant::now(),
                last_update_time: Instant::now(),
                elapsed_time: 0.0,
                frame_counter: 0,
            },
            osc_rx,
            osc_server_handle,
            osc_stats: OscServerStats::default(),
            midi_controller: None,
            active_notes: HashMap::new(),
            scroll_mode: super::ScrollMode::Continuous,
            display_mode: super::DisplayMode::Events,
            current_analysis_data: (0.0, 0.0, 0.0),
            scroll_speed: 100.0,
            notes: Vec::new(),
            drone_events: Vec::new(),
            current_realtime_data: None,
            audio_visual_mapping: crate::visual::audio_visual_mapping::AirportVisualMapper::new(
                config.airport_visual.clone(),
            ),
        }
    }
    pub fn update(&mut self) {
        // Consumir todos los eventos OSC recibidos y agregarlos al vector musical_events
        loop {
            match self.osc_rx.try_recv() {
                Ok(processed_msg) => {
                    let event_opt = match processed_msg.addr.as_str() {
                        "/note_on" => {
                            if processed_msg.args.len() == 3 {
                                let freq = match &processed_msg.args[0] {
                                    nannou_osc::Type::Float(f) => *f,
                                    nannou_osc::Type::Int(i) => *i as f32,
                                    _ => { crate::logging::Logger::log_warn("Argumento de frecuencia inválido para /note_on"); return; },
                                };
                                let amp = match &processed_msg.args[1] {
                                    nannou_osc::Type::Float(f) => *f,
                                    nannou_osc::Type::Int(i) => *i as f32,
                                    _ => { crate::logging::Logger::log_warn("Argumento de amplitud inválido para /note_on"); return; },
                                };
                                let dur = match &processed_msg.args[2] {
                                    nannou_osc::Type::Float(f) => *f,
                                    nannou_osc::Type::Int(i) => *i as f32,
                                    _ => { crate::logging::Logger::log_warn("Argumento de duración inválido para /note_on"); return; },
                                };
                                Some(crate::events::MusicalEvent::Note {
                                    frequency: freq,
                                    amplitude: amp,
                                    duration: dur,
                                    instrument: "default".to_string(),
                                    start_time: processed_msg.timestamp,
                                })
                            } else {
                                crate::logging::Logger::log_warn(&format!("Número incorrecto de argumentos para /note_on: esperado 3, recibido {}", processed_msg.args.len()));
                                None
                            }
                        }
                        // Puedes añadir más mapeos aquí para /drone_on, /cluster, etc.
                        _ => None
                    };
                    if let Some(event) = event_opt {
                        self.musical_events.push(event);
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    crate::logging::Logger::log_error("Canal OSC desconectado");
                    break;
                }
            }
        }
    }
    pub fn cleanup_expired_events(&mut self) {}
    pub fn toggle_scroll_mode(&mut self) {}
    pub fn get_scroll_mode(&self) -> super::ScrollMode { self.scroll_mode }
    pub fn get_scroll_speed(&self) -> f32 { self.scroll_speed }
    pub fn set_scroll_speed(&mut self, speed: f32) { self.scroll_speed = speed; }
    pub fn set_display_mode(&mut self, mode: DisplayMode) { self.display_mode = mode; }
    pub fn clear_events(&mut self) { self.musical_events.clear(); }
    pub fn clear_visual_notes(&mut self) { self.visual_notes.clear(); }
    pub fn display_config(&self) -> &super::AppConfig { &self.config }
    pub fn set_display_config(&mut self, _show_debug: bool, _show_grid: bool) {}
}
// Implementación mínima para evitar errores de compilación y permitir integración OSC/MIDI
use nannou::geom::Rect;
use super::Model;

impl Model {
    /// Procesa mensajes OSC recibidos y los convierte en eventos musicales o visuales.
    pub fn handle_osc_messages(&mut self) {
        // Aquí deberías consumir del canal de eventos OSC y convertirlos en eventos musicales
        // Por ahora, solo placeholder para evitar error de compilación
        // TODO: Integrar con el receiver de ProcessedOscMessage y convertir a MusicalEvent
    }

    /// Actualiza las notas visuales según el estado del modelo y la ventana.
    pub fn update_visual_notes(&mut self, _window_rect: Rect) {
        // Aquí deberías actualizar la posición, estado y visualización de las notas
        // Por ahora, solo placeholder para evitar error de compilación
        // TODO: Implementar lógica de animación y visualización
    }
}
