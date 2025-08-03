// src/audio.rs - Manejo de mensajes OSC con logging

use nannou::prelude::*;
use nannou_osc as osc;
use crate::model::Model;

/// Procesa todos los mensajes OSC recibidos durante un frame y actualiza el modelo en consecuencia.
/// Registra logs si está activado `verbose_osc_logging` en la configuración.
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
        if model.config.verbose_osc_logging {
            println!("📡 OSC: Procesando {} mensajes", message_count);
        }
    }
    
    // Procesar mensajes recopilados
    for msg in messages {
        handle_osc_message(model, &msg, app);
    }
    
    // Actualizar estadísticas OSC
    model.osc_stats.total_messages += message_count as u32;
    
    if message_count > 10 {
        if model.config.verbose_osc_logging {
            println!("⚠️ OSC: Pico de tráfico - {} mensajes en un frame", message_count);
        }
    }
}

/// Encaminador de mensajes OSC entrantes según su dirección. Llama a funciones especializadas según tipo.
fn handle_osc_message(model: &mut Model, msg: &osc::Message, app: &App) {
    let args = &msg.args;
    
    match msg.addr.as_str() {
        "/event" => {
            if model.config.verbose_osc_logging {
                println!("🎹 OSC: Evento musical recibido");
            }
            handle_event_message(model, args, app, "/event");
        },
        "/note" => {
            if model.config.verbose_osc_logging {
                println!("🎵 OSC: Nota musical recibida");
            }
            handle_event_message(model, args, app, "/note"); // Usar misma función
        },
        "/analysis" => {
            // Comentado para reducir spam
            handle_analysis_message(model, args);
        },
        "/drone" => {
            if model.config.verbose_osc_logging {
                println!("🌊 OSC: Evento drone recibido");
            }
            handle_drone_message(model, args);
        },
        "/cluster" => {
            if model.config.verbose_osc_logging {
                println!("🔵 OSC: Cluster recibido");
            }
            handle_cluster_message(model, args);
        },
        "/ping" => {
            if model.config.verbose_osc_logging {
                println!("🏓 OSC: Ping recibido - conexión activa");
            }
        },
        "/clear" => {
            if model.config.verbose_osc_logging {
                println!("🧹 OSC: Comando de limpieza");
            }
            handle_clear_message(model);
        },
        _ => {
            if model.config.verbose_osc_logging {
                println!("❓ OSC: Mensaje desconocido - {}", msg.addr);
            }
        }
    }
}

/// Maneja mensajes OSC del tipo `/note` o `/event`.
/// Realiza parsing, validación y actualización del modelo.
fn handle_event_message(model: &mut Model, args: &[osc::Type], app: &App, source: &str) {
    if args.len() >= 4 {
        let event_type = match args[0].clone().string() {
            Some(s) => s,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear event_type como string en {}", source);
                }
                "note".to_string()
            }
        };
        let freq = match args[1].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear freq como float en {}", source);
                }
                440.0
            }
        };
        let amp = match args[2].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear amp como float en {}", source);
                }
                0.5
            }
        };
        let dur = match args[3].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear dur como float en {}", source);
                }
                1.0
            }
        };

        // Validar rangos básicos
        let freq = freq.clamp(20.0, 20000.0);
        let amp = amp.clamp(0.0, 1.0);
        let dur = dur.clamp(0.1, 10.0);

        if model.config.verbose_osc_logging {
            println!("🎹 Audio: {} - {:.1}Hz {:.2}amp {:.2}s", event_type, freq, amp, dur);
        }
        
        // Agregar nota al modelo
        model.add_note(freq, amp, dur, app.window_rect());
    } else {
        if model.config.verbose_osc_logging {
            println!("⚠️ OSC: Evento con argumentos insuficientes: {}", args.len());
        }
    }
}

/// Maneja mensajes OSC del tipo `/analysis`.
/// Realiza parsing, validación y actualización del modelo.
fn handle_analysis_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 3 {
        let amp = match args[0].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear amp como float en /analysis");
                }
                0.0
            }
        };
        let brightness = match args[1].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear brightness como float en /analysis");
                }
                0.0
            }
        };
        let noisiness = match args[2].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear noisiness como float en /analysis");
                }
                0.0
            }
        };

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

/// Maneja mensajes OSC del tipo `/drone`.
/// Realiza parsing, validación y actualización del modelo.
fn handle_drone_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 2 {
        let freq = match args[0].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear freq como float en /drone");
                }
                220.0
            }
        };
        let amp = match args[1].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear amp como float en /drone");
                }
                0.5
            }
        };

        // Validar rangos
        let freq = freq.clamp(20.0, 20000.0);
        let amp = amp.clamp(0.0, 1.0);

        if model.config.verbose_osc_logging {
            println!("🌊 Audio: Drone {:.1}Hz {:.2}amp", freq, amp);
        }
        
        // Agregar evento drone
        model.add_drone_event(freq, amp);
    } else {
        if model.config.verbose_osc_logging {
            println!("⚠️ OSC: Drone con argumentos insuficientes: {}", args.len());
        }
    }
}

/// Maneja mensajes OSC del tipo `/cluster`.
/// Realiza parsing, validación y actualización del modelo.
fn handle_cluster_message(model: &mut Model, args: &[osc::Type]) {
    if args.len() >= 3 {
        let freq = match args[0].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear freq como float en /cluster");
                }
                300.0
            }
        };
        let amp = match args[1].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear amp como float en /cluster");
                }
                0.5
            }
        };
        let audio_level = match args[2].clone().float() {
            Some(f) => f,
            None => {
                if model.config.verbose_osc_logging {
                    println!("⚠️ OSC: No se pudo parsear audio_level como float en /cluster");
                }
                0.0
            }
        };

        // Validar rangos
        let freq = freq.clamp(20.0, 20000.0);
        let amp = amp.clamp(0.0, 1.0);
        let audio_level = audio_level.clamp(0.0, 1.0);

        if model.config.verbose_osc_logging {
            println!("🔵 Audio: Cluster {:.1}Hz {:.2}amp level={:.3}", freq, amp, audio_level);
        }
        
        // Actualizar cluster
        model.update_cluster_data(freq, amp, audio_level);
    } else {
        if model.config.verbose_osc_logging {
            println!("⚠️ OSC: Cluster con argumentos insuficientes: {}", args.len());
        }
    }
}

/// Maneja mensajes OSC del tipo `/clear`.
/// Realiza parsing, validación y actualización del modelo.
fn handle_clear_message(model: &mut Model) {
    if model.config.verbose_osc_logging {
        println!("🧹 Sistema: Limpiando todos los eventos");
    }
    model.notes.clear();
    model.drone_events.clear();
    model.events.clear(); // Alias para compatibilidad
}
