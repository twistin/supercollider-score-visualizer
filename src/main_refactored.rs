// 🎵 SC Score Visualizer - Refactorizado
// Visualizador de eventos musicales desde SuperCollider usando Nannou

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

fn model(app: &App) -> Model {
    initialize_app(app)
}

fn update(app: &App, model: &mut Model, update: Update) {
    update_app(app, model, update);
}
