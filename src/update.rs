// src/update.rs
use nannou::prelude::*;
use crate::Model;

/// Función principal de actualización por frame.
///
/// Esta función se llama automáticamente en cada ciclo del loop principal de Nannou.
/// Realiza dos tareas críticas:
///
/// 1. Procesa los mensajes OSC recibidos en el canal correspondiente y transforma
///    los datos entrantes en eventos musicales internos del sistema.
///
/// 2. Recalcula y actualiza la posición y apariencia de las notas visuales en pantalla
///    según el tamaño actual de la ventana, permitiendo una representación adaptativa.
pub fn update(app: &App, model: &mut Model, _update: Update) {
    // Procesar mensajes OSC entrantes y convertirlos en eventos musicales
    model.handle_osc_messages();

    // Actualizar visualización de notas según el tamaño actual de la ventana
    let window_rect = app.main_window().rect();
    model.update_visual_notes(window_rect);
}
