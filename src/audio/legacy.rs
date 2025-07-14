// src/audio.rs - Manejo de mensajes OSC con servidor robusto

use nannou::prelude::*;
use nannou_osc as osc;
use crate::model::Model;
use crate::osc_server::{ProcessedOscMessage, OscMessageValidator};

/// Procesa todos los mensajes OSC recibidos y actualiza el modelo (método legacy)
pub fn process_osc_messages(model: &mut Model, app: &App) {
    // Redirigir al método robusto
    process_osc_messages_robust(model, app);
}

/// Procesa todos los mensajes OSC recibidos usando el servidor robusto
pub fn process_osc_messages_robust(model: &mut Model, app: &App) {
    // Obtener mensajes del servidor OSC robusto
    let processed_messages = model.osc_server.process_messages();
    
    if !processed_messages.is_empty() {
        if model.config.logging.show_osc_messages {
            println!("📡 OSC: Procesando {} mensajes", processed_messages.len());
        }
        
        // Crear validador con configuración de audio
        let validator = OscMessageValidator::new(model.config.audio.clone());
        
        // Procesar cada mensaje
        for msg in processed_messages {
            handle_osc_message_robust(model, &msg, app, &validator);
        }
    }
    
    // Verificar estado de conexión
    let stats = model.osc_server.get_stats();
    if !stats.is_connected && stats.total_received > 0 {
        use std::sync::Once;
        use std::time::Instant;
        
        static ONCE: Once = Once::new();
        static mut LAST_WARNING: Option<Instant> = None;
        
        unsafe {
            ONCE.call_once(|| {
                LAST_WARNING = Some(Instant::now());
            });
            
            if let Some(last_time) = LAST_WARNING {
                if last_time.elapsed().as_secs() > 10 {
                    println!("⚠️ OSC: Sin conexión activa (última recepción hace >5s)");
                    LAST_WARNING = Some(Instant::now());
                }
            }
        }
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

/// Maneja un mensaje OSC procesado con validación robusta
fn handle_osc_message_robust(
    model: &mut Model, 
    msg: &ProcessedOscMessage, 
    app: &App,
    validator: &OscMessageValidator
) {
    match msg.address.as_str() {
        "/note" => {
            if model.config.logging.show_osc_messages {
                println!("🎵 OSC: Nota musical recibida");
            }
            handle_note_message_robust(model, &msg.args, app, validator);
        },
        "/event" => {
            if model.config.logging.show_osc_messages {
                println!("🎹 OSC: Evento musical recibido");
            }
            handle_note_message_robust(model, &msg.args, app, validator);
        },
        "/analysis" => {
            // Solo log en modo debug para evitar spam
            handle_analysis_message(model, &msg.args);
        },
        "/drone" => {
            if model.config.logging.show_osc_messages {
                println!("🌊 OSC: Evento drone recibido");
            }
            handle_drone_message_robust(model, &msg.args, validator);
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
            handle_clear_message_robust(model);
        },
        _ => {
            if model.config.logging.show_osc_messages {
                println!("❓ OSC: Mensaje desconocido - {} de {}", msg.address, msg.source_addr);
            }
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

/// Maneja mensajes de nota con validación robusta
fn handle_note_message_robust(
    model: &mut Model, 
    args: &[osc::Type], 
    app: &App,
    validator: &OscMessageValidator
) {
    match validator.validate_note_message(args) {
        Ok((instrument, freq, amp, dur)) => {
            if model.config.logging.show_osc_messages {
                println!("🎹 Audio: {} - {:.1}Hz {:.2}amp {:.2}s [{}]", 
                       "note", freq, amp, dur, instrument);
            }
            
            // Verificar límite de eventos
            if model.notes.len() < model.config.performance.max_notes {
                model.add_note_with_instrument(freq, amp, dur, &instrument, app.window_rect());
            } else {
                println!("⚠️ OSC: Límite de notas alcanzado ({}), ignorando evento", 
                       model.config.performance.max_notes);
            }
        },
        Err(e) => {
            if model.config.logging.show_osc_messages {
                println!("⚠️ OSC: Error validando nota: {}", e);
            }
        }
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

/// Maneja mensajes de drone con validación robusta
fn handle_drone_message_robust(
    model: &mut Model, 
    args: &[osc::Type],
    validator: &OscMessageValidator
) {
    match validator.validate_drone_message(args) {
        Ok((freq, amp)) => {
            if model.config.logging.show_osc_messages {
                println!("🌊 Audio: Drone {:.1}Hz {:.2}amp", freq, amp);
            }
            
            // Verificar límite de drones
            if model.drone_events.len() < model.config.performance.max_drones {
                model.add_drone_event(freq, amp);
            } else {
                println!("⚠️ OSC: Límite de drones alcanzado ({}), ignorando evento", 
                       model.config.performance.max_drones);
            }
        },
        Err(e) => {
            if model.config.logging.show_osc_messages {
                println!("⚠️ OSC: Error validando drone: {}", e);
            }
        }
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

/// Maneja comando de limpieza con logging robusto
fn handle_clear_message_robust(model: &mut Model) {
    let before_notes = model.notes.len();
    let before_drones = model.drone_events.len();
    
    model.notes.clear();
    model.drone_events.clear();
    model.events.clear(); // Alias para compatibilidad
    
    println!("🧹 Sistema: Limpieza completa - {} notas, {} drones eliminados", 
           before_notes, before_drones);
}
