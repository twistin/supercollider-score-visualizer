// src/visuals.rs

use nannou::prelude::*;
use crate::model::Model;

// La función principal que dibuja todo en capas.
pub fn draw_visualization(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    
    // --- CAPA 1: FONDO Y REJILLA PROFESIONAL ---
    // ¡Aquí vuelve tu código!
    draw_modern_background(&draw, win); // Tu fondo azul oscuro
    draw_grid(&draw, win, model);       // Tu rejilla profesional
    
    // Aquí puedes llamar a tus otras funciones de UI
    // draw_stats_ui(&draw, model, win);
    // menu::draw_menu_bar(&draw, model, win);


    // --- CAPA 2: VISUALIZACIÓN DE MASA CLUSTER ---
    // Cluster reactivo que se encoge/expande con la frecuencia
    if model.cluster_data.audio_level > 0.01 {
        let cluster_alpha = map_range(model.cluster_data.audio_level, 0.0, 0.3, 0.0, 0.8);
        let cluster_size = model.cluster_data.size;
        let cluster_density = model.cluster_data.density;
        
        // Color basado en frecuencia
        let cluster_hue = map_range(model.cluster_data.frequency, 200.0, 600.0, 0.1, 0.6);
        
        // Calcular número de partículas basado en densidad
        let num_particles = (cluster_density * 50.0) as i32;
        
        // Dibujar el cluster como múltiples círculos
        for i in 0..num_particles {
            let angle = (i as f32 / num_particles as f32) * TAU;
            let radius_variation = random_range(0.5, 1.0);
            let distance = cluster_size * radius_variation;
            
            let x = distance * angle.cos();
            let y = distance * angle.sin();
            
            // Tamaño de partícula basado en amplitud
            let particle_size = map_range(model.cluster_data.amplitude, 50.0, 800.0, 2.0, 8.0);
            
            // Variación de color por partícula
            let hue_variation = random_range(-0.1, 0.1);
            let particle_alpha = cluster_alpha * random_range(0.3, 1.0);
            
            draw.ellipse()
                .xy(pt2(x, y))
                .radius(particle_size)
                .color(hsva(cluster_hue + hue_variation, 0.8, 0.9, particle_alpha));
        }
        
        // Círculo central más grande
        let center_size = cluster_size * 0.3;
        draw.ellipse()
            .xy(pt2(0.0, 0.0))
            .radius(center_size)
            .color(hsva(cluster_hue, 0.6, 0.7, cluster_alpha * 0.5));
            
        // Anillo exterior
        draw.ellipse()
            .xy(pt2(0.0, 0.0))
            .radius(cluster_size * 1.2)
            .stroke_weight(2.0)
            .stroke(hsva(cluster_hue, 0.9, 1.0, cluster_alpha * 0.3))
            .no_fill();
    }

    // --- CAPA 3: VISUALIZACIÓN DE EVENTOS DRONE ---
    // Círculos concéntricos para eventos de drone - MÁS VISIBLES
    for drone in &model.drone_events {
        let alpha = map_range(drone.time_alive, 0.0, 3.0, 1.0, 0.2).max(0.2);
        let radius = drone.radius * (1.0 + drone.time_alive * 0.3);
        
        // Círculo exterior más grande y visible
        draw.ellipse()
            .xy(drone.position)
            .radius(radius)
            .color(hsva(drone.color.hue.into(), drone.color.saturation, drone.color.value, alpha * 0.6));
            
        // Círculo medio
        draw.ellipse()
            .xy(drone.position)
            .radius(radius * 0.7)
            .color(hsva(drone.color.hue.into(), drone.color.saturation * 0.8, 1.0, alpha * 0.8));
            
        // Círculo interior brillante
        draw.ellipse()
            .xy(drone.position)
            .radius(radius * 0.4)
            .color(hsva(drone.color.hue.into(), drone.color.saturation * 0.5, 1.0, alpha));
    }

    // --- CAPA 4: VISUALIZACIÓN DEL ANÁLISIS CONTINUO (FONDO) ---
    // El círculo en el medio, para análisis continuo
    let circle_radius = map_range(model.analysis.amplitude, 0.0, 0.3, 0.0, 350.0);
    let circle_hue = map_range(model.analysis.brightness, 200.0, 1500.0, 0.0, 0.5);
    let circle_alpha = map_range(model.analysis.amplitude, 0.0, 0.3, 0.0, 0.3);
    let circle_color = hsva(circle_hue, 0.8, 0.9, circle_alpha);

    if model.analysis.amplitude > 0.01 {
        draw.ellipse()
            .xy(pt2(0.0, 0.0))
            .radius(circle_radius)
            .color(circle_color);
    }

    // --- CAPA 5: VISUALIZACIÓN DE EVENTOS DISCRETOS (PBIND) ---
    // Se dibuja encima de todo lo demás.
    for event in &model.events {
        let alpha = map_range(event.time_alive, 0.0, event.duration, 1.0, 0.0);
        let line_color: Srgb<u8> = event.color;
        
        match event.event_type.as_str() {
            "pbind" | "melody_A" => {
                // Líneas horizontales para eventos melódicos
                let line_length = map_range(event.duration, 0.1, 2.0, 30.0, win.w() * 0.6);
                draw.line()
                    .start(event.position)
                    .end(pt2(event.position.x + line_length, event.position.y))
                    .weight(event.size)
                    .color(srgba(line_color.red, line_color.green, line_color.blue, (alpha * 255.0) as u8));
            }
            "percussion" => {
                // Círculos para eventos de percusión
                draw.ellipse()
                    .xy(event.position)
                    .radius(event.size)
                    .color(srgba(line_color.red, line_color.green, line_color.blue, (alpha * 255.0) as u8));
            }
            _ => {
                // Forma por defecto: rectángulo
                draw.rect()
                    .xy(event.position)
                    .w_h(event.size, event.size)
                    .color(srgba(line_color.red, line_color.green, line_color.blue, (alpha * 255.0) as u8));
            }
        }
    }
    
    // --- DEBUG INFO ---
    // Mostrar información de debug en pantalla
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
    
    // Mostrar información del cluster
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
    
    // Mostrar información de cada drone
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
    
    draw.to_frame(app, &frame).unwrap();
}


