// =================================================================
// 🎵 SC SCORE VISUALIZER - VISUALIZACIÓN
// =================================================================
// Funciones para dibujar todo (la grilla, eventos, UI)

use nannou::prelude::*;

use crate::model::Model;
use crate::events::{MusicalEvent, EventType};
use crate::menu;

// =================================================================
// FUNCIÓN PRINCIPAL DE DIBUJO
// =================================================================

pub fn draw_visualization(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window_rect = app.window_rect();
    
    // Ajustar área de dibujo considerando el menú
    let content_rect = if model.ui_state.show_menu {
        Rect::from_x_y_w_h(
            window_rect.x(),
            window_rect.y() - 15.0, // Espacio para el menú
            window_rect.w(),
            window_rect.h() - 30.0,
        )
    } else {
        window_rect
    };
    
    // Limpiar fondo
    draw.background().color(model.get_background_color());
    
    // Dibujar grilla de fondo (sin transformaciones)
    if model.ui_state.show_grid {
        draw_grid(&draw, content_rect, model);
    }
    
    // Aplicar zoom y viewport para eventos
    let zoom = model.ui_state.zoom_level;
    let offset = model.ui_state.viewport_offset;
    let draw_transformed = draw.scale(zoom).translate(vec3(offset.x, offset.y, 0.0));
    
    // Dibujar eventos musicales
    for event in &model.events {
        draw_musical_event(&draw_transformed, event, model);
    }
    
    // Dibujar datos en tiempo real
    if let Some(realtime_data) = &model.realtime_data {
        if realtime_data.is_recent(100) { // Solo si es reciente
            draw_realtime_visualization(&draw_transformed, realtime_data, content_rect);
        }
    }
    
    // Dibujar timer si está habilitado (sin transformaciones)
    if model.ui_state.show_timer {
        draw_timer(&draw, model, content_rect);
    }
    
    // Dibujar barra de menú (sin transformaciones)
    menu::draw_menu_bar(&draw, model, window_rect);
    
    // Dibujar UI de estadísticas (sin transformaciones)
    if model.ui_state.show_stats {
        draw_stats_ui(&draw, model, window_rect);
    }
    
    // Dibujar indicador de performance si está activo (sin transformaciones)
    if model.ui_state.performance_mode {
        draw_performance_indicator(&draw, model, window_rect);
    }
    
    // Aplicar todos los dibujos a la pantalla
    draw.to_frame(app, &frame).unwrap();
}

// =================================================================
// DIBUJO DE LA GRILLA PROFESIONAL
// =================================================================

fn draw_grid(draw: &Draw, window_rect: Rect, model: &Model) {
    let grid_config = &model.config.visual.grid;
    
    // Dibujar fondo con gradiente azul
    draw_modern_background(draw, window_rect);
    
    // Configuración de colores
    let major_color = rgba(
        grid_config.major_color[0],
        grid_config.major_color[1],
        grid_config.major_color[2],
        grid_config.major_color[3],
    );
    let minor_color = rgba(
        grid_config.minor_color[0],
        grid_config.minor_color[1],
        grid_config.minor_color[2],
        grid_config.minor_color[3],
    );
    let center_color = rgba(
        grid_config.center_color[0],
        grid_config.center_color[1],
        grid_config.center_color[2],
        grid_config.center_color[3],
    );
    let label_color = rgba(
        grid_config.label_color[0],
        grid_config.label_color[1],
        grid_config.label_color[2],
        grid_config.label_color[3],
    );
    
    // Dibujar líneas horizontales (frecuencias en eje Y)
    draw_frequency_grid(draw, window_rect, grid_config, major_color, minor_color);
    
    // Dibujar líneas verticales (tiempo en eje X)
    draw_time_grid(draw, window_rect, grid_config, major_color, minor_color);
    
    // Dibujar líneas centrales prominentes con efecto glow
    draw_center_lines_modern(draw, window_rect, center_color);
    
    // Dibujar etiquetas DESPUÉS de las líneas para que aparezcan encima
    if grid_config.show_labels {
        // Etiquetas de frecuencia en el eje Y (izquierda)
        if grid_config.show_frequency_labels {
            draw_frequency_labels_left(draw, window_rect, grid_config, label_color);
        }
        
        // Etiquetas de tiempo en el eje X (abajo)
        if grid_config.show_time_labels {
            draw_time_labels_bottom(draw, window_rect, grid_config, label_color);
        }
        
        // Información de la rejilla
        draw_grid_info_modern(draw, window_rect, grid_config, label_color);
    }
}

fn draw_modern_background(draw: &Draw, window_rect: Rect) {
    // Fondo base azul más oscuro
    draw.rect()
        .wh(window_rect.wh())
        .xy(window_rect.xy())
        .color(rgba(0.03, 0.06, 0.12, 1.0)); // Azul más oscuro
    
    // Gradiente sutil desde el centro
    let center = window_rect.xy();
    let radius = window_rect.w().max(window_rect.h()) * 0.8;
    
    // Crear efecto de resplandor radial más sutil
    for i in 0..15 {
        let alpha = (1.0 - (i as f32 / 15.0)) * 0.015;
        let current_radius = radius * (i as f32 / 15.0);
        
        draw.ellipse()
            .xy(center)
            .radius(current_radius)
            .color(rgba(0.1, 0.2, 0.4, alpha));
    }
    
    // Esquinas con resplandor más sutil
    let corner_glow = rgba(0.05, 0.1, 0.2, 0.2);
    let glow_size = 80.0;
    
    draw.ellipse()
        .xy(pt2(window_rect.left() + glow_size, window_rect.top() - glow_size))
        .radius(glow_size)
        .color(corner_glow);
    
    draw.ellipse()
        .xy(pt2(window_rect.right() - glow_size, window_rect.bottom() + glow_size))
        .radius(glow_size)
        .color(corner_glow);
}

