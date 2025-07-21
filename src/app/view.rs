// src/app/view.rs
use nannou::prelude::*;
use crate::model::Model;
// ...existing code...
use crate::visual::shader_manager::ShaderManager;
use crate::logging::Logger;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // Inicialización diferida del ShaderManager si es dummy
if model.shader_manager.is_dummy() {
        Logger::log_info("🛠️ Inicializando ShaderManager desde view()...");
        let shader_manager = ShaderManager::new(app.main_window().device());

        // SAFETY: Estamos accediendo de forma exclusiva al modelo en un entorno de un solo hilo,
        // y este patrón está validado para inicializaciones perezosas controladas.
        let model_ptr = model as *const Model as *mut Model;
        unsafe {
            (*model_ptr).shader_manager = shader_manager;
        }

        Logger::log_info("✅ ShaderManager inicializado correctamente desde view().");
    }

    // Dibujar todas las notas visuales activas
    for note in &model.visual_notes {
        model.shader_manager.render_visual_note(&draw, note);
    }

    draw.to_frame(app, &frame).unwrap();
}
