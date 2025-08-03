// Implementación mínima para evitar errores de compilación y permitir integración OSC/MIDI
use crate::model::Model;
use nannou::geom::Rect;

impl Model {
    /// Lee y procesa mensajes OSC no bloqueantes desde el canal correspondiente.
    /// Convierte los mensajes relevantes en eventos musicales internos.
    pub fn handle_osc_messages(&mut self) {
        while let Ok(msg) = self.osc_receiver.try_recv() {
            match msg {
                crate::events::ProcessedOscMessage::NoteEvent(event) => {
                    self.musical_events.push(event);
                }
                // Agrega otros tipos si los tienes
                _ => {}
            }
        }
    }

    /// Recalcula la posición y apariencia de cada nota visual
    /// según el tamaño de la ventana y las configuraciones activas.
    pub fn update_visual_notes(&mut self, window_rect: Rect) {
        for note in &mut self.visual_notes {
            note.update(window_rect, &self.audio_config, &self.visual_config);
        }
    }
}
