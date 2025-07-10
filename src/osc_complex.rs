// =================================================================
// 🎵 SC SCORE VISUALIZER - COMUNICACIÓN OSC
// =================================================================
// Lógica para recibir y parsear mensajes OSC

use nannou_osc as osc;
use nannou_osc::Packet;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::net::UdpSocket;

use crate::events::{MusicalEvent, RealtimeData, EventType};
use crate::model::Config;

// =================================================================
// INICIALIZACIÓN DEL RECEPTOR OSC
// =================================================================

pub fn setup_osc_receiver(port: u16) -> Result<Receiver<Packet>, Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel();
    
    thread::spawn(move || {
        let addr = format!("0.0.0.0:{}", port);
        
        match UdpSocket::bind(&addr) {
            Ok(socket) => {
                println!("📡 Receptor OSC activo en puerto {}", port);
                
                let mut buf = [0u8; 1024];
                
                loop {
                    match socket.recv_from(&mut buf) {
                        Ok((size, _addr)) => {
                            match osc::decoder::decode_udp(&buf[..size]) {
                                Ok(packet) => {
                                    if sender.send(packet).is_err() {
                                        eprintln!("⚠️ Error enviando paquete OSC al canal");
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("⚠️ Error decodificando OSC: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠️ Error recibiendo OSC: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error creando socket OSC en puerto {}: {}", port, e);
            }
        }
    });
    
    Ok(receiver)
}

// =================================================================
// PROCESAMIENTO DE MENSAJES OSC
// =================================================================

pub fn process_osc_packet(packet: &Packet, window_rect: nannou::geom::Rect) -> Option<OscMessage> {
    match packet {
        Packet::Message(msg) => {
            match msg.addr.as_str() {
                "/event" => parse_event_message(msg, window_rect),
                "/realtime_audio" => parse_realtime_message(msg),
                _ => {
                    // Ignorar mensajes desconocidos silenciosamente
                    None
                }
            }
        }
        Packet::Bundle(_) => {
            // Los bundles no son comunes en nuestro uso
            None
        }
    }
}

#[derive(Debug)]
pub enum OscMessage {
    MusicalEvent(MusicalEvent),
    RealtimeData(RealtimeData),
}

// =================================================================
// PARSEO DE MENSAJES ESPECÍFICOS
// =================================================================

fn parse_event_message(msg: &nannou_osc::Message, window_rect: nannou::geom::Rect) -> Option<OscMessage> {
    if msg.args.len() < 7 {
        eprintln!("⚠️ Mensaje /event incompleto: esperaba 7 argumentos, recibió {}", msg.args.len());
        return None;
    }
    
    // Extraer argumentos
    let event_type_str = match &msg.args[0] {
        nannou_osc::Type::String(s) => s.clone(),
        _ => {
            eprintln!("⚠️ Primer argumento de /event debe ser string");
            return None;
        }
    };
    
    let args: Vec<f32> = msg.args[1..7].iter()
        .filter_map(|arg| match arg {
            nannou_osc::Type::Float(f) => Some(*f),
            nannou_osc::Type::Int(i) => Some(*i as f32),
            _ => None,
        })
        .collect();
    
    if args.len() != 6 {
        eprintln!("⚠️ Argumentos numéricos incorrectos en /event");
        return None;
    }
    
    // Determinar tipo de evento
    let event_type = match event_type_str.as_str() {
        "point" | "pbind" | "synth" | "proxy" | "test" => EventType::Point,
        "glissando" => EventType::Glissando,
        "texture" => EventType::Texture,
        "rhythm" => EventType::Rhythm,
        "harmony" => EventType::Harmony,
        _ => EventType::Point, // Por defecto
    };
    
    // Crear evento
    let event = MusicalEvent::new(
        event_type,
        args[0], // frequency
        args[1], // amplitude
        args[2], // duration
        args[3], // timbre
        args[4], // curvature
        args[5], // cf
        window_rect,
    );
    
    Some(OscMessage::MusicalEvent(event))
}

fn parse_realtime_message(msg: &nannou_osc::Message) -> Option<OscMessage> {
    if msg.args.len() < 5 {
        return None;
    }
    
    let args: Vec<f32> = msg.args[0..5].iter()
        .filter_map(|arg| match arg {
            nannou_osc::Type::Float(f) => Some(*f),
            nannou_osc::Type::Int(i) => Some(*i as f32),
            _ => None,
        })
        .collect();
    
    if args.len() != 5 {
        return None;
    }
    
    let data = RealtimeData::new(
        args[0], // frequency
        args[1], // amplitude
        args[2], // spectral_centroid
        args[3], // spectral_flatness
        args[4] > 0.5, // onset_detected
    );
    
    Some(OscMessage::RealtimeData(data))
}

// =================================================================
// UTILIDADES DE DEBUG
// =================================================================

pub fn log_osc_message(msg: &nannou_osc::Message, verbose: bool) {
    if verbose {
        println!("📡 OSC: {} con {} argumentos", msg.addr, msg.args.len());
        for (i, arg) in msg.args.iter().enumerate() {
            match arg {
                nannou_osc::Type::String(s) => println!("  [{}] String: {}", i, s),
                nannou_osc::Type::Float(f) => println!("  [{}] Float: {:.3}", i, f),
                nannou_osc::Type::Int(i) => println!("  [{}] Int: {}", i, i),
                _ => println!("  [{}] Other: {:?}", i, arg),
            }
        }
    } else {
        println!("📡 OSC: {}", msg.addr);
    }
}
