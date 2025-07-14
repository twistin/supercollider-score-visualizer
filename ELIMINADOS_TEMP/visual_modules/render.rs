// src/visual/render.rs - Funciones de renderizado con Nannou

use crate::core::model::{BackgroundStyle, Model, VisualQuality};
use crate::visual::mapping::{visual_utils, AudioVisualMapping, EventVisualRules};
use nannou::prelude::*;

/// Función principal de renderizado
pub fn draw_visualization(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    // CAPA 1: Fondo y rejilla
    draw_background(&draw, win, &model.display_config.background_style);

    if model.display_config.show_grid {
        draw_grid(&draw, win);
    }

    // CAPA 2: Cluster (si está activo)
    if model.cluster_data.audio_level > 0.01 {
        draw_cluster(
            &draw,
            &model.cluster_data,
            &model.display_config.visual_quality,
        );
    }

    // CAPA 3: Drones
    for drone in &model.drone_events {
        draw_drone(&draw, drone, &model.display_config.visual_quality);
    }

    // CAPA 4: Análisis continuo (fondo)
    if model.analysis.amplitude > 0.01 {
        draw_analysis(&draw, &model.analysis);
    }

    // CAPA 5: Eventos discretos
    for event in &model.events {
        draw_musical_event(&draw, event, win);
    }

    // CAPA 6: Debug info
    if model.display_config.show_debug {
        draw_debug_info(&draw, model, win);
    }

    draw.to_frame(app, &frame).unwrap();
}

/// Dibuja el fondo según el estilo configurado
fn draw_background(draw: &Draw, win: Rect, style: &BackgroundStyle) {
    match style {
        BackgroundStyle::Modern => {
            // Fondo con gradiente sutil azul oscuro
            draw.background().color(rgb(0.03, 0.06, 0.12));

            // Agregar un gradiente sutil desde el centro
            let center_color = rgba(0.05, 0.08, 0.15, 0.8);
            draw.ellipse()
                .xy(pt2(0.0, 0.0))
                .radius(win.w().max(win.h()) * 0.7)
                .color(center_color);
        }
        BackgroundStyle::Simple => {
            draw.background().color(rgb(0.1, 0.1, 0.1));
        }
        BackgroundStyle::Gradient => {
            draw.background().color(rgb(0.02, 0.05, 0.1));
            // Gradiente más pronunciado
            for i in 0..10 {
                let alpha = (i as f32) * 0.1;
                let radius = (win.w().max(win.h()) * 0.1) * (i as f32);
                draw.ellipse().xy(pt2(0.0, 0.0)).radius(radius).color(rgba(
                    0.1,
                    0.2,
                    0.3,
                    alpha * 0.1,
                ));
            }
        }
        BackgroundStyle::None => {
            draw.background().color(BLACK);
        }
    }
}

/// Dibuja la rejilla profesional
fn draw_grid(draw: &Draw, win: Rect) {
    // Líneas verticales de tiempo
    let main_step_x = 120.0;
    let main_line_color = rgba(0.4, 0.6, 0.8, 0.25);
    let count_main_x = (win.w() / main_step_x) as i32;

    for i in 0..=count_main_x.min(12) {
        let x = win.left() + i as f32 * main_step_x;
        draw.line()
            .start(pt2(x, win.bottom()))
            .end(pt2(x, win.top()))
            .stroke_weight(1.0)
            .color(main_line_color);
    }

    // Líneas horizontales de frecuencia (octavas principales)
    let main_freq_markers = [
        (55.0, "55Hz"),    // A1
        (110.0, "110"),    // A2
        (220.0, "220"),    // A3
        (440.0, "440"),    // A4 (La central)
        (880.0, "880"),    // A5
        (1760.0, "1.76k"), // A6
        (3520.0, "3.52k"), // A7
    ];

    let mapping = AudioVisualMapping::default();

    for (freq, label) in &main_freq_markers {
        let y = mapping.freq_to_y(*freq, win);

        // Línea principal de frecuencia
        draw.line()
            .start(pt2(win.left(), y))
            .end(pt2(win.right(), y))
            .stroke_weight(1.0)
            .color(rgba(0.5, 0.7, 0.9, 0.3));

        // Etiqueta de frecuencia
        draw.text(label)
            .x_y(win.left() + 25.0, y + 6.0)
            .font_size(10)
            .color(rgba(0.8, 0.9, 1.0, 0.6));
    }
}

/// Dibuja un cluster de partículas
fn draw_cluster(draw: &Draw, cluster: &crate::core::model::ClusterData, quality: &VisualQuality) {
    let alpha = map_range(cluster.audio_level, 0.0, 0.3, 0.0, 0.8);
    let cluster_size = cluster.size;
    let cluster_density = cluster.density;

    // Color basado en frecuencia
    let mapping = AudioVisualMapping::default();
    let cluster_hue = mapping.freq_to_hue(cluster.frequency);

    // Número de partículas según calidad
    let base_particles = visual_utils::particles_from_density(cluster_density, 50);
    let num_particles = match quality {
        VisualQuality::Low => base_particles / 2,
        VisualQuality::Medium => base_particles,
        VisualQuality::High => base_particles,
        VisualQuality::Ultra => base_particles * 2,
    };

    // Dibujar el cluster como múltiples círculos
    for i in 0..num_particles {
        let angle = (i as f32 / num_particles as f32) * TAU;
        let radius_variation = random_range(0.5, 1.0);
        let distance = cluster_size * radius_variation;

        let x = distance * angle.cos();
        let y = distance * angle.sin();

        // Tamaño de partícula basado en amplitud
        let particle_size = map_range(cluster.amplitude, 50.0, 800.0, 2.0, 8.0);

        // Variación de color por partícula
        let hue_variation = visual_utils::hue_variation(cluster_hue, 0.1);
        let particle_alpha = visual_utils::alpha_with_variation(alpha, 0.3);

        draw.ellipse()
            .xy(pt2(x, y))
            .radius(particle_size)
            .color(hsva(hue_variation, 0.8, 0.9, particle_alpha));
    }

    // Círculo central más grande
    let center_size = cluster_size * 0.3;
    draw.ellipse()
        .xy(pt2(0.0, 0.0))
        .radius(center_size)
        .color(hsva(cluster_hue, 0.6, 0.7, alpha * 0.5));

    // Anillo exterior
    draw.ellipse()
        .xy(pt2(0.0, 0.0))
        .radius(cluster_size * 1.2)
        .stroke_weight(2.0)
        .stroke(hsva(cluster_hue, 0.9, 1.0, alpha * 0.3))
        .no_fill();
}

