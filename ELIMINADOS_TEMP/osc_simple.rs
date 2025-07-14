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
        let addr = format!("0.0.0.0:{}", port);
        println!("📡 Receptor OSC activo en puerto {}", port);
        
        // Receptor OSC usando nannou_osc
        let receiver = osc::receiver(port).unwrap();
        
        loop {
            if let Ok(packet) = receiver.recv() {
                if sender.send(packet).is_err() {
                    break;
                }
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
        match msg.addr.as_str() {
            "/event" => parse_event_message(msg, window_rect),
            "/realtime_audio" => parse_realtime_message(msg),
            _ => None,
        }
    } else {
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
    let args = &msg.args;
    
    if args.len() < 7 {
        return None;
    }
    
    // Extraer tipo de evento
    let event_type_str = match &args[0] {
        osc::Type::String(s) => s.clone(),
        _ => return None,
    };
    
    // Extraer argumentos numéricos
    let mut numeric_args = Vec::new();
    for i in 1..7 {
        if let Some(arg) = args.get(i) {
            match arg {
                osc::Type::Float(f) => numeric_args.push(*f),
                osc::Type::Int(i) => numeric_args.push(*i as f32),
                _ => numeric_args.push(0.0),
            }
        }
    }
    
    if numeric_args.len() < 6 {
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
}

fn parse_realtime_message(msg: &osc::Message) -> Option<OscMessage> {
    let args = &msg.args;
    
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
}

// =================================================================
// UTILIDADES DE DEBUG
// =================================================================

pub fn log_osc_message(msg: &osc::Message, verbose: bool) {
    if verbose {
        println!("📡 OSC: {} con {} argumentos", msg.addr, msg.args.len());
    } else {
        println!("📡 OSC: {}", msg.addr);
    }
}