fn draw_frequency_grid(
    draw: &Draw,
    window_rect: Rect,
    grid_config: &crate::model::GridSettings,
    major_color: Rgba,
    minor_color: Rgba,
) {
    let (freq_min, freq_max) = grid_config.frequency_range;
    let freq_min_log = freq_min.log2();
    let freq_max_log = freq_max.log2();
    
    if grid_config.musical_divisions {
        // Divisiones musicales: octavas y semitonos
        draw_musical_frequency_lines(draw, window_rect, freq_min_log, freq_max_log, major_color, minor_color);
    } else {
        // Divisiones lineales regulares
        draw_linear_frequency_lines(draw, window_rect, grid_config, major_color, minor_color);
    }
}

fn draw_time_grid(
    draw: &Draw,
    window_rect: Rect,
    grid_config: &crate::model::GridSettings,
    major_color: Rgba,
    minor_color: Rgba,
) {
    // Líneas verticales mayores (tiempo en eje X)
    for i in 0..=grid_config.major_lines {
        let x = map_range(i, 0, grid_config.major_lines, window_rect.left(), window_rect.right());
        
        // Efecto glow para líneas principales
        for j in 0..2 {
            let alpha = major_color.alpha * (1.0 - (j as f32 / 2.0)) * 0.6;
            let weight = 2.0 - (j as f32 * 0.3);
            
            draw.line()
                .start(pt2(x, window_rect.bottom()))
                .end(pt2(x, window_rect.top()))
                .color(rgba(major_color.red, major_color.green, major_color.blue, alpha))
                .weight(weight);
        }
    }
    
    // Líneas verticales menores
    let total_minor_lines = grid_config.major_lines * grid_config.minor_lines;
    for i in 0..=total_minor_lines {
        if i % grid_config.minor_lines != 0 {
            let x = map_range(i, 0, total_minor_lines, window_rect.left(), window_rect.right());
            
            draw.line()
                .start(pt2(x, window_rect.bottom()))
                .end(pt2(x, window_rect.top()))
                .color(minor_color)
                .weight(1.0);
        }
    }
}

fn draw_musical_frequency_lines(
    draw: &Draw,
    window_rect: Rect,
    freq_min_log: f32,
    freq_max_log: f32,
    major_color: Rgba,
    minor_color: Rgba,
) {
    // Notas musicales (C, C#, D, D#, E, F, F#, G, G#, A, A#, B)
    let _note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    
    // Dibujar líneas HORIZONTALES para cada frecuencia/nota (frecuencias en eje Y)
    for octave in 2..=7 {
        for (note_idx, _note_name) in _note_names.iter().enumerate() {
            // Calcular frecuencia: A4 = 440 Hz, cada semitono = 2^(1/12)
            let freq_hz = 440.0 * 2.0_f32.powf((octave - 4) as f32 + (note_idx as f32 - 9.0) / 12.0);
            let freq_log = freq_hz.log2();
            
            if freq_log >= freq_min_log && freq_log <= freq_max_log {
                let y = map_range(freq_log, freq_min_log, freq_max_log, window_rect.bottom(), window_rect.top());
                
                // Líneas mayores para C (inicio de octava) con efecto glow
                if note_idx == 0 {
                    // Efecto glow para notas C
                    for i in 0..3 {
                        let alpha = major_color.alpha * (1.0 - (i as f32 / 3.0)) * 0.8;
                        let weight = 2.0 - (i as f32 * 0.3);
                        
                        draw.line()
                            .start(pt2(window_rect.left(), y))
                            .end(pt2(window_rect.right(), y))
                            .color(rgba(major_color.red, major_color.green, major_color.blue, alpha))
                            .weight(weight);
                    }
                    
                    // Punto indicador en el lado izquierdo
                    draw.ellipse()
                        .xy(pt2(window_rect.left() + 10.0, y))
                        .radius(3.0)
                        .color(rgba(major_color.red, major_color.green, major_color.blue, 0.8));
                        
                } else {
                    // Líneas menores con gradiente sutil
                    draw.line()
                        .start(pt2(window_rect.left(), y))
                        .end(pt2(window_rect.right(), y))
                        .color(minor_color)
                        .weight(1.0);
                    
                    // Líneas más tenues en el lado izquierdo
                    draw.line()
                        .start(pt2(window_rect.left(), y))
                        .end(pt2(window_rect.left() + 20.0, y))
                        .color(rgba(minor_color.red, minor_color.green, minor_color.blue, minor_color.alpha * 0.5))
                        .weight(0.5);
                }
            }
        }
    }
}

