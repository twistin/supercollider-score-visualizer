// =================================================================
// 🎵 SC SCORE VISUALIZER - COMUNICACIÓN OSC SIMPLIFICADA
// =================================================================

use nannou_osc as osc;
use osc::Packet;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use crate::events::{MusicalEvent, RealtimeData, EventType};

// =================================================================
// INICIALIZACIÓN DEL RECEPTOR OSC
// =================================================================

pub fn setup_osc_receiver(port: u16) -> Result<Receiver<Packet>, Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let _addr = format!("0.0.0.0:{}", port);
        println!("📡 Receptor OSC activo en puerto {}", port);
        
        // Receptor OSC usando nannou_osc
        let osc_receiver = osc::receiver(port).unwrap();
        
        for (packet, _addr) in osc_receiver.iter() {
            if sender.send(packet).is_err() {
                break;
            }
        }
    });
    
    Ok(receiver)
}

// =================================================================
// PROCESAMIENTO DE MENSAJES OSC
// =================================================================

pub fn process_osc_packet(packet: &Packet, window_rect: nannou::geom::Rect) -> Option<OscMessage> {
    if let Packet::Message(msg) = packet {
        println!("🔍 Procesando mensaje OSC: {}", msg.addr);
        match msg.addr.as_str() {
            "/event" => {
                println!("🎵 Procesando /event");
                parse_event_message(msg, window_rect)
            },
            "/realtime_audio" => {
                println!("📊 Procesando /realtime_audio");
                parse_realtime_message(msg)
            },
            "/test" => {
                println!("🧪 Mensaje de prueba recibido");
                None
            },
            _ => {
                println!("❓ Mensaje OSC desconocido: {}", msg.addr);
                None
            }
        }
    } else {
        println!("⚠️ Paquete OSC no es un mensaje");
        None
    }
}

#[derive(Debug)]
pub enum OscMessage {
    MusicalEvent(MusicalEvent),
    RealtimeData(RealtimeData),
}

// =================================================================
// PARSEO SIMPLIFICADO DE MENSAJES
// =================================================================

fn parse_event_message(msg: &osc::Message, window_rect: nannou::geom::Rect) -> Option<OscMessage> {
    if let Some(args) = &msg.args {
        println!("🔍 Parseando /event con {} argumentos", args.len());
        
        if args.len() < 7 {
            println!("❌ /event: Insuficientes argumentos (necesarios 7, recibidos {})", args.len());
            return None;
        }
    
        // Extraer tipo de evento
        let event_type_str = match args.get(0) {
            Some(osc::Type::String(s)) => {
                println!("📝 Tipo de evento: {}", s);
                s.clone()
            },
            _ => {
                println!("❌ /event: Primer argumento no es string");
                return None;
            }
        };
        
        // Extraer argumentos numéricos
        let mut numeric_args = Vec::new();
        for i in 1..7 {
            if let Some(arg) = args.get(i) {
                match arg {
                    osc::Type::Float(f) => {
                        println!("📊 Arg {}: {} (float)", i, f);
                        numeric_args.push(*f);
                    },
                    osc::Type::Int(i_val) => {
                        println!("📊 Arg {}: {} (int->float)", i, i_val);
                        numeric_args.push(*i_val as f32);
                    },
                    _ => {
                        println!("⚠️ Arg {}: tipo no soportado, usando 0.0", i);
                        numeric_args.push(0.0);
                    }
                }
            }
        }
        
        if numeric_args.len() < 6 {
            println!("❌ /event: Insuficientes args numéricos");
            return None;
        }
        
        // Determinar tipo de evento
        let event_type = match event_type_str.as_str() {
            "glissando" => EventType::Glissando,
            "texture" => EventType::Texture,
            "rhythm" => EventType::Rhythm,
            "harmony" => EventType::Harmony,
            _ => EventType::Point,
        };
        
        // Crear evento
        let event = MusicalEvent::new(
            event_type,
            numeric_args[0], // frequency
            numeric_args[1], // amplitude
            numeric_args[2], // duration
            numeric_args[3], // timbre
            numeric_args[4], // curvature
            numeric_args[5], // cf
            window_rect,
        );
    
        Some(OscMessage::MusicalEvent(event))
    } else {
        None
    }
}

fn parse_realtime_message(msg: &osc::Message) -> Option<OscMessage> {
    if let Some(args) = &msg.args {
        if args.len() < 5 {
            return None;
        }
        
        let mut numeric_args = Vec::new();
        for i in 0..5 {
            if let Some(arg) = args.get(i) {
                match arg {
                    osc::Type::Float(f) => numeric_args.push(*f),
                    osc::Type::Int(i) => numeric_args.push(*i as f32),
                    _ => numeric_args.push(0.0),
                }
            }
        }
        
        if numeric_args.len() < 5 {
            return None;
        }
        
        let data = RealtimeData::new(
            numeric_args[0], // frequency
            numeric_args[1], // amplitude
            numeric_args[2], // spectral_centroid
            numeric_args[3], // spectral_flatness
            numeric_args[4] > 0.5, // onset_detected
        );
    
        Some(OscMessage::RealtimeData(data))
    } else {
        None
    }
}

// =================================================================
// UTILIDADES DE DEBUG
// =================================================================

pub fn log_osc_message(msg: &osc::Message, verbose: bool) {
    if verbose {
        if let Some(args) = &msg.args {
            println!("📡 OSC: {} con {} argumentos", msg.addr, args.len());
        }
    } else {
        println!("📡 OSC: {}", msg.addr);
    }
}
