use nannou::prelude::*;
use crate::model::Model;

pub fn update(app: &App, model: &mut Model, update: Update) {
    model.elapsed_time += update.since_last.as_secs_f32();
    model.handle_osc_messages();
    let window_rect = app.window_rect();
    model.update_visual_notes(window_rect);
}