// Nueva función para etiquetas de frecuencia en el EJE Y (izquierda)
fn draw_musical_frequency_labels_left(
    draw: &Draw,
    window_rect: Rect,
    freq_min_log: f32,
    freq_max_log: f32,
    label_color: Rgba,
) {
    // Etiquetas de frecuencia distribuidas logarítmicamente en el eje Y (izquierda)
    let num_labels = 8;
    
    for i in 0..=num_labels {
        let freq_log = map_range(i, 0, num_labels, freq_min_log, freq_max_log);
        let freq_hz = 2.0_f32.powf(freq_log);
        let y = map_range(freq_log, freq_min_log, freq_max_log, window_rect.bottom(), window_rect.top());
        
        // Encontrar la nota más cercana
        let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let a4_freq = 440.0;
        let semitones_from_a4 = (freq_hz / a4_freq).log2() * 12.0;
        let octave = 4 + (semitones_from_a4 / 12.0).floor() as i32;
        let note_idx = ((semitones_from_a4 % 12.0 + 12.0) % 12.0).round() as usize % 12;
        
        // Fondo para etiqueta en el eje Y eliminado - solo texto
        // let bg_color = rgba(0.03, 0.06, 0.12, 0.9);
        // draw.rect()
        //     .xy(pt2(window_rect.left() - 60.0, y))
        //     .wh(pt2(100.0, 30.0))
        //     .color(bg_color);
        // 
        // // Borde sutil
        // draw.rect()
        //     .xy(pt2(window_rect.left() - 60.0, y))
        //     .wh(pt2(100.0, 30.0))
        //     .no_fill()
        //     .stroke_weight(1.0)
        //     .stroke_color(rgba(0.2, 0.4, 0.8, 0.5));
        
        // Etiqueta de nota y octava
        let note_label = format!("{}{}", note_names[note_idx], octave);
        draw.text(&note_label)
            .xy(pt2(window_rect.left() - 60.0, y + 8.0))
            .font_size(12)
            .color(label_color)
            .center_justify();
        
        // Frecuencia en Hz
        let freq_label = if freq_hz >= 1000.0 {
            format!("{:.1}kHz", freq_hz / 1000.0)
        } else {
            format!("{:.0}Hz", freq_hz)
        };
        
        draw.text(&freq_label)
            .xy(pt2(window_rect.left() - 60.0, y - 8.0))
            .font_size(10)
            .color(rgba(label_color.red, label_color.green, label_color.blue, 0.7))
            .center_justify();
        
        // Línea conectora hacia la rejilla
        draw.line()
            .start(pt2(window_rect.left() - 10.0, y))
            .end(pt2(window_rect.left(), y))
            .color(rgba(label_color.red, label_color.green, label_color.blue, 0.5))
            .weight(1.0);
    }
}



fn draw_linear_frequency_lines(
    draw: &Draw,
    window_rect: Rect,
    grid_config: &crate::model::GridSettings,
    major_color: Rgba,
    minor_color: Rgba,
) {
    let (freq_min, freq_max) = grid_config.frequency_range;
    let freq_min_log = freq_min.log2();
    let freq_max_log = freq_max.log2();
    
    // Líneas horizontales mayores (frecuencias en eje Y)
    for i in 0..=grid_config.major_lines {
        let freq_log = map_range(i, 0, grid_config.major_lines, freq_min_log, freq_max_log);
        let y = map_range(freq_log, freq_min_log, freq_max_log, window_rect.bottom(), window_rect.top());
        
        // Efecto glow para líneas principales
        for j in 0..2 {
            let alpha = major_color.alpha * (1.0 - (j as f32 / 2.0)) * 0.6;
            let weight = 2.0 - (j as f32 * 0.3);
            
            draw.line()
                .start(pt2(window_rect.left(), y))
                .end(pt2(window_rect.right(), y))
                .color(rgba(major_color.red, major_color.green, major_color.blue, alpha))
                .weight(weight);
        }
        
        // Punto indicador en el lado izquierdo
        draw.ellipse()
            .xy(pt2(window_rect.left() + 5.0, y))
            .radius(2.0)
            .color(rgba(major_color.red, major_color.green, major_color.blue, 0.6));
    }
    
    // Líneas horizontales menores
    let total_minor_lines = grid_config.major_lines * grid_config.minor_lines;
    for i in 0..=total_minor_lines {
        if i % grid_config.minor_lines != 0 { // Evitar duplicar líneas mayores
            let freq_log = map_range(i, 0, total_minor_lines, freq_min_log, freq_max_log);
            let y = map_range(freq_log, freq_min_log, freq_max_log, window_rect.bottom(), window_rect.top());
            
            draw.line()
                .start(pt2(window_rect.left(), y))
                .end(pt2(window_rect.right(), y))
                .color(minor_color)
                .weight(1.0);
        }
    }
}

