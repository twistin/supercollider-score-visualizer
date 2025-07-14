// src/audio/osc.rs - Gestión de mensajes OSC

use crate::model::Model;
use nannou::prelude::*;
use nannou_osc as osc;

/// Procesa todos los mensajes OSC recibidos y actualiza el modelo
pub fn process_osc_messages(model: &mut Model, app: &App) {
    // Usar el servidor OSC robusto del modelo
    let processed_messages = model.osc_server.process_messages();
    
    if !processed_messages.is_empty() {
        if model.config.logging.show_osc_messages {
            println!("📡 OSC: Procesando {} mensajes", processed_messages.len());
        }
        
        // Procesar cada mensaje
        for msg in processed_messages {
            handle_osc_message(model, &msg, app);
        }
    }
}

/// Maneja un mensaje OSC procesado individual
fn handle_osc_message(model: &mut Model, msg: &crate::osc_server::ProcessedOscMessage, app: &App) {
    match msg.address.as_str() {
        "/note" => {
            if model.config.logging.show_osc_messages {
                println!("🎵 OSC: Nota musical recibida");
            }
            handle_note_message(model, &msg.args, app);
        },
        "/event" => {
            if model.config.logging.show_osc_messages {
                println!("🎹 OSC: Evento musical recibido");
            }
            handle_note_message(model, &msg.args, app);
        },
        "/analyzer" => {
            if model.config.logging.show_osc_messages {
                println!("🔍 OSC: Mensaje de análisis recibido");
            }
            handle_analyzer_message(model, &msg.args, app);
        },
        "/analysis" => {
            handle_analysis_message(model, &msg.args);
        },
        "/drone" => {
            if model.config.logging.show_osc_messages {
                println!("🌊 OSC: Evento drone recibido");
            }
            handle_drone_message(model, &msg.args);
        },
        "/cluster" => {
            if model.config.logging.show_osc_messages {
                println!("🔵 OSC: Cluster recibido");
            }
            handle_cluster_message(model, &msg.args);
        },
        "/ping" => {
            if model.config.logging.show_osc_messages {
                println!("🏓 OSC: Ping recibido de {}", msg.source_addr);
            }
        },
        "/clear" => {
            println!("🧹 OSC: Comando de limpieza");
            handle_clear_message(model);
        },
        _ => {
            if model.config.logging.show_osc_messages {
                println!("❓ OSC: Mensaje desconocido - {} de {}", msg.address, msg.source_addr);
            }
        }
    }
}

/// Maneja mensajes de nota musical (/note y /event)
fn handle_note_message(model: &mut Model, args: &[osc::Type], app: &App) {
    if args.len() >= 3 {
        let freq = args[0].clone().float().unwrap_or(440.0);
        let amp = args[1].clone().float().unwrap_or(0.5);
        let dur = args[2].clone().float().unwrap_or(1.0);

        // Validar rangos básicos
        let freq = freq.clamp(model.config.audio.freq_min, model.config.audio.freq_max);
        let amp = amp.clamp(model.config.audio.amp_min, model.config.audio.amp_max);
        let dur = dur.clamp(model.config.audio.dur_min, model.config.audio.dur_max);

        if model.config.logging.show_osc_messages {
            println!("🎹 Audio: Nota - {:.1}Hz {:.2}amp {:.2}s", freq, amp, dur);
        }
        
        // Agregar nota al modelo
        model.add_note(freq, amp, dur, app.window_rect());
    } else {
        println!("⚠️ OSC: Nota con argumentos insuficientes: {}", args.len());
    }
}

/// Maneja mensajes del analizador (/analyzer) - NUEVA FUNCIONALIDAD
fn handle_analyzer_message(model: &mut Model, args: &[osc::Type], app: &App) {
    // Esperar formato: /analyzer timestamp pitch amplitude onset duration timbre_features
    if args.len() >= 6 {
        let timestamp = args[0].clone().float().unwrap_or(0.0);
        let pitch = args[1].clone().float().unwrap_or(440.0);
        let amplitude = args[2].clone().float().unwrap_or(0.5);
        let onset = args[3].clone().float().unwrap_or(0.0);
        let duration = args[4].clone().float().unwrap_or(1.0);
        let timbre_features = args[5].clone().float().unwrap_or(0.5);

        // Validar rangos
        let pitch = pitch.clamp(model.config.audio.freq_min, model.config.audio.freq_max);
        let amplitude = amplitude.clamp(model.config.audio.amp_min, model.config.audio.amp_max);
        let duration = duration.clamp(model.config.audio.dur_min, model.config.audio.dur_max);
        let timbre_features = timbre_features.clamp(0.0, 1.0);

        if model.config.logging.show_osc_messages {
            println!("🔍 Analyzer: t={:.2} p={:.1}Hz a={:.2} o={:.2} d={:.2} t={:.2}", 
                   timestamp, pitch, amplitude, onset, duration, timbre_features);
        }
        
        // Crear evento visual con datos de análisis
        model.add_analyzer_event(timestamp, pitch, amplitude, onset, duration, timbre_features, app.window_rect());
    } else {
        println!("⚠️ OSC: /analyzer con argumentos insuficientes: {} (esperados: 6)", args.len());
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
        let freq = freq.clamp(model.config.audio.freq_min, model.config.audio.freq_max);
        let amp = amp.clamp(model.config.audio.amp_min, model.config.audio.amp_max);

        if model.config.logging.show_osc_messages {
            println!("🌊 Audio: Drone {:.1}Hz {:.2}amp", freq, amp);
        }
        
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
        let freq = freq.clamp(model.config.audio.freq_min, model.config.audio.freq_max);
        let amp = amp.clamp(model.config.audio.amp_min, model.config.audio.amp_max);
        let audio_level = audio_level.clamp(0.0, 1.0);

        if model.config.logging.show_osc_messages {
            println!("🔵 Audio: Cluster {:.1}Hz {:.2}amp level={:.3}", freq, amp, audio_level);
        }
        
        // Actualizar cluster
        model.update_cluster_data(freq, amp, audio_level);
    } else {
        println!("⚠️ OSC: Cluster con argumentos insuficientes: {}", args.len());
    }
}

/// Maneja comando de limpieza (/clear)
fn handle_clear_message(model: &mut Model) {
    let before_notes = model.notes.len();
    let before_drones = model.drone_events.len();
    let before_visual = model.visual_notes.len();
    
    model.notes.clear();
    model.drone_events.clear();
    model.visual_notes.clear();
    
    println!("🧹 Sistema: Limpieza completa - {} notas, {} drones, {} visuales eliminados", 
           before_notes, before_drones, before_visual);
}
