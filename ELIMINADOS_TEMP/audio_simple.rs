// src/audio.rs - Manejo de mensajes OSC con logging

use nannou::prelude::*;
use nannou_osc as osc;
use crate::model::Model;

/// Procesa todos los mensajes OSC recibidos y actualiza el modelo
pub fn process_osc_messages(model: &mut Model, app: &App) {
    // Recopilar todos los mensajes para evitar problemas de borrowing
    let mut messages = Vec::new();
    let mut message_count = 0;
    
    for (packet, _addr) in model.osc_receiver.try_iter() {
        for msg in packet.into_msgs() {
            messages.push(msg);
            message_count += 1;
        }
    }
    
    if message_count > 0 {
        println!("📡 OSC: Procesando {} mensajes", message_count);
    }
    
    // Procesar mensajes recopilados
    for msg in messages {
        handle_osc_message(model, &msg, app);
    }
    
    // Actualizar estadísticas OSC
    model.osc_stats.total_messages += message_count as u32;
    
    if message_count > 10 {
        println!("⚠️ OSC: Pico de tráfico - {} mensajes en un frame", message_count);
    }
}

/// Maneja un mensaje OSC individual
fn handle_osc_message(model: &mut Model, msg: &osc::Message, app: &App) {
    let args = &msg.args;
    
    match msg.addr.as_str() {
        "/event" => {
            println!("🎹 OSC: Evento musical recibido");
            handle_event_message(model, args, app);
        },
        "/note" => {
            println!("🎵 OSC: Nota musical recibida");
            handle_event_message(model, args, app); // Usar misma función
        },
        "/analysis" => {
            // Comentado para reducir spam
            handle_analysis_message(model, args);
        },
        "/drone" => {
            println!("🌊 OSC: Evento drone recibido");
            handle_drone_message(model, args);
        },
        "/cluster" => {
            println!("🔵 OSC: Cluster recibido");
            handle_cluster_message(model, args);
        },
        "/ping" => {
            println!("🏓 OSC: Ping recibido - conexión activa");
        },
        "/clear" => {
            println!("🧹 OSC: Comando de limpieza");
            handle_clear_message(model);
        },
        _ => {
            println!("❓ OSC: Mensaje desconocido - {}", msg.addr);
        }
    }
}

/// Maneja eventos musicales (/event y /note)
fn handle_event_message(model: &mut Model, args: &[osc::Type], app: &App) {
    if args.len() >= 4 {
        let event_type = args[0].clone().string().unwrap_or("note".to_string());
        let freq = args[1].clone().float().unwrap_or(440.0);
        let amp = args[2].clone().float().unwrap_or(0.5);
        let dur = args[3].clone().float().unwrap_or(1.0);

        // Validar rangos básicos
        let freq = freq.clamp(20.0, 20000.0);
        let amp = amp.clamp(0.0, 1.0);
        let dur = dur.clamp(0.1, 10.0);

        println!("🎹 Audio: {} - {:.1}Hz {:.2}amp {:.2}s", event_type, freq, amp, dur);
        
        // Agregar nota al modelo
        model.add_note(freq, amp, dur, app.window_rect());
    } else {
        println!("⚠️ OSC: Evento con argumentos insuficientes: {}", args.len());
    }
}

/// Maneja datos de análisis continuo (/analysis)
fn handle_analysis_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 3 {
        let amp = args[0].clone().float().unwrap_or(0.0);
        let brightness = args[1].clone().float().unwrap_or(0.0);
        let noisiness = args[2].clone().float().unwrap_or(0.0);

        // Validar y clamp rangos
        let amp = amp.clamp(0.0, 1.0);
        let brightness = brightness.clamp(0.0, 1.0);
        let noisiness = noisiness.clamp(0.0, 1.0);

        // Actualizar modelo
        model.analysis.amplitude = amp;
        model.analysis.brightness = brightness;
        model.analysis.noisiness = noisiness;
    }
}

/// Maneja eventos de drone (/drone)
fn handle_drone_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 2 {
        let freq = args[0].clone().float().unwrap_or(220.0);
        let amp = args[1].clone().float().unwrap_or(0.5);

        // Validar rangos
        let freq = freq.clamp(20.0, 20000.0);
        let amp = amp.clamp(0.0, 1.0);

        println!("🌊 Audio: Drone {:.1}Hz {:.2}amp", freq, amp);
        
        // Agregar evento drone
        model.add_drone_event(freq, amp);
    } else {
        println!("⚠️ OSC: Drone con argumentos insuficientes: {}", args.len());
    }
}

/// Maneja cluster de datos (/cluster)
fn handle_cluster_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 3 {
        let freq = args[0].clone().float().unwrap_or(300.0);
        let amp = args[1].clone().float().unwrap_or(0.5);
        let audio_level = args[2].clone().float().unwrap_or(0.0);

        // Validar rangos
        let freq = freq.clamp(20.0, 20000.0);
        let amp = amp.clamp(0.0, 1.0);
        let audio_level = audio_level.clamp(0.0, 1.0);

        println!("🔵 Audio: Cluster {:.1}Hz {:.2}amp level={:.3}", freq, amp, audio_level);
        
        // Actualizar cluster
        model.update_cluster_data(freq, amp, audio_level);
    } else {
        println!("⚠️ OSC: Cluster con argumentos insuficientes: {}", args.len());
    }
}

/// Maneja comando de limpieza (/clear)
fn handle_clear_message(model: &mut Model) {
    println!("🧹 Sistema: Limpiando todos los eventos");
    model.notes.clear();
    model.drone_events.clear();
    model.events.clear(); // Alias para compatibilidad
}