fn draw_linear_frequency_labels_left(
    draw: &Draw,
    window_rect: Rect,
    grid_config: &crate::model::GridSettings,
    label_color: Rgba,
) {
    let (freq_min, freq_max) = grid_config.frequency_range;
    let freq_min_log = freq_min.log2();
    let freq_max_log = freq_max.log2();
    
    // Etiquetas de frecuencia distribuidas logarítmicamente en el eje Y (izquierda)
    for i in 0..=grid_config.major_lines {
        let freq_log = map_range(i, 0, grid_config.major_lines, freq_min_log, freq_max_log);
        let freq_hz = 2.0_f32.powf(freq_log);
        let y = map_range(freq_log, freq_min_log, freq_max_log, window_rect.bottom(), window_rect.top());
        
        // Fondo para etiqueta en el eje Y eliminado - solo texto
        // let bg_color = rgba(0.03, 0.06, 0.12, 0.9);
        // draw.rect()
        //     .xy(pt2(window_rect.left() - 60.0, y))
        //     .wh(pt2(100.0, 30.0))
        //     .color(bg_color);
        // 
        // // Borde sutil
        // draw.rect()
        //     .xy(pt2(window_rect.left() - 60.0, y))
        //     .wh(pt2(100.0, 30.0))
        //     .no_fill()
        //     .stroke_weight(1.0)
        //     .stroke_color(rgba(0.2, 0.4, 0.8, 0.5));
        
        // Etiqueta de frecuencia
        let freq_label = if freq_hz >= 1000.0 {
            format!("{:.1}kHz", freq_hz / 1000.0)
        } else {
            format!("{:.0}Hz", freq_hz)
        };
        
        draw.text(&freq_label)
            .xy(pt2(window_rect.left() - 60.0, y))
            .font_size(12)
            .color(label_color)
            .center_justify();
        
        // Línea conectora hacia la rejilla
        draw.line()
            .start(pt2(window_rect.left() - 10.0, y))
            .end(pt2(window_rect.left(), y))
            .color(rgba(label_color.red, label_color.green, label_color.blue, 0.5))
            .weight(1.0);
    }
}

// Función para dibujar etiquetas de frecuencia en el eje Y (izquierda)
fn draw_frequency_labels_left(
    draw: &Draw,
    window_rect: Rect,
    grid_config: &crate::model::GridSettings,
    label_color: Rgba,
) {
    let (freq_min, freq_max) = grid_config.frequency_range;
    let freq_min_log = freq_min.log2();
    let freq_max_log = freq_max.log2();
    
    if grid_config.musical_divisions {
        // Etiquetas musicales distribuidas según las líneas de la rejilla
        for i in 0..=grid_config.major_lines {
            let freq_log = map_range(i, 0, grid_config.major_lines, freq_min_log, freq_max_log);
            let freq_hz = 2.0_f32.powf(freq_log);
            let y = map_range(freq_log, freq_min_log, freq_max_log, window_rect.bottom(), window_rect.top());
            
            // Calcular nota musical más cercana
            let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
            let a4_freq = 440.0;
            let semitones_from_a4 = (freq_hz / a4_freq).log2() * 12.0;
            let octave = 4 + (semitones_from_a4 / 12.0).floor() as i32;
            let note_idx = ((semitones_from_a4 % 12.0 + 12.0) % 12.0).round() as usize % 12;
            
            // Posicionar etiqueta dentro de la ventana (lado izquierdo)
            let label_x = window_rect.left() + 60.0;
            
            // Fondo para etiqueta eliminado - solo texto
            // let bg_color = rgba(0.03, 0.06, 0.12, 0.95);
            // draw.rect()
            //     .xy(pt2(label_x, y))
            //     .wh(pt2(100.0, 30.0))
            //     .color(bg_color);
            // 
            // // Borde
            // draw.rect()
            //     .xy(pt2(label_x, y))
            //     .wh(pt2(100.0, 30.0))
            //     .no_fill()
            //     .stroke_weight(1.0)
            //     .stroke_color(rgba(0.3, 0.6, 1.0, 0.6));
            
            // Texto de nota
            let note_label = format!("{}{}", note_names[note_idx], octave);
            draw.text(&note_label)
                .xy(pt2(label_x, y + 6.0))
                .font_size(12)
                .color(label_color)
                .center_justify();
            
            // Texto de frecuencia
            let freq_label = if freq_hz >= 1000.0 {
                format!("{:.1}kHz", freq_hz / 1000.0)
            } else {
                format!("{:.0}Hz", freq_hz)
            };
            
            draw.text(&freq_label)
                .xy(pt2(label_x, y - 6.0))
                .font_size(10)
                .color(rgba(label_color.red, label_color.green, label_color.blue, 0.8))
                .center_justify();
            
            // Línea conectora a la rejilla
            draw.line()
                .start(pt2(label_x + 50.0, y))
                .end(pt2(window_rect.left() + 120.0, y))
                .color(rgba(label_color.red, label_color.green, label_color.blue, 0.3))
                .weight(1.0);
        }
    } else {
        // Etiquetas lineales de frecuencia
        for i in 0..=grid_config.major_lines {
            let freq_log = map_range(i, 0, grid_config.major_lines, freq_min_log, freq_max_log);
            let freq_hz = 2.0_f32.powf(freq_log);
            let y = map_range(freq_log, freq_min_log, freq_max_log, window_rect.bottom(), window_rect.top());
            
            // Posicionar etiqueta dentro de la ventana (lado izquierdo)
            let label_x = window_rect.left() + 50.0;
            
            // Fondo para etiqueta eliminado - solo texto
            // let bg_color = rgba(0.03, 0.06, 0.12, 0.95);
            // draw.rect()
            //     .xy(pt2(label_x, y))
            //     .wh(pt2(80.0, 25.0))
            //     .color(bg_color);
            // 
            // // Borde
            // draw.rect()
            //     .xy(pt2(label_x, y))
            //     .wh(pt2(80.0, 25.0))
            //     .no_fill()
            //     .stroke_weight(1.0)
            //     .stroke_color(rgba(0.3, 0.6, 1.0, 0.6));
            
            // Texto de frecuencia
            let freq_label = if freq_hz >= 1000.0 {
                format!("{:.1}kHz", freq_hz / 1000.0)
            } else {
                format!("{:.0}Hz", freq_hz)
            };
            
            draw.text(&freq_label)
                .xy(pt2(label_x, y))
                .font_size(11)
                .color(label_color)
                .center_justify();
            
            // Línea conectora a la rejilla
            draw.line()
                .start(pt2(label_x + 40.0, y))
                .end(pt2(window_rect.left() + 100.0, y))
                .color(rgba(label_color.red, label_color.green, label_color.blue, 0.3))
                .weight(1.0);
        }
    }
}

