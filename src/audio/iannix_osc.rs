// =============================================================================
// RECEPTOR OSC ESTILO IANNIX - RUST SIDE
// =============================================================================
// Adaptador para recibir mensajes OSC multi-canal como IanniX

use nannou_osc::rosc::{OscMessage, OscType};
use std::collections::HashMap;
use std::time::Instant;

/// Estructura para manejar datos OSC estilo IanniX
#[derive(Debug, Clone)]
pub struct IanniXOscData {
    pub timestamp: f64,
    pub continuous: ContinuousData,
    pub triggers: Vec<TriggerEvent>,
    pub controls: HashMap<i32, f32>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ContinuousData {
    pub frequency: f32,
    pub amplitude: f32,
    pub centroid: f32,
    pub brightness: f32,
    pub freq_velocity: f32,
    pub amp_velocity: f32,
}

#[derive(Debug, Clone)]
pub struct TriggerEvent {
    pub event_type: String, // "onset", "beat", etc.
    pub intensity: f32,
    pub position: f32,
    pub timestamp: f64,
}

impl Default for ContinuousData {
    fn default() -> Self {
        Self {
            frequency: 440.0,
            amplitude: 0.0,
            centroid: 1000.0,
            brightness: 0.5,
            freq_velocity: 0.0,
            amp_velocity: 0.0,
        }
    }
}

impl Default for IanniXOscData {
    fn default() -> Self {
        Self {
            timestamp: 0.0,
            continuous: ContinuousData::default(),
            triggers: Vec::new(),
            controls: HashMap::new(),
            status: "inactive".to_string(),
        }
    }
}

/// Procesador OSC estilo IanniX
pub struct IanniXOscProcessor {
    data: IanniXOscData,
    start_time: Instant,
    message_count: u64,
    error_count: u64,
}

impl IanniXOscProcessor {
    pub fn new() -> Self {
        Self {
            data: IanniXOscData::default(),
            start_time: Instant::now(),
            message_count: 0,
            error_count: 0,
        }
    }

    /// Procesa mensajes OSC estilo IanniX
    pub fn process_message(&mut self, msg: &OscMessage) -> Result<bool, String> {
        self.message_count += 1;
        let timestamp = self.start_time.elapsed().as_secs_f64();

        match msg.addr.as_str() {
            "/continuous" => {
                self.process_continuous(msg, timestamp)
            }
            "/trigger" => {
                self.process_trigger(msg, timestamp)
            }
            "/control" => {
                self.process_control(msg, timestamp)
            }
            "/status" => {
                self.process_status(msg, timestamp)
            }
            _ => {
                self.error_count += 1;
                Err(format!("Unknown OSC address: {}", msg.addr))
            }
        }
    }

    /// Procesa datos continuos (/continuous)
    fn process_continuous(&mut self, msg: &OscMessage, timestamp: f64) -> Result<bool, String> {
        if msg.args.len() >= 5 {
            self.data.timestamp = timestamp;
            
            if let (Some(OscType::Float(freq)), 
                    Some(OscType::Float(amp)), 
                    Some(OscType::Float(centroid)), 
                    Some(OscType::Float(brightness))) = 
                    (msg.args.get(1), msg.args.get(2), msg.args.get(3), msg.args.get(4)) {
                
                self.data.continuous.frequency = freq.max(50.0).min(4000.0);
                self.data.continuous.amplitude = amp.max(0.0).min(1.0);
                self.data.continuous.centroid = centroid.max(200.0).min(8000.0);
                self.data.continuous.brightness = brightness.max(0.0).min(1.0);

                // Velocidades si están disponibles
                if msg.args.len() >= 7 {
                    if let (Some(OscType::Float(freq_vel)), Some(OscType::Float(amp_vel))) = 
                           (msg.args.get(5), msg.args.get(6)) {
                        self.data.continuous.freq_velocity = freq_vel.max(0.0).min(1.0);
                        self.data.continuous.amp_velocity = amp_vel.max(0.0).min(1.0);
                    }
                }

                return Ok(true);
            }
        }
        
        self.error_count += 1;
        Err("Invalid continuous message format".to_string())
    }

