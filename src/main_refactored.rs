//! 🌟 Punto de entrada refactorizado para SC Score Visualizer v2.0
//!
//! Este binario usa una arquitectura modular, separando claramente:
//! - `musical_events`: definición de eventos musicales (notas, clusters, etc.)
//! - `app::lifecycle`: ciclo de vida de la app (model, update, init...)
//! - `audio`: procesamiento de audio (RMS, espectro...)
//! - `visual`: renderizado visual con Nannou
//!
//! Para lanzar esta versión desde terminal:
//! ```bash
//! cargo run --bin main_refactored
//! ```

mod musical_events;
mod app;
mod audio;
mod visual;

use nannou::prelude::*;
use app::lifecycle::{initialize_app, update_app, Model};

fn main() {
    println!("🎵 Iniciando SC Score Visualizer v2.0...");
    nannou::app(model)
        .update(update)
        .run();
}

/// Construye e inicializa el modelo principal delegando a `app::lifecycle::initialize_app`.
fn model(app: &App) -> Model {
    initialize_app(app)
}

/// Función de actualización que se ejecuta en cada frame.
/// Llama a `update_app` para lógica de aplicación y animación.
fn update(app: &App, model: &mut Model, update: Update) {
    update_app(app, model, update);
}
