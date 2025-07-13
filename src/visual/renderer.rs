// 🎨 Renderizado visual
// Maneja toda la lógica de renderizado de eventos musicales

use nannou::prelude::*;
use crate::app::state::AppState;
use crate::config::AppConfig;
use tracing::warn;

/// Renderiza un frame completo de la aplicación
pub fn render_frame(app: &App, model: &crate::app::lifecycle::Model, frame: Frame) {
    let draw = app.draw();

    // Usar color de fondo de la configuración
    let bg_color = &model.config.visual.background_color;
    draw.background().color(srgb8(bg_color[0], bg_color[1], bg_color[2]));

    let win = app.window_rect();

    // Dibujar grilla de referencia si está habilitada
    if model.config.visual.grid_enabled {
        draw_frequency_grid(&draw, &win, &model.config);
    }

    // Renderizar eventos con renderizador optimizado
    match model.state.get_events_for_render() {
        Ok(events) => {
            model
                .renderer
                .render(&draw, &win, &events, model.state.time);
        }
        Err(e) => {
            warn!("⚠️ Error obteniendo eventos para renderizar: {}", e);
            // Continuar sin renderizar eventos
        }
    }

    // Panel de información
    match draw_info_panel(
        &draw,
        &win,
        model.state.time,
        &model.state,
        &model.config,
    ) {
        Ok(_) => {}
        Err(e) => {
            warn!("⚠️ Error dibujando panel de información: {}", e);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

/// Dibuja la grilla de frecuencias usando configuración
fn draw_frequency_grid(draw: &Draw, win: &Rect, config: &AppConfig) {
    let grid_color = &config.visual.grid_color;
    let grid_rgba = srgba(
        grid_color[0] as f32 / 255.0,
        grid_color[1] as f32 / 255.0,
        grid_color[2] as f32 / 255.0,
        grid_color[3] as f32 / 255.0,
    );

    // Líneas horizontales de frecuencia usando configuración
    for freq in &config.visual.grid_frequency_lines {
        let y = map_range(
            *freq,
            config.audio.freq_range_min,
            config.audio.freq_range_max,
            win.bottom(),
            win.top(),
        );

        // Solo dibujar si está dentro del área visible con margen
        if y >= win.bottom() + 30.0 && y <= win.top() - 30.0 {
            draw.line()
                .start(pt2(win.left(), y))
                .end(pt2(win.right(), y))
                .weight(2.0)
                .color(grid_rgba);

            // Etiqueta de frecuencia - solo mostrar algunas para evitar superposición
            // Mostrar solo cada segunda frecuencia para reducir clutter
            if (*freq as u32) % 440 == 0 || *freq == 1760.0 {
                draw.text(&format!("{}Hz", freq))
                    .xy(pt2(win.left() + 50.0, y + 5.0)) // Posición más separada
                    .font_size(10) // Tamaño menor
                    .color(srgba(0.8, 0.8, 0.9, 0.7)); // Menos opacidad
            }
        }
    }

    // Líneas verticales de tiempo usando configuración
    for i in 0..=config.visual.grid_time_divisions {
        let x = win.left() + (i as f32 * win.w() / config.visual.grid_time_divisions as f32);

        // Solo dibujar si está dentro del área visible con margen
        if x >= win.left() + 60.0 && x <= win.right() - 20.0 {
            draw.line()
                .start(pt2(x, win.bottom()))
                .end(pt2(x, win.top()))
                .weight(1.5)
                .color(srgba(
                    grid_rgba.red * 0.7,
                    grid_rgba.green * 0.7,
                    grid_rgba.blue * 0.7,
                    grid_rgba.alpha * 0.8,
                ));

            // Etiquetas de tiempo - mostrar 0s y luego cada 4 divisiones
            if i == 0 || (i % 4 == 0 && i > 0) {
                let time_seconds = i as f32 * 0.5; // Asumiendo 0.5s por división
                draw.text(&format!("{:.1}s", time_seconds))
                    .xy(pt2(x - 5.0, win.bottom() + 20.0)) // Centrar horizontalmente y posición fija visible
                    .font_size(10) // Tamaño más grande para mejor visibilidad
                    .color(srgba(0.8, 0.8, 0.9, 0.8)); // Más visible
            }
        }
    }
}

/// Dibuja el panel de información
fn draw_info_panel(
    draw: &Draw,
    win: &Rect,
    current_time: f32,
    state: &AppState,
    config: &AppConfig,
) -> anyhow::Result<()> {
    let panel_width = 300.0;
    let panel_height = 140.0;
    let panel_x = win.right() - panel_width / 2.0 - 20.0;
    let panel_y = win.top() - panel_height / 2.0 - 20.0;

    // Fondo del panel
    draw.rect()
        .x_y(panel_x, panel_y)
        .w_h(panel_width, panel_height)
        .color(srgba(0.0, 0.0, 0.0, 0.8))
        .stroke(srgba(0.4, 0.6, 0.9, 0.9))
        .stroke_weight(2.5);

    // Información
    let active_events = state.get_active_events_count().unwrap_or_else(|e| {
        eprintln!("Error obteniendo eventos activos: {}", e);
        0
    });
    let total_events = state.get_total_events_count().unwrap_or_else(|e| {
        eprintln!("Error obteniendo total de eventos: {}", e);
        0
    });

    let status_lines = [
        format!("🎵 {} v2.0", config.visual.window_title),
        format!("📊 Activos: {}/{}", active_events, total_events),
        format!("⏱️ {:.1}s", current_time),
        format!("📡 OSC: {}:{}", config.audio.osc_host, config.audio.osc_port),
        format!(
            "⏰ Sync: {}",
            if config.performance.time_sync_enabled {
                "ON"
            } else {
                "OFF"
            }
        ),
        format!(
            "🎨 Batching: {}",
            if config.performance.batching_enabled {
                "ON"
            } else {
                "OFF"
            }
        ),
        "💾 'S' = Exportar | 'T' = Sync | 'C' = Config".to_string(),
    ];

    // Centrado perfecto
    let line_spacing = 17.0;
    let total_text_height = (status_lines.len() as f32 - 1.0) * line_spacing;
    let text_start_y = panel_y + (total_text_height / 2.0);

    for (i, line) in status_lines.iter().enumerate() {
        let text_y = text_start_y - (i as f32 * line_spacing);

        draw.text(line)
            .xy(pt2(panel_x, text_y))
            .font_size(11)
            .color(srgba(0.7, 0.8, 0.95, 0.95))
            .center_justify();
    }

    Ok(())
}