// --- TUS FUNCIONES HELPER DE DIBUJO ---
// Implementaciones de las funciones de dibujo

/// Dibuja un fondo moderno con gradiente
pub fn draw_modern_background(draw: &Draw, win: Rect) {
    // Fondo con gradiente sutil azul oscuro
    draw.background().color(rgb(0.03, 0.06, 0.12));
    
    // Agregar un gradiente sutil desde el centro
    let center_color = rgba(0.05, 0.08, 0.15, 0.8);
    let _edge_color = rgba(0.02, 0.04, 0.08, 0.3);
    
    draw.ellipse()
        .xy(pt2(0.0, 0.0))
        .radius(win.w().max(win.h()) * 0.7)
        .color(center_color);
}

/// Dibuja una rejilla profesional con líneas de frecuencia
pub fn draw_grid(draw: &Draw, win: Rect, _model: &Model) {
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
        (55.0, "55Hz"),      // A1
        (110.0, "110"),      // A2  
        (220.0, "220"),      // A3
        (440.0, "440"),      // A4 (La central)
        (880.0, "880"),      // A5
        (1760.0, "1.76k"),   // A6
        (3520.0, "3.52k"),   // A7
    ];
    
    for (freq, label) in &main_freq_markers {
        let y = freq_to_y(*freq, win);
        
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

/// Convierte frecuencia a posición Y en la ventana
fn freq_to_y(freq: f32, win: Rect) -> f32 {
    let min_freq = 40.0;
    let max_freq = 4000.0;
    map_range(freq.log2(), min_freq.log2(), max_freq.log2(), win.bottom() + 20.0, win.top() - 20.0)
}