// Función para dibujar etiquetas de tiempo en el eje X (abajo)
fn draw_time_labels_bottom(
    draw: &Draw,
    window_rect: Rect,
    grid_config: &crate::model::GridSettings,
    label_color: Rgba,
) {
    // Dibujar etiquetas exactamente alineadas con las líneas verticales principales
    for i in 0..=grid_config.major_lines {
        let time_value = map_range(i, 0, grid_config.major_lines, 0.0, grid_config.time_range);
        let x = map_range(i, 0, grid_config.major_lines, window_rect.left(), window_rect.right());
        
        // Posicionar etiqueta dentro de la ventana (parte inferior)
        let label_y = window_rect.bottom() + 30.0;
        
        // Fondo para etiqueta eliminado - solo texto
        // let bg_color = rgba(0.03, 0.06, 0.12, 0.95);
        // draw.rect()
        //     .xy(pt2(x, label_y))
        //     .wh(pt2(50.0, 20.0))
        //     .color(bg_color);
        // 
        // // Borde
        // draw.rect()
        //     .xy(pt2(x, label_y))
        //     .wh(pt2(50.0, 20.0))
        //     .no_fill()
        //     .stroke_weight(1.0)
        //     .stroke_color(rgba(0.3, 0.6, 1.0, 0.6));
        
        // Texto de tiempo
        let time_label = format!("{:.1}s", time_value);
        draw.text(&time_label)
            .xy(pt2(x, label_y))
            .font_size(11)
            .color(label_color)
            .center_justify();
        
        // Línea conectora a la rejilla
        draw.line()
            .start(pt2(x, label_y + 10.0))
            .end(pt2(x, window_rect.bottom() - 10.0))
            .color(rgba(label_color.red, label_color.green, label_color.blue, 0.3))
            .weight(1.0);
    }
}

fn draw_center_lines_modern(draw: &Draw, window_rect: Rect, center_color: Rgba) {
    // Línea central horizontal con efecto glow
    for i in 0..5 {
        let alpha = center_color.alpha * (1.0 - (i as f32 / 5.0)) * 0.8;
        let weight = 3.0 - (i as f32 * 0.4);
        
        draw.line()
            .start(pt2(window_rect.left(), 0.0))
            .end(pt2(window_rect.right(), 0.0))
            .color(rgba(center_color.red, center_color.green, center_color.blue, alpha))
            .weight(weight);
    }
    
    // Línea central vertical con efecto glow
    let center_x = window_rect.x();
    for i in 0..5 {
        let alpha = center_color.alpha * (1.0 - (i as f32 / 5.0)) * 0.8;
        let weight = 3.0 - (i as f32 * 0.4);
        
        draw.line()
            .start(pt2(center_x, window_rect.bottom()))
            .end(pt2(center_x, window_rect.top()))
            .color(rgba(center_color.red, center_color.green, center_color.blue, alpha))
            .weight(weight);
    }
    
    // Punto central con resplandor
    draw.ellipse()
        .xy(pt2(center_x, 0.0))
        .radius(8.0)
        .color(rgba(center_color.red, center_color.green, center_color.blue, center_color.alpha * 0.6));
    
    draw.ellipse()
        .xy(pt2(center_x, 0.0))
        .radius(4.0)
        .color(rgba(1.0, 1.0, 1.0, 0.8));
}

