// src/midi.rs

//! 🎹 Módulo de entrada MIDI para SC Score Visualizer
//!
//! Este módulo gestiona la conexión MIDI utilizando la biblioteca `midir`.
//! Si está activado en la configuración (`MidiConfig`), busca un puerto específico
//! y se conecta a él, registrando los mensajes recibidos.
//!
//! Las funcionalidades futuras incluyen el reenvío de datos MIDI mediante canales
//! MPSC para sincronizar entrada MIDI con visualizaciones y configuración dinámica.

use crate::config::MidiConfig;
use crate::errors::{VisualizerError, VisualizerResult};
use crate::logging::Logger;
use midir::{MidiInput, MidiInputConnection};
// ...existing code...


pub struct MidiController {
    _conn_in: Option<MidiInputConnection<()>>,
    config: MidiConfig,
}

impl std::fmt::Debug for MidiController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MidiController")
            .field("config", &self.config)
            .finish()
    }
}

impl MidiController {
    pub fn new(config: &MidiConfig) -> VisualizerResult<Self> {
        let midi_in = MidiInput::new("nannou-midi-input")
            .map_err(|e| VisualizerError::MidiError {
                message: format!("Error al crear MidiInput: {e}"),
            })?;

        let conn_in = if config.enabled {
            let port_name = &config.input_port_name;
            let available_ports = midi_in.ports();

            let maybe_port = available_ports.iter().find(|p| {
                midi_in
                    .port_name(p)
                    .is_ok_and(|name| name == *port_name)
            });

            if let Some(port) = maybe_port {
                Logger::log_info(&format!(
                    "🎹 Conectando a puerto MIDI: {}",
                    midi_in.port_name(port).unwrap_or_default()
                ));

                let conn = midi_in
                    .connect(
                        port,
                        "nannou-midi-read",
                        Self::midi_callback,
                        (),
                    )
                    .map_err(|e| VisualizerError::MidiError {
                        message: format!("Error conectando MIDI: {e}"),
                    })?;
                Some(conn)
            } else {
                Logger::log_warn(&format!(
                    "⚠️ Puerto MIDI '{port_name}' no encontrado. MIDI deshabilitado."
                ));
                None
            }
        } else {
            Logger::log_info("MIDI deshabilitado en la configuración.");
            None
        };

        Ok(MidiController {
            _conn_in: conn_in,
            config: config.clone(),
        })
    }

    /// Callback ejecutado al recibir un mensaje MIDI.
    /// Actualmente solo registra el contenido del mensaje, pero en el futuro
    /// debería enviar eventos a un canal MPSC para ser procesados en el hilo principal.
    fn midi_callback(_timestamp: u64, message: &[u8], _data: &mut ()) {
        Logger::log_debug(&format!("🎵 MIDI recibido: {message:?}"));
    }

    /// Procesa eventos MIDI recibidos si se implementa un canal MPSC.
    /// Actualmente es un placeholder.
    pub fn handle_midi_events(
        &mut self,
        _config: &mut crate::config::AppConfig,
        _scroll_mode: &mut crate::model::ScrollMode,
        _display_mode: &mut crate::model::DisplayMode,
    ) {
        // Placeholder
    }

    fn midi_to_hz(midi_note: f32) -> f32 {
        440.0 * (2.0f32).powf((midi_note - 69.0) / 12.0)
    }

    fn get_instrument_for_channel(&self, channel: u8) -> String {
        if (channel as usize) < self.config.channel_instruments.len() {
            self.config.channel_instruments[channel as usize].clone()
        } else {
            "default".to_string()
        }
    }
}

impl Drop for MidiController {
    fn drop(&mut self) {
        if self._conn_in.is_some() {
            Logger::log_info("🎹 Cerrando conexión MIDI.");
        }
    }
}
