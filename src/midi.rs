// src/midi.rs - Soporte básico MIDI para SC Score Visualizer
// Permite usar hardware MIDI para pruebas rápidas sin SuperCollider

use midir::{MidiInput, MidiInputConnection, Ignore};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver};
use crate::config::AppConfig;

/// Nota MIDI procesada para visualización
#[derive(Debug, Clone)]
pub struct MidiNote {
    pub freq: f32,
    pub amp: f32,
    pub dur: f32,
    pub instrument: String,
    pub velocity: u8,
    pub channel: u8,
}

/// Mensaje MIDI procesado
#[derive(Debug, Clone)]
pub struct MidiMessage {
    pub note: u8,
    pub velocity: u8,
    pub channel: u8,
    pub message_type: MidiMessageType,
}

#[derive(Debug, Clone)]
pub enum MidiMessageType {
    NoteOn,
    NoteOff,
    ControlChange { controller: u8, value: u8 },
}

/// Conversor de notas MIDI a frecuencias
pub struct MidiToFreq;

impl MidiToFreq {
    /// Convierte número de nota MIDI a frecuencia en Hz
    /// Nota 69 (A4) = 440 Hz
    pub fn midi_to_freq(midi_note: u8) -> f32 {
        440.0 * 2.0_f32.powf((midi_note as f32 - 69.0) / 12.0)
    }
    
    /// Convierte velocidad MIDI (0-127) a amplitud (0.0-1.0)
    pub fn velocity_to_amp(velocity: u8) -> f32 {
        (velocity as f32) / 127.0
    }
    
    /// Mapea canal MIDI a tipo de instrumento
    pub fn channel_to_instrument(channel: u8) -> &'static str {
        match channel {
            1 => "piano",
            2 => "sine", 
            3 => "triangle",
            4 => "square",
            5 => "sawtooth",
            6 => "bell",
            7 => "pad",
            8 => "lead",
            9 => "drums", // Canal 10 en especificación MIDI (indexado desde 1)
            _ => "sine", // Por defecto
        }
    }
}

/// Servidor MIDI que escucha dispositivos MIDI conectados
pub struct MidiServer {
    _connection: Option<MidiInputConnection<()>>,
    message_receiver: Receiver<MidiMessage>,
    config: crate::config::AppConfig,
    is_active: Arc<Mutex<bool>>,
}

impl MidiServer {
    /// Crea un nuevo servidor MIDI
    pub fn new(config: AppConfig) -> Result<Self, Box<dyn Error>> {
        println!("🎹 Iniciando servidor MIDI...");
        
        let mut midi_in = MidiInput::new("SC Score Visualizer MIDI Input")?;
        
        // Configurar qué tipos de mensajes MIDI ignorar
        midi_in.ignore(Ignore::None);
        
        // Listar puertos MIDI disponibles
        let in_ports = midi_in.ports();
        println!("📋 Puertos MIDI disponibles:");
        for (i, port) in in_ports.iter().enumerate() {
            match midi_in.port_name(port) {
                Ok(name) => println!("  {}. {}", i, name),
                Err(_) => println!("  {}. Puerto sin nombre", i),
            }
        }
        
        if in_ports.is_empty() {
            println!("⚠️  No se encontraron dispositivos MIDI");
            let (_sender, receiver) = mpsc::channel();
            return Ok(Self {
                _connection: None,
                message_receiver: receiver,
                config,
                is_active: Arc::new(Mutex::new(false)),
            });
        }
        
        // Crear canal para comunicación entre threads
        let (sender, receiver) = mpsc::channel::<MidiMessage>();
        let is_active = Arc::new(Mutex::new(true));
        let is_active_clone = Arc::clone(&is_active);
        
        // Usar el primer puerto disponible
        let in_port = &in_ports[0];
        let port_name = midi_in.port_name(in_port)
            .unwrap_or_else(|_| "Puerto MIDI".to_string());
        
        println!("🔌 Conectando a: {}", port_name);
        
        // Callback para procesar mensajes MIDI
        let connection = midi_in.connect(in_port, "sc-visualizer-input", 
            move |_timestamp, message, _| {
                if let Ok(is_active) = is_active_clone.lock() {
                    if !*is_active {
                        return;
                    }
                }
                
                if let Some(midi_msg) = Self::parse_midi_message(message) {
                    if let Err(_e) = sender.send(midi_msg) {
                        // Error enviando mensaje MIDI - no imprimir para evitar spam
                    }
                }
            }, 
            ()
        )?;
        
        println!("✅ Servidor MIDI iniciado exitosamente");
        println!("🎵 Esperando mensajes MIDI desde: {}", port_name);
        
        Ok(Self {
            _connection: Some(connection),
            message_receiver: receiver,
            config,
            is_active,
        })
    }
    
