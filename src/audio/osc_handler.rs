// 🎵 Manejo de mensajes OSC
// Recibe y procesa mensajes OSC desde SuperCollider

use anyhow::Context;
use nannou_osc as osc;
use std::sync::{Arc, Mutex};
use std::thread;
use super::super::musical_events::{MusicalEvent, VisualParams};
use super::super::utils::VisualizerError;
use log::{info, warn, error, debug};

/// Configuración del receptor OSC
pub struct OscHandler {
    port: u16,
}

impl OscHandler {
    /// Crea un nuevo manejador OSC
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Inicia el receptor OSC en un hilo separado
    pub fn start_receiver(&self, events: Arc<Mutex<Vec<MusicalEvent>>>) -> anyhow::Result<()> {
        let port = self.port;
        info!("🎵 Iniciando receptor OSC en puerto {}...", port);
        
        thread::spawn(move || {
            if let Err(e) = Self::run_receiver_loop(port, events) {
                error!("❌ Error crítico en receptor OSC: {}", e);
                error!("🔧 Causa: {:?}", e);
            }
        });
        
        Ok(())
    }

    /// Bucle principal del receptor OSC
    fn run_receiver_loop(port: u16, events: Arc<Mutex<Vec<MusicalEvent>>>) -> anyhow::Result<()> {
        let osc_receiver = osc::receiver(port)
            .with_context(|| format!("No se pudo crear receptor OSC en puerto {}", port))?;
        
        info!("✅ Receptor OSC iniciado correctamente en puerto {}", port);
        
        loop {
            match osc_receiver.recv() {
                Ok((packet, _addr)) => {
                    debug!("📡 Paquete OSC recibido");
                    for msg in packet.into_msgs() {
                        if let Err(e) = Self::process_osc_message(msg, &events) {
                            warn!("⚠️ Error procesando mensaje OSC: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("❌ Error recibiendo OSC: {}", e);
                    // Continuar el bucle en lugar de terminar
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }

    /// Procesa un mensaje OSC individual
    fn process_osc_message(msg: osc::Message, events: &Arc<Mutex<Vec<MusicalEvent>>>) -> anyhow::Result<()> {
        let args = msg.args.clone();
        debug!("OSC Message: {} {:?}", msg.addr, args);

        if (msg.addr == "/event" || msg.addr == "/musical_event") && args.len() >= 4 {
            match Self::parse_event_message(&args) {
                Ok(event) => {
                    let mut events_lock = events.lock()
                        .map_err(|_| VisualizerError::sync("Error obteniendo lock para añadir evento OSC"))?;
                    events_lock.push(event);
                    info!("🎵 Evento agregado! Total eventos: {}", events_lock.len());
                    Ok(())
                }
                Err(e) => {
                    error!("❌ Error parseando evento OSC: {}", e);
                    Err(e)
                }
            }
        } else {
            let error_msg = format!("Mensaje OSC incompleto o incorrecto: {} (args: {})", msg.addr, args.len());
            error!("❌ {}", error_msg);
            Err(VisualizerError::osc(error_msg).into())
        }
    }

    /// Parsea los argumentos de un mensaje de evento
    fn parse_event_message(args: &[osc::Type]) -> anyhow::Result<MusicalEvent> {
        if args.len() < 4 {
            return Err(VisualizerError::event_parsing(
                "unknown".to_string(),
                format!("Insuficientes argumentos: {} (mínimo 4)", args.len())
            ).into());
        }

        // Función auxiliar para convertir Int o Float a f32
        let to_f32 = |arg: &osc::Type| -> Option<f32> {
            match arg {
                osc::Type::Float(f) => Some(*f),
                osc::Type::Int(i) => Some(*i as f32),
                _ => None,
            }
        };

        let event_type = match args.get(0) {
            Some(osc::Type::String(s)) => s,
            _ => {
                return Err(VisualizerError::event_parsing(
                    "unknown".to_string(),
                    "Primer argumento debe ser String (tipo de evento)".to_string()
                ).into());
            }
        };

        let freq = to_f32(args.get(1).unwrap_or(&osc::Type::Float(440.0)))
            .ok_or_else(|| VisualizerError::event_parsing(
                event_type.clone(),
                "Segundo argumento debe ser número (frecuencia)".to_string()
            ))?;

        let amp = to_f32(args.get(2).unwrap_or(&osc::Type::Float(0.5)))
            .ok_or_else(|| VisualizerError::event_parsing(
                event_type.clone(),
                "Tercer argumento debe ser número (amplitud)".to_string()
            ))?;

        let dur = to_f32(args.get(3).unwrap_or(&osc::Type::Float(1.0)))
            .ok_or_else(|| VisualizerError::event_parsing(
                event_type.clone(),
                "Cuarto argumento debe ser número (duración)".to_string()
            ))?;

        info!("✅ Parseando evento: {} freq={} amp={} dur={}", event_type, freq, amp, dur);

        // Crear parámetros visuales opcionales
        let visual_params = if args.len() >= 8 {
            Self::parse_visual_params(&args[4..8])?
        } else {
            None
        };

        match event_type.as_str() {
            "point" => {
                info!("✅ Creando evento Point: freq={}, amp={}, dur={}", freq, amp, dur);
                Ok(MusicalEvent::Point {
                    time: 0.0, // Se ajustará en el hilo principal
                    freq,
                    amp,
                    duration: dur,
                    visual_params,
                })
            }
            "gliss" | "glissando" => {
                if args.len() < 5 {
                    return Err(VisualizerError::event_parsing(
                        "glissando".to_string(),
                        "Glissando requiere 5 argumentos mínimo".to_string()
                    ).into());
                }
                
                let end_freq = to_f32(args.get(4).unwrap_or(&osc::Type::Float(880.0)))
                    .ok_or_else(|| VisualizerError::event_parsing(
                        "glissando".to_string(),
                        "Frecuencia final del glissando debe ser un número".to_string()
                    ))?;

                info!("✅ Creando evento Glissando: {} -> {} Hz, amp={}, dur={}", freq, end_freq, amp, dur);
                Ok(MusicalEvent::Glissando {
                    time: 0.0,
                    start_freq: freq,
                    end_freq,
                    amp,
                    duration: dur,
                    visual_params,
                })
            }
            "cluster" => {
                if args.len() < 5 {
                    return Err(VisualizerError::event_parsing(
                        "cluster".to_string(),
                        "Cluster requiere 5 argumentos mínimo".to_string()
                    ).into());
                }
                
                let bandwidth = to_f32(args.get(4).unwrap_or(&osc::Type::Float(100.0)))
                    .ok_or_else(|| VisualizerError::event_parsing(
                        "cluster".to_string(),
                        "Bandwidth del cluster debe ser un número".to_string()
                    ))?;

                info!("✅ Creando evento Cluster: center={} Hz, bandwidth={}, amp={}, dur={}", freq, bandwidth, amp, dur);
                Ok(MusicalEvent::Cluster {
                    time: 0.0,
                    center_freq: freq,
                    bandwidth,
                    amp,
                    duration: dur,
                    visual_params,
                })
            }
            "noise" => {
                if args.len() < 5 {
                    return Err(VisualizerError::event_parsing(
                        "noise".to_string(),
                        "Noise requiere 5 argumentos mínimo".to_string()
                    ).into());
                }
                
                let bandwidth = to_f32(args.get(4).unwrap_or(&osc::Type::Float(200.0)))
                    .ok_or_else(|| VisualizerError::event_parsing(
                        "noise".to_string(),
                        "Bandwidth del noise debe ser un número".to_string()
                    ))?;

                info!("✅ Creando evento Noise: center={} Hz, bandwidth={}, amp={}, dur={}", freq, bandwidth, amp, dur);
                Ok(MusicalEvent::Noise {
                    time: 0.0,
                    center_freq: freq,
                    bandwidth,
                    amp,
                    duration: dur,
                    visual_params,
                })
            }
            _ => {
                Err(VisualizerError::event_parsing(
                    event_type.to_string(),
                    "Tipo de evento desconocido".to_string()
                ).into())
            }
        }
    }

    /// Parsea los parámetros visuales opcionales
    fn parse_visual_params(args: &[osc::Type]) -> anyhow::Result<Option<VisualParams>> {
        if args.len() < 4 {
            return Ok(None);
        }

        match (args.get(0), args.get(1), args.get(2), args.get(3)) {
            (Some(osc::Type::Float(timbre)), Some(osc::Type::Float(spread)), 
             Some(osc::Type::Float(hue)), Some(osc::Type::Float(intensity))) => {
                Ok(Some(VisualParams {
                    timbre: *timbre,
                    spatial_spread: *spread,
                    color_hue: *hue,
                    intensity: *intensity,
                }))
            }
            _ => {
                warn!("⚠️ Parámetros visuales con tipos incorrectos, usando defaults");
                Ok(None)
            }
        }
    }
}