// src/audio/osc.rs - Gestión de mensajes OSC

use crate::model::Model;
use nannou::prelude::*;
use nannou_osc as osc;

/// Procesa todos los mensajes OSC recibidos y actualiza el modelo
pub fn process_osc_messages(model: &mut Model, app: &App) {
    // Recopilar todos los mensajes para evitar problemas de borrowing
    let mut messages = Vec::new();
    for (packet, _addr) in model.osc_server.receiver.try_iter() {
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
    println!("🎵 OSC recibido: {} con {} args", msg.addr, msg.args.len());
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
        let event_type = args[0].clone().string().unwrap_or("default".to_string());
        let freq = args[1].clone().float().unwrap_or(440.0);
        let amp = args[2].clone().float().unwrap_or(0.5);
        let dur = args[3].clone().float().unwrap_or(1.0);

        println!(
            "🎹 Evento discreto: {} - {}Hz {}amp {}dur",
            event_type, freq, amp, dur
        );
        model.add_musical_event(freq, amp, dur, app.window_rect());
    }
}

/// Maneja datos de análisis continuo (/analysis)
fn handle_analysis_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 3 {
        let amp = args[0].clone().float().unwrap_or(0.0);
        let bright = args[1].clone().float().unwrap_or(0.0);
        let noisy = args[2].clone().float().unwrap_or(0.0);

        println!(
            "🌊 Análisis continuo: amp={:.3} bright={:.1} noise={:.3}",
            amp, bright, noisy
        );
        model.update_analysis_data(amp, bright, noisy);
    }
}

/// Maneja eventos de drone (/drone)
fn handle_drone_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 2 {
        let freq = args[0].clone().float().unwrap_or(440.0);
        let amp = args[1].clone().float().unwrap_or(0.0);

        println!("🎵 Drone: {}Hz {}amp", freq, amp);
        model.add_drone_event(freq, amp);
    }
}

/// Maneja datos de cluster básico (/cluster)
fn handle_cluster_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 3 {
        let freq = args[0].clone().float().unwrap_or(300.0);
        let amp = args[1].clone().float().unwrap_or(200.0);
        let level = args[2].clone().float().unwrap_or(0.0);

        model.update_cluster_data(freq, amp, level);
    }
}

/// Maneja datos de cluster avanzado (/cluster_advanced)
fn handle_cluster_advanced_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 7 {
        let final_freq = args[0].clone().float().unwrap_or(300.0);
        let final_amp = args[1].clone().float().unwrap_or(200.0);
        let audio_level = args[2].clone().float().unwrap_or(0.0);
        // Los otros parámetros se podrían usar para efectos adicionales

        model.update_cluster_data(final_freq, final_amp, audio_level);
    }
}

/// Maneja datos de cluster extremo (/cluster_extreme)
fn handle_cluster_extreme_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 11 {
        let avg_freq = args[0].clone().float().unwrap_or(300.0);
        let avg_amp = args[1].clone().float().unwrap_or(200.0);
        let total_level = args[2].clone().float().unwrap_or(0.0);
        // Los otros parámetros se podrían usar para efectos de múltiples capas

        model.update_cluster_data(avg_freq, avg_amp, total_level);
    }
}

/// Maneja mensajes de ping (/ping)
fn handle_ping_message() {
    println!("📡 Ping recibido desde SuperCollider");
}

/// Maneja mensajes OSC desconocidos
fn handle_unknown_message(addr: &str) {
    println!("❓ Mensaje OSC desconocido: {}", addr);
}