    /// Procesa mensajes MIDI raw y los convierte a MidiMessage
    fn parse_midi_message(message: &[u8]) -> Option<MidiMessage> {
        if message.len() < 3 {
            return None;
        }
        
        let status = message[0];
        let channel = status & 0x0F;
        let command = status & 0xF0;
        
        match command {
            0x90 => { // Note On
                let note = message[1];
                let velocity = message[2];
                
                // Velocity 0 en Note On es equivalente a Note Off
                let message_type = if velocity > 0 {
                    MidiMessageType::NoteOn
                } else {
                    MidiMessageType::NoteOff
                };
                
                Some(MidiMessage {
                    note,
                    velocity,
                    channel,
                    message_type,
                })
            },
            0x80 => { // Note Off
                let note = message[1];
                let velocity = message[2];
                
                Some(MidiMessage {
                    note,
                    velocity,
                    channel,
                    message_type: MidiMessageType::NoteOff,
                })
            },
            0xB0 => { // Control Change
                let controller = message[1];
                let value = message[2];
                
                Some(MidiMessage {
                    note: 0,
                    velocity: 0,
                    channel,
                    message_type: MidiMessageType::ControlChange { controller, value },
                })
            },
            _ => None, // Ignorar otros tipos de mensajes por ahora
        }
    }
    
    /// Obtiene mensajes MIDI pendientes y los convierte a MidiNotes
    pub fn get_notes(&self) -> Vec<MidiNote> {
        let mut notes = Vec::new();
        
        // Procesar todos los mensajes disponibles
        while let Ok(midi_msg) = self.message_receiver.try_recv() {
            match midi_msg.message_type {
                MidiMessageType::NoteOn => {
                    let freq = MidiToFreq::midi_to_freq(midi_msg.note);
                    let amp = MidiToFreq::velocity_to_amp(midi_msg.velocity);
                    let instrument = MidiToFreq::channel_to_instrument(midi_msg.channel + 1); // MIDI es 0-indexed, pero usamos 1-indexed
                    
                    // Duración fija para notas MIDI (configurable)
                    let duration = 1.0; // 1 segundo por defecto
                    
                    println!("🎹 MIDI Note ON: Canal={}, Nota={}, Vel={}, Freq={:.1}Hz", 
                        midi_msg.channel + 1, midi_msg.note, midi_msg.velocity, freq);
                    
                    let note = MidiNote {
                        freq,
                        amp,
                        dur: duration,
                        instrument: instrument.to_string(),
                        velocity: midi_msg.velocity,
                        channel: midi_msg.channel,
                    };
                    
                    notes.push(note);
                },
                MidiMessageType::NoteOff => {
                    // Por ahora no manejamos Note Off específicamente
                    // Las notas se desvanecen automáticamente
                    println!("🎹 MIDI Note OFF: Canal={}, Nota={}", 
                        midi_msg.channel + 1, midi_msg.note);
                },
                MidiMessageType::ControlChange { controller, value } => {
                    // Manejar control changes en el futuro (modulación, etc.)
                    println!("🎛️  MIDI CC: Canal={}, Controller={}, Value={}", 
                        midi_msg.channel + 1, controller, value);
                },
            }
        }
        
        notes
    }
    