fn draw_grid_info_modern(
    draw: &Draw,
    window_rect: Rect,
    grid_config: &crate::model::GridSettings,
    label_color: Rgba,
) {
    // Fondo para la información con gradiente azul
    let info_bg = rgba(0.1, 0.2, 0.4, 0.8);
    let info_rect = Rect::from_x_y_w_h(
        window_rect.left() + 150.0,
        window_rect.top() - 40.0,
        280.0,
        70.0,
    );
    
    // Fondo con bordes redondeados (simulado)
    draw.rect()
        .xy(info_rect.xy())
        .wh(info_rect.wh())
        .color(info_bg);
    
    // Borde con resplandor
    draw.rect()
        .xy(info_rect.xy())
        .wh(info_rect.wh())
        .no_fill()
        .stroke_weight(1.0)
        .stroke_color(rgba(0.3, 0.6, 1.0, 0.6));
    
    // Texto principal con estilo moderno
    let mode_emoji = if grid_config.musical_divisions { "🎵" } else { "📊" };
    let mode_text = if grid_config.musical_divisions { "Musical" } else { "Lineal" };
    
    let info_text = format!("{} Escala {} | {}Hz - {}Hz", 
        mode_emoji,
        mode_text,
        grid_config.frequency_range.0, 
        grid_config.frequency_range.1
    );
    
    draw.text(&info_text)
        .xy(pt2(window_rect.left() + 150.0, window_rect.top() - 30.0))
        .font_size(14)
        .color(label_color)
        .left_justify();
    
    // Información de tiempo con icono
    let time_info = format!("⏱️ Temporal: {}s | Resolución: {}×{}", 
        grid_config.time_range,
        grid_config.major_lines,
        grid_config.minor_lines
    );
    
    draw.text(&time_info)
        .xy(pt2(window_rect.left() + 150.0, window_rect.top() - 50.0))
        .font_size(12)
        .color(rgba(label_color.red, label_color.green, label_color.blue, 0.8))
        .left_justify();
    
    // Indicador de estado en la esquina superior derecha
    let status_text = "● EN VIVO";
    draw.text(status_text)
        .xy(pt2(window_rect.right() - 80.0, window_rect.top() - 20.0))
        .font_size(12)
        .color(rgba(0.3, 1.0, 0.3, 0.9))
        .right_justify();
}

// =================================================================
// DIBUJO DE EVENTOS MUSICALES
// =================================================================

fn draw_musical_event(draw: &Draw, event: &MusicalEvent, model: &Model) {
    let fade_time = model.config.visual.event_fade_time;
    let alpha = event.get_alpha(fade_time);
    
    if alpha <= 0.0 {
        return;
    }
    
    let rgb_color: Rgb = event.color.into();
    let color = rgba(
        rgb_color.red,
        rgb_color.green, 
        rgb_color.blue,
        alpha,
    );
    
    match event.event_type {
        EventType::Point => draw_point_event(draw, event, color, model),
        EventType::Glissando => draw_glissando_event(draw, event, color, model),
        EventType::Texture => draw_texture_event(draw, event, color, model),
        EventType::Rhythm => draw_rhythm_event(draw, event, color, model),
        EventType::Harmony => draw_harmony_event(draw, event, color, model),
    }
}

fn draw_point_event(draw: &Draw, event: &MusicalEvent, color: Rgba, model: &Model) {
    // Círculo principal
    draw.ellipse()
        .xy(event.position)
        .radius(event.size)
        .color(color);
    
    // Halo exterior
    let halo_alpha = color.alpha * 0.3;
    draw.ellipse()
        .xy(event.position)
        .radius(event.size * 2.0)
        .color(rgba(color.red, color.green, color.blue, halo_alpha));
    
    // Trail si está habilitado
    if model.ui_state.show_trails && event.trail.len() > 1 {
        draw_trail(draw, &event.trail, color);
    }
}

fn draw_glissando_event(draw: &Draw, event: &MusicalEvent, color: Rgba, model: &Model) {
    // Línea ondulada basada en curvatura
    let age = event.created_at.elapsed().as_secs_f32();
    let curve_offset = (age * 2.0).sin() * event.curvature * 20.0;
    
    let start = event.position;
    let end = start + vec2(100.0, curve_offset);
    
    draw.line()
        .start(start)
        .end(end)
        .color(color)
        .weight(event.size * 0.5);
        
    // Punto de inicio
    draw_point_event(draw, event, color, model);
}

fn draw_texture_event(draw: &Draw, event: &MusicalEvent, color: Rgba, _model: &Model) {
    // Múltiples puntos pequeños creando textura
    let num_points = (event.timbre * 10.0 + 3.0) as usize;
    let spread = event.size * 1.5;
    
    for i in 0..num_points {
        let angle = (i as f32 / num_points as f32) * TAU;
        let radius = spread * (0.5 + 0.5 * (i as f32 / num_points as f32));
        let offset = vec2(angle.cos(), angle.sin()) * radius;
        
        draw.ellipse()
            .xy(event.position + offset)
            .radius(event.size * 0.3)
            .color(color);
    }
}

fn draw_rhythm_event(draw: &Draw, event: &MusicalEvent, color: Rgba, _model: &Model) {
    // Formas geométricas angulares
    let vertices = 6;
    let points: Vec<Vec2> = (0..vertices)
        .map(|i| {
            let angle = (i as f32 / vertices as f32) * TAU;
            let radius = event.size * (1.0 + 0.3 * (i % 2) as f32);
            event.position + vec2(angle.cos(), angle.sin()) * radius
        })
        .collect();
    
    draw.polygon()
        .points(points)
        .color(color);
}

