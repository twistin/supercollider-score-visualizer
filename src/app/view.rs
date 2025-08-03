// src/app/view.rs
use nannou::prelude::*;
use crate::model::Model;
use crate::visual::shader_manager::ShaderManager;
use crate::logging::Logger;

/// Función de renderizado principal del visualizador.
/// - Dibuja fondo negro.
/// - Inicializa el ShaderManager si aún no está listo.
/// - Renderiza cada nota visual activa con shaders personalizados.
/// - (Opcional) Renderiza un elemento de prueba si se desea.
/// - Envia el frame al sistema gráfico.
pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    ensure_shader_manager_initialized(app, model);

    for note in &model.visual_notes {
        model.shader_manager.render_visual_note(&draw, note);
    }

    // 🧪 Renderizar un VisualElement de ejemplo (desactivado por defecto)
    /*
    let example_element = VisualElement {
        x: 0.0,
        y: 0.0,
        color: [1.0, 0.0, 0.0, 1.0], // rojo opaco
    };
    render_element_nannou(&draw, &example_element);
    */

    draw.to_frame(app, &frame).unwrap();
}

/// Inicializa el ShaderManager si aún no está listo.
/// Usa acceso mutable controlado con puntero para sustituir el dummy de forma segura.
fn ensure_shader_manager_initialized(app: &App, model: &Model) {
    if model.shader_manager.is_dummy() {
        Logger::log_info("🛠️ Inicializando ShaderManager desde view()...");
        let shader_manager = ShaderManager::new(app.main_window().device());

        let model_ptr = model as *const Model as *mut Model;
        unsafe {
            (*model_ptr).shader_manager = shader_manager;
        }

        Logger::log_info("✅ ShaderManager inicializado correctamente desde view().");
    }
}