    /// Verifica si el servidor MIDI está activo
    pub fn is_active(&self) -> bool {
        self.is_active.lock().map(|guard| *guard).unwrap_or(false)
    }
    
    /// Detiene el servidor MIDI
    pub fn stop(&self) {
        if let Ok(mut is_active) = self.is_active.lock() {
            *is_active = false;
        }
        println!("🛑 Servidor MIDI detenido");
    }
    
    /// Obtiene estadísticas básicas del servidor MIDI
    pub fn get_stats(&self) -> MidiServerStats {
        MidiServerStats {
            is_connected: self._connection.is_some(),
            messages_available: 0, // No podemos obtener el tamaño del channel fácilmente
        }
    }
}

/// Estadísticas del servidor MIDI
#[derive(Debug)]
pub struct MidiServerStats {
    pub is_connected: bool,
    pub messages_available: usize,
}

impl Drop for MidiServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Pruebas y funciones de utilidad para MIDI
pub mod midi_utils {
    
    /// Convierte nombre de nota musical a número MIDI
    pub fn note_name_to_midi(note_name: &str) -> Option<u8> {
        let note_map = [
            ("C", 0), ("C#", 1), ("Db", 1), ("D", 2), ("D#", 3), ("Eb", 3),
            ("E", 4), ("F", 5), ("F#", 6), ("Gb", 6), ("G", 7), ("G#", 8),
            ("Ab", 8), ("A", 9), ("A#", 10), ("Bb", 10), ("B", 11)
        ];
        
        // Parsear nota y octava (ej: "C4", "A#3")
        if note_name.len() < 2 {
            return None;
        }
        
        let (note_part, octave_part) = if note_name.contains('#') || note_name.contains('b') {
            (&note_name[..2], &note_name[2..])
        } else {
            (&note_name[..1], &note_name[1..])
        };
        
        let octave: i8 = octave_part.parse().ok()?;
        
        for (name, semitone) in note_map.iter() {
            if *name == note_part {
                let midi_note = (octave + 1) * 12 + *semitone;
                if midi_note >= 0 && midi_note <= 127 {
                    return Some(midi_note as u8);
                }
            }
        }
        
        None
    }
    
    /// Convierte número MIDI a nombre de nota
    pub fn midi_to_note_name(midi_note: u8) -> String {
        let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let octave = (midi_note / 12) as i8 - 1;
        let note = note_names[(midi_note % 12) as usize];
        format!("{}{}", note, octave)
    }
}

/// Función para probar el sistema MIDI
pub fn test_midi_system() -> Result<(), Box<dyn Error>> {
    println!("🧪 Probando sistema MIDI...");
    
    // Crear configuración temporal
    let config = crate::config::AppConfig::load_or_default("config.toml");
    
    // Intentar crear servidor MIDI
    match MidiServer::new(config) {
        Ok(server) => {
            println!("✅ Servidor MIDI creado exitosamente");
            
            // Mostrar estadísticas
            let stats = server.get_stats();
            println!("📊 Estado: Conectado={}, Mensajes={}", 
                stats.is_connected, stats.messages_available);
            
            // Probar conversiones
            println!("🔄 Probando conversiones:");
            println!("  MIDI 60 (C4) = {:.1} Hz", MidiToFreq::midi_to_freq(60));
            println!("  MIDI 69 (A4) = {:.1} Hz", MidiToFreq::midi_to_freq(69));
            println!("  Velocity 127 = {:.2} amp", MidiToFreq::velocity_to_amp(127));
            
            Ok(())
        },
        Err(e) => {
            println!("⚠️  No se pudo inicializar MIDI: {}", e);
            println!("   Esto es normal si no hay dispositivos MIDI conectados");
            Ok(())
        }
    }
}
