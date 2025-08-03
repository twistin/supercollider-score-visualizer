// src/visual/renderer.rs
use nannou::prelude::*;
use crate::model::Model;
// use crate::visual::VisualNote; // Eliminado: no se usa
use crate::config::AppConfig;
use crate::events::map_freq_to_y; // <-- Importar map_freq_to_y desde events.rs

// Función principal para renderizar el visualizador
pub fn render_visualizer(app: &App, model: &Model, frame: Frame) {
    println!("[DEBUG] render_visualizer llamado");
    let draw = app.draw();
    let win = app.window_rect();

    // Limpiar el fondo con el color configurado
    let bg_color = &model.config.visual.background_color;
draw.background().color(Rgba::new(bg_color[0], bg_color[1], bg_color[2], bg_color[3]));

    // Dibujar cuadrícula si está habilitada
    if model.config.visual.show_grid {
        draw_grid(&draw, &win, &model.config);
    }

    // Dibujar notas visuales
    for note in &model.visual_notes {
        model.shader_manager.render_visual_note(&draw, note);
    }

    // Dibujar línea de tiempo
    draw_timeline_bar(&draw, &win, model.time_info.elapsed_time, &model.config);

    // Dibujar estadísticas OSC y otros datos de depuración
    if model.config.visual.show_debug {
        draw_debug_info(&draw, &win, model);
    }
    // Renderizar al frame
    draw.to_frame(app, &frame).unwrap();
}

// Dibuja la cuadrícula de fondo
fn draw_grid(draw: &Draw, win: &Rect, config: &AppConfig) {
    println!("[DEBUG] draw_grid llamado");
    let grid_color = &config.visual.grid_color;
    let color = Rgba::new(grid_color[0], grid_color[1], grid_color[2], grid_color[3]);

    // Dibujar líneas horizontales de frecuencia
    for &freq in &config.visual.grid_frequency_lines {
        let y = map_freq_to_y(freq, &config.audio, *win);
        draw.line()
            .points(pt2(win.left(), y), pt2(win.right(), y))
            .color(color)
            .stroke_weight(1.0);
        draw.text(&format!("{freq:.0} Hz"))
            .font_size(10)
            .color(color)
            .x(win.left() + 20.0)
            .y(y + 5.0);
    }

    // Líneas verticales (divisiones de tiempo)
    let divisions = config.visual.timeline_divisions.max(1);
    let step_x = win.w() / divisions as f32;
    for i in 0..=divisions {
        let x = win.left() + (i as f32 * step_x);
        draw.line()
            .points(pt2(x, win.bottom()), pt2(x, win.top()))
            .color(color)
            .stroke_weight(1.0);
    }
}

// Dibuja la barra de progreso de la línea de tiempo
fn draw_timeline_bar(draw: &Draw, win: &Rect, current_time: f32, config: &AppConfig) {
    let timeline_duration = config.visual.timeline_duration;
    let progress = (current_time % timeline_duration) / timeline_duration;
    let bar_width = win.w() * progress;

    draw.rect()
        .x(win.left() + bar_width / 2.0)
        .y(win.bottom() + 10.0)
        .w(bar_width)
        .h(5.0)
        .color(STEELBLUE);

    // Dibujar marcadores de tiempo
    let divisions = config.visual.timeline_divisions.max(4);
    for i in 0..=divisions {
        let t = i as f32 * timeline_duration / divisions as f32;
        let x = map_time_to_x(t, win, timeline_duration);
        draw.line()
.points(pt2(x, win.bottom() + 5.0), pt2(x, win.bottom() + 15.0))
            .color(WHITE)
            .stroke_weight(1.0);
        draw.text(&format!("{t:.1}s"))
            .font_size(10)
            .color(WHITE)
            .x(x)
            .y(win.bottom() + 20.0);
    }
}

// Dibuja información de depuración (FPS, estadísticas OSC)
fn draw_debug_info(draw: &Draw, win: &Rect, model: &Model) {
    let text_color = WHITE;
    let font_size = 12;
    let margin = 20.0;

    let fps = 1.0 / model.time_info.last_update_time.elapsed().as_secs_f32();
    let fps_text = draw.text(&format!("FPS: {fps:.0}"))
        .font_size(font_size)
        .color(text_color)
        .x(win.right() - margin)
        .y(win.top() - margin);
    fps_text.right_justify().no_line_wrap();

    let osc_stats_text = draw.text(&format!(
        "OSC Recibidos: {}\nOSC Procesados: {}\nErrores OSC: {}",
        model.osc_stats.total_received,
        model.osc_stats.total_processed,
        model.osc_stats.failed_messages
    ))
    .font_size(font_size)
    .color(text_color)
    .x(win.right() - margin)
    .y(win.top() - margin - 30.0);
    osc_stats_text.right_justify().no_line_wrap();

    let scroll_text = draw.text(&format!(
        "Scroll Mode: {:?}\nDisplay Mode: {:?}",
        model.scroll_mode, model.display_mode
    ))
    .font_size(font_size)
    .color(text_color)
    .x(win.left() + margin)
    .y(win.top() - margin);
    scroll_text.left_justify().no_line_wrap();

    // Mostrar datos de análisis actuales o realtime
    let analysis_text = if let Some(rt_data) = &model.current_realtime_data {
        format!("Realtime: P={:.2}, A={:.2}, C={:.2}", rt_data.pitch, rt_data.amplitude, rt_data.centroid)
    } else {
        format!("Análisis: Amp={:.2}, Bright={:.2}, Noisy={:.2}",
            model.current_analysis_data.0,
            model.current_analysis_data.1,
            model.current_analysis_data.2
        )
    };
    let analysis_text_obj = draw.text(&analysis_text)
        .font_size(font_size)
        .color(text_color)
        .x(win.left() + margin)
        .y(win.top() - margin - 50.0);
    analysis_text_obj.left_justify().no_line_wrap();
}


// map_freq_to_y ha sido movida a events.rs
// fn map_freq_to_y(freq: f32, win: &Rect, audio_config: &crate::config::AudioConfig) -> f32 {
//     map_range(freq, audio_config.freq_min, audio_config.freq_max, win.bottom() + 50.0, win.top() - 50.0)
// }

fn map_time_to_x(time: f32, win: &Rect, timeline_duration: f32) -> f32 {
    map_range(time, 0.0, timeline_duration, win.left() + 50.0, win.right() - 50.0)
}