/// Dibuja un evento de drone
fn draw_drone(draw: &Draw, drone: &crate::core::model::DroneEvent, quality: &VisualQuality) {
    let alpha = map_range(drone.time_alive, 0.0, 3.0, 1.0, 0.2).max(0.2);
    let radius = drone.radius * (1.0 + drone.time_alive * 0.3);

    let num_layers = match quality {
        VisualQuality::Low => 1,
        VisualQuality::Medium => 2,
        VisualQuality::High => 3,
        VisualQuality::Ultra => 4,
    };

    // Múltiples capas concéntricas
    for i in 0..num_layers {
        let layer_factor = 1.0 - (i as f32 * 0.3);
        let layer_alpha = alpha * (1.0 - i as f32 * 0.2);

        draw.ellipse()
            .xy(drone.position)
            .radius(radius * layer_factor)
            .color(hsva(
                drone.color.hue.into(),
                drone.color.saturation * layer_factor,
                drone.color.value,
                layer_alpha,
            ));
    }
}

/// Dibuja el análisis continuo de fondo
fn draw_analysis(draw: &Draw, analysis: &crate::core::model::AnalysisData) {
    let circle_radius = map_range(analysis.amplitude, 0.0, 0.3, 0.0, 350.0);
    let circle_hue = map_range(analysis.brightness, 200.0, 1500.0, 0.0, 0.5);
    let circle_alpha = map_range(analysis.amplitude, 0.0, 0.3, 0.0, 0.3);
    let circle_color = hsva(circle_hue, 0.8, 0.9, circle_alpha);

    draw.ellipse()
        .xy(pt2(0.0, 0.0))
        .radius(circle_radius)
        .color(circle_color);
}

/// Dibuja un evento musical discreto
fn draw_musical_event(draw: &Draw, event: &crate::core::model::MusicalEvent, win: Rect) {
    let mapping = AudioVisualMapping::default();
    let alpha = mapping.time_to_alpha(event.time_alive, event.duration);
    let line_color: Srgb<u8> = event.color;

    match event.event_type.as_str() {
        "pbind" | "melody_A" => {
            // Líneas horizontales para eventos melódicos
            let (line_length_factor, _, _) = EventVisualRules::pbind_config();
            let line_length =
                map_range(event.duration, 0.1, 2.0, 30.0, win.w() * line_length_factor);

            draw.line()
                .start(event.position)
                .end(pt2(event.position.x + line_length, event.position.y))
                .weight(event.size)
                .color(srgba(
                    line_color.red,
                    line_color.green,
                    line_color.blue,
                    (alpha * 255.0) as u8,
                ));
        }
        "percussion" => {
            // Círculos para eventos de percusión
            draw.ellipse()
                .xy(event.position)
                .radius(event.size)
                .color(srgba(
                    line_color.red,
                    line_color.green,
                    line_color.blue,
                    (alpha * 255.0) as u8,
                ));
        }
        _ => {
            // Forma por defecto: rectángulo
            draw.rect()
                .xy(event.position)
                .w_h(event.size, event.size)
                .color(srgba(
                    line_color.red,
                    line_color.green,
                    line_color.blue,
                    (alpha * 255.0) as u8,
                ));
        }
    }
}

/// Dibuja información de debug
fn draw_debug_info(draw: &Draw, model: &Model, win: Rect) {
    // Información principal
    let debug_text = format!(
        "Eventos: {} | Drones: {} | Cluster: {:.1}Hz {:.1}amp {:.1}size | Análisis: amp={:.2}",
        model.events.len(),
        model.drone_events.len(),
        model.cluster_data.frequency,
        model.cluster_data.amplitude,
        model.cluster_data.size,
        model.analysis.amplitude
    );

    draw.text(&debug_text)
        .x_y(win.left() + 20.0, win.top() - 30.0)
        .font_size(14)
        .color(WHITE);

    // Información del cluster
    if model.cluster_data.audio_level > 0.01 {
        let cluster_info = format!(
            "CLUSTER ACTIVO: freq={:.1}Hz amp={:.1} level={:.3} density={:.2}",
            model.cluster_data.frequency,
            model.cluster_data.amplitude,
            model.cluster_data.audio_level,
            model.cluster_data.density
        );

        draw.text(&cluster_info)
            .x_y(win.left() + 20.0, win.top() - 60.0)
            .font_size(12)
            .color(YELLOW);
    }

    // Información de drones
    for (i, drone) in model.drone_events.iter().enumerate() {
        let drone_info = format!(
            "Drone {}: {}Hz {:.2}amp r={:.1} t={:.1}s",
            i + 1,
            drone.frequency,
            drone.amplitude,
            drone.radius,
            drone.time_alive
        );

        draw.text(&drone_info)
            .x_y(win.left() + 20.0, win.top() - 90.0 - (i as f32 * 20.0))
            .font_size(12)
            .color(LIGHTBLUE);
    }
}