    /// Procesa eventos trigger (/trigger)
    fn process_trigger(&mut self, msg: &OscMessage, timestamp: f64) -> Result<bool, String> {
        if msg.args.len() >= 4 {
            if let (Some(OscType::String(event_type)), 
                    Some(OscType::Float(intensity)), 
                    Some(OscType::Float(position))) = 
                    (msg.args.get(1), msg.args.get(2), msg.args.get(3)) {
                
                let trigger = TriggerEvent {
                    event_type: event_type.clone(),
                    intensity: intensity.max(0.0).min(1.0),
                    position: position.max(0.0).min(1.0),
                    timestamp,
                };

                // Mantener solo los últimos 10 triggers
                self.data.triggers.push(trigger);
                if self.data.triggers.len() > 10 {
                    self.data.triggers.remove(0);
                }

                return Ok(true);
            }
        }

        self.error_count += 1;
        Err("Invalid trigger message format".to_string())
    }

    /// Procesa controles (/control)
    fn process_control(&mut self, msg: &OscMessage, _timestamp: f64) -> Result<bool, String> {
        if msg.args.len() >= 3 {
            if let (Some(OscType::Int(control_id)), 
                    Some(OscType::Float(value))) = 
                    (msg.args.get(1), msg.args.get(2)) {
                
                self.data.controls.insert(*control_id, value.max(0.0).min(1.0));
                return Ok(true);
            }
        }

        self.error_count += 1;
        Err("Invalid control message format".to_string())
    }

    /// Procesa estado (/status)
    fn process_status(&mut self, msg: &OscMessage, _timestamp: f64) -> Result<bool, String> {
        if msg.args.len() >= 3 {
            if let (Some(OscType::String(component)), 
                    Some(OscType::String(state)), 
                    Some(OscType::String(info))) = 
                    (msg.args.get(1), msg.args.get(2), msg.args.get(3)) {
                
                self.data.status = format!("{}: {} ({})", component, state, info);
                return Ok(true);
            }
        }

        self.error_count += 1;
        Err("Invalid status message format".to_string())
    }

    /// Obtiene los datos actuales
    pub fn get_data(&self) -> &IanniXOscData {
        &self.data
    }

    /// Obtiene estadísticas
    pub fn get_stats(&self) -> (u64, u64, f64) {
        let success_rate = if self.message_count > 0 {
            ((self.message_count - self.error_count) as f64 / self.message_count as f64) * 100.0
        } else {
            0.0
        };
        (self.message_count, self.error_count, success_rate)
    }

    /// Convierte a formato legacy para compatibilidad
    pub fn to_legacy_format(&self) -> (f32, f32, f32, f32, f32) {
        let data = &self.data.continuous;
        (
            data.frequency,
            data.amplitude,
            if !self.data.triggers.is_empty() { 1.0 } else { 0.0 }, // onset
            data.centroid,
            data.brightness,
        )
    }

    /// Resetea estadísticas
    pub fn reset_stats(&mut self) {
        self.message_count = 0;
        self.error_count = 0;
        self.start_time = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rosc::OscType;

    #[test]
    fn test_continuous_message() {
        let mut processor = IanniXOscProcessor::new();
        
        let msg = OscMessage {
            addr: "/continuous".to_string(),
            args: vec![
                OscType::Float(0.0),    // timestamp
                OscType::Float(440.0),  // frequency
                OscType::Float(0.5),    // amplitude
                OscType::Float(1000.0), // centroid
                OscType::Float(0.7),    // brightness
            ],
        };

        assert!(processor.process_message(&msg).is_ok());
        assert_eq!(processor.get_data().continuous.frequency, 440.0);
        assert_eq!(processor.get_data().continuous.amplitude, 0.5);
    }

    #[test]
    fn test_trigger_message() {
        let mut processor = IanniXOscProcessor::new();
        
        let msg = OscMessage {
            addr: "/trigger".to_string(),
            args: vec![
                OscType::Float(0.0),           // timestamp
                OscType::String("onset".to_string()), // type
                OscType::Float(0.8),           // intensity
                OscType::Float(0.3),           // position
            ],
        };

        assert!(processor.process_message(&msg).is_ok());
        assert_eq!(processor.get_data().triggers.len(), 1);
        assert_eq!(processor.get_data().triggers[0].event_type, "onset");
    }
}
