// src/audio/osc.rs - Gestión de mensajes OSC

use crate::core::model::Model;
use nannou::prelude::*;
use nannou_osc as osc;

fn get_float_arg(args: &[osc::Type], index: usize, default: f32) -> f32 {
    args.get(index)
        .and_then(|v| v.clone().float())
        .unwrap_or(default)
}

/// Procesa todos los mensajes OSC recibidos y actualiza el modelo
pub fn process_osc_messages(model: &mut Model, app: &App) {
    // Recopilar todos los mensajes para evitar problemas de borrowing
    let mut messages = Vec::new();
    for (packet, _addr) in model.osc_receiver.try_iter() {
        for msg in packet.into_msgs() {
            messages.push(msg);
        }
    }

    // Procesar mensajes recopilados
    for msg in messages {
        handle_osc_message(model, &msg, app);
    }
}

/// Maneja un mensaje OSC individual
fn handle_osc_message(model: &mut Model, msg: &osc::Message, app: &App) {
    log::info!("🎵 OSC recibido: {} con {} args", msg.addr, msg.args.len());
    let args = &msg.args;

    match msg.addr.as_str() {
        "/event" => handle_event_message(model, args, app),
        "/analysis" => handle_analysis_message(model, args),
        "/drone" => handle_drone_message(model, args),
        "/cluster" => handle_cluster_message(model, args),
        "/cluster_advanced" => handle_cluster_advanced_message(model, args),
        "/cluster_extreme" => handle_cluster_extreme_message(model, args),
        "/ping" => handle_ping_message(),
        _ => handle_unknown_message(&msg.addr),
    }
}

/// Maneja eventos discretos (/event)
fn handle_event_message(model: &mut Model, args: &[osc::Type], app: &App) {
    if args.len() >= 4 {
        let event_type = args.get(0).and_then(|v| v.clone().string()).unwrap_or("default".to_string());
        let freq = get_float_arg(args, 1, 440.0);
        let amp = get_float_arg(args, 2, 0.5);
        let dur = get_float_arg(args, 3, 1.0);

        log::info!(
            "🎹 Evento discreto: {} - {}Hz {}amp {}dur",
            event_type, freq, amp, dur
        );
        model.add_musical_event(freq, amp, dur, app.window_rect());
    }
}

/// Maneja datos de análisis continuo (/analysis)
fn handle_analysis_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 3 {
        let amp = get_float_arg(args, 0, 0.0);
        let bright = get_float_arg(args, 1, 0.0);
        let noisy = get_float_arg(args, 2, 0.0);

        log::info!(
            "🌊 Análisis continuo: amp={:.3} bright={:.1} noise={:.3}",
            amp, bright, noisy
        );
        model.update_analysis_data(amp, bright, noisy);
    }
}

/// Maneja eventos de drone (/drone)
fn handle_drone_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 2 {
        let freq = get_float_arg(args, 0, 440.0);
        let amp = get_float_arg(args, 1, 0.0);

        log::info!("🎵 Drone: {}Hz {}amp", freq, amp);
        model.add_drone_event(freq, amp);
    }
}

/// Maneja datos de cluster básico (/cluster)
fn handle_cluster_message(model: &mut Model, args: &[osc::Type]) {
    let freq = get_float_arg(args, 0, 300.0);
    let amp = get_float_arg(args, 1, 200.0);
    let level = get_float_arg(args, 2, 0.0);
    model.update_cluster_data(freq, amp, level);
}

/// Maneja datos de cluster avanzado (/cluster_advanced)
fn handle_cluster_advanced_message(model: &mut Model, args: &[osc::Type]) {
    handle_cluster_message(model, args);
}

/// Maneja datos de cluster extremo (/cluster_extreme)
fn handle_cluster_extreme_message(model: &mut Model, args: &[osc::Type]) {
    handle_cluster_message(model, args);
}

/// Maneja mensajes de ping (/ping)
fn handle_ping_message() {
    log::info!("📡 Ping recibido desde SuperCollider");
}

/// Maneja mensajes OSC desconocidos
fn handle_unknown_message(addr: &str) {
    log::info!("❓ Mensaje OSC desconocido: {}", addr);
}
