use nannou::prelude::*;
use crate::model::Model;

/// Función llamada en cada frame por Nannou. Se encarga de:
/// - Acumular el tiempo total transcurrido (`model.elapsed_time`)
/// - Procesar mensajes OSC entrantes y actualizar el estado del modelo
/// - Obtener dimensiones actuales de la ventana
/// - Actualizar la representación visual de eventos musicales
pub fn update(app: &App, model: &mut Model, update: Update) {
    // ⏱️ Acumula el tiempo transcurrido desde el último frame (para sincronización y animaciones)
    model.time_info.elapsed_time += update.since_last.as_secs_f32();

    // 📡 Procesa mensajes OSC recibidos y actualiza eventos musicales en `model.events`
    model.handle_osc_messages();

    // 📐 Obtiene las dimensiones actuales de la ventana para escalar o centrar visualización
    let window_rect = app.window_rect();

    // 🎵 Actualiza la representación visual de las notas musicales activas o entrantes
    model.update_visual_notes(window_rect);
}