fn draw_harmony_event(draw: &Draw, event: &MusicalEvent, color: Rgba, _model: &Model) {
    // Múltiples círculos concéntricos
    let num_rings = 3;
    for i in 0..num_rings {
        let radius = event.size * (1.0 + i as f32 * 0.5);
        let ring_alpha = color.alpha * (0.8 - i as f32 * 0.2);
        let ring_rgb: Rgb = color.into();
        
        draw.ellipse()
            .xy(event.position)
            .radius(radius)
            .stroke_weight(2.0)
            .stroke_color(rgba(ring_rgb.red, ring_rgb.green, ring_rgb.blue, ring_alpha))
            .no_fill();
    }
}

fn draw_trail(draw: &Draw, trail: &[Vec2], color: Rgba) {
    if trail.len() < 2 {
        return;
    }
    
    for i in 1..trail.len() {
        let alpha = (i as f32 / trail.len() as f32) * color.alpha * 0.5;
        let trail_rgb: Rgb = color.into();
        let trail_color = rgba(trail_rgb.red, trail_rgb.green, trail_rgb.blue, alpha);
        
        draw.line()
            .start(trail[i-1])
            .end(trail[i])
            .color(trail_color)
            .weight(2.0);
    }
}

// =================================================================
// VISUALIZACIÓN DE DATOS EN TIEMPO REAL
// =================================================================

fn draw_realtime_visualization(draw: &Draw, data: &crate::events::RealtimeData, window_rect: Rect) {
    let bottom_margin = 50.0;
    let bar_height = 20.0;
    
    // Barra de amplitud
    let amp_width = data.amplitude * window_rect.w() * 0.8;
    let amp_rect = Rect::from_x_y_w_h(
        window_rect.left() + amp_width * 0.5,
        window_rect.bottom() + bottom_margin,
        amp_width,
        bar_height,
    );
    
    draw.rect()
        .xy(amp_rect.xy())
        .wh(amp_rect.wh())
        .color(rgba(1.0, 0.3, 0.3, 0.7));
    
    // Indicador de frecuencia
    let freq_x = map_range(
        data.frequency.log2(),
        5.0, 11.0,
        window_rect.left(),
        window_rect.right()
    );
    
    draw.line()
        .start(pt2(freq_x, window_rect.bottom()))
        .end(pt2(freq_x, window_rect.bottom() + bottom_margin * 2.0))
        .color(rgba(0.3, 1.0, 0.3, 0.8))
        .weight(3.0);
    
    // Indicador de onset
    if data.onset_detected {
        draw.ellipse()
            .xy(pt2(freq_x, window_rect.bottom() + bottom_margin * 1.5))
            .radius(15.0)
            .color(rgba(1.0, 1.0, 0.3, 0.9));
    }
}

// =================================================================
// INTERFAZ DE USUARIO
// =================================================================

fn draw_stats_ui(draw: &Draw, model: &Model, window_rect: Rect) {
    let text_size = 14.0;
    let margin = 20.0;
    
    // Posicionar en la esquina superior derecha, pero dentro de la ventana
    let ui_width = 300.0;
    let ui_height = 220.0;
    let stats_x = window_rect.right() - ui_width / 2.0 - margin;
    let stats_y = window_rect.top() - ui_height / 2.0 - margin;
    
    // Fondo semi-transparente con borde
    let ui_bg = match model.ui_state.theme {
        crate::model::ColorTheme::Light => rgba(0.95, 0.95, 0.95, 0.9),
        crate::model::ColorTheme::Dark => rgba(0.03, 0.06, 0.12, 0.9),
        crate::model::ColorTheme::Blue => rgba(0.05, 0.1, 0.2, 0.9),
        crate::model::ColorTheme::Classic => rgba(0.1, 0.1, 0.1, 0.9),
    };
    
    draw.rect()
        .xy(pt2(stats_x, stats_y))
        .wh(pt2(ui_width, ui_height))
        .color(ui_bg);
    
    // Borde según el tema
    let border_color = match model.ui_state.theme {
        crate::model::ColorTheme::Light => rgba(0.6, 0.6, 0.6, 0.8),
        _ => rgba(0.3, 0.6, 1.0, 0.6),
    };
    
    draw.rect()
        .xy(pt2(stats_x, stats_y))
        .wh(pt2(ui_width, ui_height))
        .no_fill()
        .stroke_weight(1.0)
        .stroke_color(border_color);
    
    // Color de texto según el tema
    let text_color = match model.ui_state.theme {
        crate::model::ColorTheme::Light => rgba(0.2, 0.2, 0.4, 1.0),
        _ => rgba(0.8, 0.9, 1.0, 1.0),
    };
    
    let secondary_text_color = match model.ui_state.theme {
        crate::model::ColorTheme::Light => rgba(0.4, 0.4, 0.6, 0.8),
        _ => rgba(1.0, 1.0, 1.0, 0.8),
    };
    
    // Título
    draw.text("📊 ESTADÍSTICAS")
        .xy(pt2(stats_x, stats_y + 95.0))
        .font_size(12)
        .color(text_color)
        .center_justify();
    
    // Timer y duración de sesión
    if model.ui_state.show_timer {
        let timer_text = format!("⏱️ Tiempo: {}", model.get_current_time_formatted());
        draw.text(&timer_text)
            .xy(pt2(stats_x, stats_y + 75.0))
            .font_size(11)
            .color(secondary_text_color)
            .center_justify();
            
        let session_text = format!("📅 Sesión: {}", model.get_session_duration());
        draw.text(&session_text)
            .xy(pt2(stats_x, stats_y + 60.0))
            .font_size(11)
            .color(secondary_text_color)
            .center_justify();
    }
    
    // Estadísticas principales
    let stats_text = format!(
        "FPS: {:.1} | Eventos: {} | EPS: {:.1}",
        model.stats.fps,
        model.events.len(),
        model.stats.events_per_second,
    );
    
    draw.text(&stats_text)
        .xy(pt2(stats_x, stats_y + 40.0))
        .font_size(text_size as u32)
        .color(text_color)
        .center_justify();
    
    // Información de zoom y tema
    let zoom_text = format!("🔍 Zoom: {:.1}x | Tema: {:?}", model.ui_state.zoom_level, model.ui_state.theme);
    draw.text(&zoom_text)
        .xy(pt2(stats_x, stats_y + 25.0))
        .font_size(11)
        .color(secondary_text_color)
        .center_justify();
    
    // Información de rejilla
    let grid_text = format!("📏 Rejilla: {} | Snap: {}/{}", 
        model.ui_state.grid_resolution,
        if model.ui_state.snap_to_x_grid { "X" } else { "-" },
        if model.ui_state.snap_to_y_grid { "Y" } else { "-" }
    );
    draw.text(&grid_text)
        .xy(pt2(stats_x, stats_y + 10.0))
        .font_size(11)
        .color(secondary_text_color)
        .center_justify();
    
    // Indicadores de modo
    let mut mode_indicators = Vec::new();
    if model.ui_state.performance_mode { mode_indicators.push("⚡Performance"); }
    if model.ui_state.fullscreen { mode_indicators.push("🖥️Fullscreen"); }
    if model.ui_state.high_resolution { mode_indicators.push("📺HD"); }
    if model.ui_state.pause_updates { mode_indicators.push("⏸️Paused"); }
    
    if !mode_indicators.is_empty() {
        let mode_text = mode_indicators.join(" | ");
        draw.text(&mode_text)
            .xy(pt2(stats_x, stats_y - 5.0))
            .font_size(10)
            .color(rgba(0.3, 1.0, 0.3, 0.8))
            .center_justify();
    }
    
    // Tiempo real
    let realtime_text = format!(
        "Tiempo real: {}ms",
        if let Some(data) = &model.realtime_data {
            data.received_at.elapsed().as_millis()
        } else {
            0
        }
    );
    
    draw.text(&realtime_text)
        .xy(pt2(stats_x, stats_y - 20.0))
        .font_size(10)
        .color(secondary_text_color)
        .center_justify();
    
    // Controles (texto más compacto)
    let controls_lines = [
        "🎮 CONTROLES:",
        "TAB-Menú | 1-4 Temas | F11-Fullscreen",
        "S-Stats | G-Grid | L-Labels | M-Musical",
        "T-Trails | P-Pause | +/-Zoom | ↑↓Rejilla",
        "5-Performance | 6-Timer | 7-HD | 8-9 Snap"
    ];
    
    for (i, line) in controls_lines.iter().enumerate() {
        let y_offset = stats_y - 35.0 - (i as f32 * 12.0);
        let font_size = if i == 0 { 10 } else { 9 };
        let color = if i == 0 { 
            text_color
        } else { 
            secondary_text_color
        };
        
        draw.text(line)
            .xy(pt2(stats_x, y_offset))
            .font_size(font_size)
            .color(color)
            .center_justify();
    }
}

// =================================================================
// TIMER Y INDICADORES ADICIONALES
// =================================================================

fn draw_timer(draw: &Draw, model: &Model, window_rect: Rect) {
    let time_elapsed = model.stats.last_fps_update.elapsed().as_secs_f32();
    let timer_text = format!("⏱️ {:.1}s", time_elapsed);
    
    let timer_color = match model.ui_state.theme {
        crate::model::ColorTheme::Light => rgba(0.3, 0.3, 0.5, 0.8),
        _ => rgba(0.7, 0.8, 0.9, 0.8),
    };
    
    draw.text(&timer_text)
        .xy(pt2(window_rect.left() + 100.0, window_rect.top() - 50.0))
        .font_size(12)
        .color(timer_color)
        .left_justify();
}

fn draw_performance_indicator(draw: &Draw, model: &Model, window_rect: Rect) {
    let perf_color = rgba(1.0, 0.8, 0.0, 0.9);
    
    // Fondo del indicador
    draw.rect()
        .xy(pt2(window_rect.left() + 80.0, window_rect.bottom() + 20.0))
        .wh(pt2(140.0, 25.0))
        .color(rgba(0.0, 0.0, 0.0, 0.8));
    
    // Texto del indicador
    draw.text("⚡ PERFORMANCE MODE")
        .xy(pt2(window_rect.left() + 80.0, window_rect.bottom() + 20.0))
        .font_size(11)
        .color(perf_color)
        .center_justify();
    
    // Mostrar FPS optimizado
    let fps_text = format!("FPS: {:.0}", model.stats.fps);
    draw.text(&fps_text)
        .xy(pt2(window_rect.left() + 80.0, window_rect.bottom() + 5.0))
        .font_size(10)
        .color(perf_color)
        .center_justify();
}
