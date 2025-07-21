use nannou_osc as osc;
use std::sync::{mpsc::{self, TryRecvError, Receiver}, Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use crate::config::{OscConfig, AudioConfig};
use crate::errors::{VisualizerError, VisualizerResult};
use crate::logging::Logger;
use crate::events::{MusicalEvent, RealtimeData}; // Asegúrate de importar MusicalEvent y RealtimeData

/// Estructura para mensajes OSC procesados, útil para el hilo principal.
#[derive(Debug, Clone)]
pub struct ProcessedOscMessage {
    pub addr: String,
    pub args: Vec<osc::Type>,
    pub timestamp: Instant,
    pub source_addr: String,
}

/// Estadísticas del servidor OSC.
#[derive(Debug, Clone)]
pub struct OscServerStats {
    pub total_received: u64,
    pub total_processed: u64,
    pub failed_messages: u32,
    pub last_message_time: f64,
    pub messages_per_second: f64,
    pub is_connected: bool,
}

impl Default for OscServerStats {
    fn default() -> Self {
        OscServerStats {
            total_received: 0,
            total_processed: 0,
            failed_messages: 0,
            last_message_time: 0.0,
            messages_per_second: 0.0,
            is_connected: false,
        }
    }
}

/// El servidor OSC que escucha en un hilo separado.
#[derive(Debug)]
pub struct OscServer {
    pub config: OscConfig,
    audio_config: AudioConfig,
    sender: mpsc::Sender<ProcessedOscMessage>,
    /// Este receiver no se utiliza; el real se devuelve desde `OscServer::new()`.
    receiver: mpsc::Receiver<ProcessedOscMessage>,
    stats_sender: mpsc::Sender<OscServerStats>,
    stats_receiver: mpsc::Receiver<OscServerStats>,
    is_running: bool,
    last_stats: OscServerStats,
}

impl OscServer {
    /// Crea una nueva instancia del servidor OSC.
    pub fn new(config: OscConfig, audio_config: AudioConfig) -> VisualizerResult<(Arc<Mutex<OscServer>>, Receiver<ProcessedOscMessage>)> {
        // Canal principal para mensajes procesados
        let (sender, receiver) = mpsc::channel();
        let (stats_sender, stats_receiver) = mpsc::channel();

        // El receiver real se entrega al main, el struct usa un receiver dummy que nunca se usa
        let dummy_receiver = mpsc::channel().1;
        let server = OscServer {
            config: config.clone(),
            audio_config: audio_config.clone(),
            sender: sender.clone(),
            receiver: dummy_receiver,
            stats_sender: stats_sender.clone(),
            stats_receiver,
            is_running: false,
            last_stats: OscServerStats::default(),
        };

        let server_arc = Arc::new(Mutex::new(server));

        {
            let mut guard = server_arc.lock().unwrap();
            guard.start()?;
            guard.self_test()?;
        }

        Ok((server_arc, receiver))
    }

    /// Permite acceder al Receiver interno de mensajes OSC procesados.
    pub fn receiver(&self) -> &Receiver<ProcessedOscMessage> {
        &self.receiver
    }

    /// Inicia el servidor OSC en un hilo separado.
    fn start(&mut self) -> VisualizerResult<()> {
        if self.is_running {
            Logger::log_info("El servidor OSC ya está corriendo.");
            return Ok(());
        }

        let listen_port = self.config.listen_port;
        let sender_clone = self.sender.clone();
        let stats_sender_clone = self.stats_sender.clone();
        let _audio_config_clone = self.audio_config.clone();

        Logger::log_info(&format!("🔧 Preparando para iniciar OscServer en 127.0.0.1:{}", listen_port));

        thread::spawn(move || {
            Logger::log_info(&format!("🔧 Iniciando servidor OSC robusto (hilo dedicado + flume bounded)..."));
            Logger::log_info(&format!("🔧 Configuración: 127.0.0.1:{}", listen_port));
            Logger::log_info(&format!("🔧 Buffer: {} mensajes, timeout: {}ms", 1024, 10));

            let receiver = match osc::receiver(listen_port) {
                Ok(r) => r,
                Err(e) => {
                    Logger::log_error(&format!("❌ Error al enlazar el puerto OSC {}: {}", listen_port, e));
                    let mut error_stats = OscServerStats::default();
                    error_stats.is_connected = false;
                    error_stats.failed_messages = 9999;
                    let _ = stats_sender_clone.send(error_stats);
                    return;
                }
            };

            Logger::log_info(&format!("🔧 Servidor OSC iniciado exitosamente (hilo dedicado)"));
            Logger::log_info(&format!("🔧 Esperando mensajes en 127.0.0.1:{}", listen_port));

            let mut current_stats = OscServerStats::default();
            let mut last_stats_update = Instant::now();
            let mut msg_count_since_last_update = 0;
            current_stats.is_connected = true;

            for (packet, addr) in receiver.iter() {
                current_stats.total_received += 1;
                msg_count_since_last_update += 1;
                let start_time = Instant::now();

                for msg in packet.into_msgs() {
                    Logger::log_info(&format!("🎵 OSC recibido: {} {:?}", msg.addr, msg.args));
                    let processed_msg = ProcessedOscMessage {
                        addr: msg.addr.clone(),
                        args: msg.args.clone(),
                        timestamp: Instant::now(),
                        source_addr: addr.to_string(),
                    };

                    // Enviar todos los mensajes procesados al canal para que main.rs los reciba
                    if let Err(e) = sender_clone.send(processed_msg) {
                        Logger::log_error(&format!("❌ Error al enviar mensaje OSC al hilo principal: {}", e));
                        current_stats.failed_messages += 1;
                    } else {
                        current_stats.total_processed += 1;
                    }
                }

                if last_stats_update.elapsed() >= Duration::from_secs(1) {
                    current_stats.messages_per_second = msg_count_since_last_update as f64 / last_stats_update.elapsed().as_secs_f64();
                    current_stats.last_message_time = start_time.elapsed().as_secs_f64();
                    if let Err(e) = stats_sender_clone.send(current_stats.clone()) {
                        Logger::log_error(&format!("❌ Error al enviar estadísticas OSC: {}", e));
                    }
                    last_stats_update = Instant::now();
                    msg_count_since_last_update = 0;
                }
            }
            current_stats.is_connected = false;
            let _ = stats_sender_clone.send(current_stats);
        });

        self.is_running = true;
        Logger::log_info("🔧 Servidor OSC iniciado exitosamente.");
        Ok(())
    }

    /// Recibe mensajes procesados del hilo del servidor OSC.
    pub fn try_recv(&self) -> Result<ProcessedOscMessage, TryRecvError> {
        self.receiver.try_recv()
    }

    /// Obtiene las últimas estadísticas del servidor OSC.
    pub fn get_stats(&mut self) -> OscServerStats {
        while let Ok(stats) = self.stats_receiver.try_recv() {
            self.last_stats = stats;
        }
        self.last_stats.clone()
    }

    /// Realiza un auto-test enviando un mensaje OSC a sí mismo.
    pub fn self_test(&self) -> VisualizerResult<()> {
        Logger::log_info("🔧 Realizando auto-test OSC...");
        let test_port = self.config.listen_port;
        let test_addr = format!("{}:{}", self.config.listen_host, test_port);

        let sender = osc::sender()?;
        let receiver = osc::receiver(test_port)?;

        let test_message = osc::Message {
            addr: "/test".to_string(),
            args: vec![osc::Type::Float(1.0)],
        };
        sender.send(test_message, test_addr)?;

        let start_time = Instant::now();
        let timeout = Duration::from_secs(1);
        let mut received = false;

        while start_time.elapsed() < timeout {
            if let Ok(Some((packet, _addr))) = receiver.try_recv() {
                for msg in packet.into_msgs() {
                    if msg.addr == "/test" {
                        Logger::log_info(&format!("🔧 Auto-test OSC exitoso - Puerto {} accesible", test_port));
                        received = true;
                        break;
                    }
                    // Si recibimos otros mensajes durante el test, también los logueamos
                    Logger::log_debug(&format!("DEBUG: Auto-test recibió mensaje inesperado: {}", msg.addr));
                }
            }
            if received { break; }
            thread::sleep(Duration::from_millis(10));
        }

        if received {
            Ok(())
        } else {
            Err(VisualizerError::OscConnectionError {
                message: format!("Fallo el auto-test: no se recibió el mensaje de prueba en el puerto {}", test_port),
            })
        }
    }


    // Métodos de validación de argumentos OSC (se mantienen, pero setup_osc_receiver los usa directamente)
    pub fn validate_note_args(&self, args: &[osc::Type]) -> VisualizerResult<(f32, f32, f32)> {
        // Esta función podría ser privada o eliminada si setup_osc_receiver es el único punto de validación
        // Por ahora, se mantiene para compatibilidad si otras partes del código la usan.
        Logger::log_debug(&format!("DEBUG: Validación de nota: {} argumentos recibidos", args.len()));
        if args.len() != 3 {
            return Err(VisualizerError::ValidationError {
                field: "número de argumentos".to_string(),
                expected: "3".to_string(),
                actual: format!("{}", args.len()),
                details: "Se esperaban 3 argumentos (frecuencia, amplitud, duración) para /note_on".to_string(),
            });
        }

        let freq = match args[0] {
            osc::Type::Float(f) => f,
            osc::Type::Int(i) => i as f32,
            _ => return Err(VisualizerError::ValidationError {
                field: "frecuencia".to_string(),
                expected: "Float o Int".to_string(),
                actual: format!("{:?}", args[0]),
                details: "El primer argumento debe ser un número (frecuencia)".to_string(),
            }),
        };

        let amp = match args[1] {
            osc::Type::Float(f) => f,
            _ => return Err(VisualizerError::ValidationError {
                field: "amplitud".to_string(),
                expected: "Float".to_string(),
                actual: format!("{:?}", args[1]),
                details: "El segundo argumento debe ser un flotante (amplitud)".to_string(),
            }),
        };

        let dur = match args[2] {
            osc::Type::Float(f) => f,
            _ => return Err(VisualizerError::ValidationError {
                field: "duración".to_string(),
                expected: "Float".to_string(),
                actual: format!("{:?}", args[2]),
                details: "El tercer argumento debe ser un flotante (duración)".to_string(),
            }),
        };

        if freq < self.audio_config.freq_min || freq > self.audio_config.freq_max {
            return Err(VisualizerError::ValidationError {
                field: "frecuencia".to_string(),
                expected: format!("entre {} y {}", self.audio_config.freq_min, self.audio_config.freq_max),
                actual: format!("{}", freq),
                details: "Frecuencia fuera de rango válido".to_string(),
            });
        }
        if amp < self.audio_config.amp_min || amp > self.audio_config.amp_max {
            return Err(VisualizerError::ValidationError {
                field: "amplitud".to_string(),
                expected: format!("entre {} y {}", self.audio_config.amp_min, self.audio_config.amp_max),
                actual: format!("{}", amp),
                details: "Amplitud fuera de rango válido".to_string(),
            });
        }
        if dur < self.audio_config.dur_min || dur > self.audio_config.dur_max {
            return Err(VisualizerError::ValidationError {
                field: "duración".to_string(),
                expected: format!("entre {} y {}", self.audio_config.dur_min, self.audio_config.dur_max),
                actual: format!("{}", dur),
                details: "Duración fuera de rango válido".to_string(),
            });
        }

        Logger::log_debug(&format!("DEBUG: Parseado: instrument=default, freq={:.2}, amp={:.3}, dur={:.2}", freq, amp, dur));
        Logger::log_debug("DEBUG: Validación exitosa: nota aceptada");
        Ok((freq, amp, dur))
    }

    pub fn validate_drone_args(&self, args: &[osc::Type]) -> VisualizerResult<(f32, f32, f32)> {
        // Esta función también podría ser privada o eliminada.
        if args.len() != 3 {
            return Err(VisualizerError::ValidationError {
                field: "número de argumentos".to_string(),
                expected: "3".to_string(),
                actual: format!("{}", args.len()),
                details: "Se esperaban 3 argumentos (frecuencia, amplitud, duración) para /drone_on".to_string(),
            });
        }

        let freq = match args[0] {
            osc::Type::Float(f) => f,
            osc::Type::Int(i) => i as f32,
            _ => return Err(VisualizerError::ValidationError {
                field: "frecuencia".to_string(),
                expected: "Float o Int".to_string(),
                actual: format!("{:?}", args[0]),
                details: "El primer argumento debe ser un número (frecuencia)".to_string(),
            }),
        };

        let amp = match args[1] {
            osc::Type::Float(f) => f,
            _ => return Err(VisualizerError::ValidationError {
                field: "amplitud".to_string(),
                expected: "Float".to_string(),
                actual: format!("{:?}", args[1]),
                details: "El segundo argumento debe ser un flotante (amplitud)".to_string(),
            }),
        };

        let dur = match args[2] {
            osc::Type::Float(f) => f,
            _ => return Err(VisualizerError::ValidationError {
                field: "duración".to_string(),
                expected: "Float".to_string(),
                actual: format!("{:?}", args[2]),
                details: "El tercer argumento debe ser un flotante (duración)".to_string(),
            }),
        };

        if freq < self.audio_config.freq_min || freq > self.audio_config.freq_max {
            return Err(VisualizerError::ValidationError {
                field: "frecuencia".to_string(),
                expected: format!("entre {} y {}", self.audio_config.freq_min, self.audio_config.freq_max),
                actual: format!("{}", freq),
                details: "Frecuencia de drone fuera de rango válido".to_string(),
            });
        }
        if amp < self.audio_config.amp_min || amp > self.audio_config.amp_max {
            return Err(VisualizerError::ValidationError {
                field: "amplitud".to_string(),
                expected: format!("entre {} y {}", self.audio_config.amp_min, self.audio_config.amp_max),
                actual: format!("{}", amp),
                details: "Amplitud de drone fuera de rango válido".to_string(),
            });
        }
        if dur < self.audio_config.dur_min || dur > self.audio_config.dur_max {
            return Err(VisualizerError::ValidationError {
                field: "duración".to_string(),
                expected: format!("entre {} y {}", self.audio_config.dur_min, self.audio_config.dur_max),
                actual: format!("{}", dur),
                details: "Duración de drone fuera de rango válido".to_string(),
            });
        }

        Ok((freq, amp, dur))
    }

    pub fn validate_analysis_data_args(&self, args: &[osc::Type]) -> VisualizerResult<(f32, f32, f32)> {
        // Esta función también podría ser privada o eliminada.
        if args.len() != 3 {
            return Err(VisualizerError::ValidationError {
                field: "número de argumentos".to_string(),
                expected: "3".to_string(),
                actual: format!("{}", args.len()),
                details: "Se esperaban 3 argumentos (amplitud, brillo, ruidoso) para /analysis_data".to_string(),
            });
        }

        let amp = match args[0] {
            osc::Type::Float(f) => f,
            _ => return Err(VisualizerError::ValidationError {
                field: "amplitud".to_string(),
                expected: "Float".to_string(),
                actual: format!("{:?}", args[0]),
                details: "El primer argumento debe ser un flotante (amplitud)".to_string(),
            }),
        };

        let bright = match args[1] {
            osc::Type::Float(f) => f,
            _ => return Err(VisualizerError::ValidationError {
                field: "brillo".to_string(),
                expected: "Float".to_string(),
                actual: format!("{:?}", args[1]),
                details: "El segundo argumento debe ser un flotante (brillo)".to_string(),
            }),
        };

        let noisy = match args[2] {
            osc::Type::Float(f) => f,
            _ => return Err(VisualizerError::ValidationError {
                field: "ruidoso".to_string(),
                expected: "Float".to_string(),
                actual: format!("{:?}", args[2]),
                details: "El tercer argumento debe ser un flotante (ruidoso)".to_string(),
            }),
        };

        Ok((amp, bright, noisy))
    }

    pub fn validate_cluster_args(&self, args: &[osc::Type]) -> VisualizerResult<(f32, f32, f32, f32, f32)> {
        // Esta función también podría ser privada o eliminada.
        if args.len() != 5 {
            return Err(VisualizerError::ValidationError {
                field: "número de argumentos".to_string(),
                expected: "5".to_string(),
                actual: format!("{}", args.len()),
                details: "Se esperaban 5 argumentos (centro_frec, ancho_frec, densidad, amplitud, duración) para /cluster".to_string(),
            });
        }

        let center_freq = match args[0] {
            osc::Type::Float(f) => f,
            osc::Type::Int(i) => i as f32,
            _ => return Err(VisualizerError::ValidationError {
                field: "centro_frecuencia".to_string(),
                expected: "Float o Int".to_string(),
                actual: format!("{:?}", args[0]),
                details: "El primer argumento debe ser un número (frecuencia central)".to_string(),
            }),
        };

        let freq_width = match args[1] {
            osc::Type::Float(f) => f,
            osc::Type::Int(i) => i as f32,
            _ => return Err(VisualizerError::ValidationError {
                field: "ancho_frecuencia".to_string(),
                expected: "Float o Int".to_string(),
                actual: format!("{:?}", args[1]),
                details: "El segundo argumento debe ser un número (ancho de frecuencia)".to_string(),
            }),
        };

        let density = match args[2] {
            osc::Type::Float(f) => f,
            osc::Type::Int(i) => i as f32,
            _ => return Err(VisualizerError::ValidationError {
                field: "densidad".to_string(),
                expected: "Float o Int".to_string(),
                actual: format!("{:?}", args[2]),
                details: "El tercer argumento debe ser un número (densidad)".to_string(),
            }),
        };

        let amp = match args[3] {
            osc::Type::Float(f) => f,
            _ => return Err(VisualizerError::ValidationError {
                field: "amplitud".to_string(),
                expected: "Float".to_string(),
                actual: format!("{:?}", args[3]),
                details: "El cuarto argumento debe ser un flotante (amplitud)".to_string(),
            }),
        };

        let dur = match args[4] {
            osc::Type::Float(f) => f,
            _ => return Err(VisualizerError::ValidationError {
                field: "duración".to_string(),
                expected: "Float".to_string(),
                actual: format!("{:?}", args[4]),
                details: "El quinto argumento debe ser un flotante (duración)".to_string(),
            }),
        };

        if center_freq < self.audio_config.freq_min || center_freq > self.audio_config.freq_max {
            return Err(VisualizerError::ValidationError {
                field: "centro_frecuencia".to_string(),
                expected: format!("entre {} y {}", self.audio_config.freq_min, self.audio_config.freq_max),
                actual: format!("{}", center_freq),
                details: "Frecuencia central fuera de rango válido".to_string(),
            });
        }
        
        Ok((center_freq, freq_width, density, amp, dur))
    }


}

pub fn map_processed_to_musical(rx: std::sync::mpsc::Receiver<ProcessedOscMessage>) -> impl Iterator<Item = crate::events::MusicalEvent> {
    rx.into_iter().filter_map(|processed| {
        let timestamp = processed.timestamp; // Captura el timestamp una vez

        match processed.addr.as_str() {
            "/note_on" => {
                if processed.args.len() == 3 {
                    let freq = match &processed.args[0] {
                        osc::Type::Float(f) => *f,
                        osc::Type::Int(i) => *i as f32,
                        _ => { Logger::log_warn("Argumento de frecuencia inválido para /note_on"); return None; },
                    };
                    let amp = match &processed.args[1] {
                        osc::Type::Float(f) => *f,
                        _ => { Logger::log_warn("Argumento de amplitud inválido para /note_on"); return None; },
                    };
                    let dur = match &processed.args[2] {
                        osc::Type::Float(f) => *f,
                        _ => { Logger::log_warn("Argumento de duración inválido para /note_on"); return None; },
                    };
                    Some(MusicalEvent::Note {
                        frequency: freq,
                        amplitude: amp,
                        duration: dur,
                        instrument: "default".to_string(), // Podrías añadir un argumento para esto
                        start_time: timestamp,
                    })
                } else {
                    Logger::log_warn(&format!("Número incorrecto de argumentos para /note_on: esperado 3, recibido {}", processed.args.len()));
                    None
                }
            },
            "/drone_on" => {
                if processed.args.len() == 3 {
                    let freq = match &processed.args[0] {
                        osc::Type::Float(f) => *f,
                        osc::Type::Int(i) => *i as f32,
                        _ => { Logger::log_warn("Argumento de frecuencia inválido para /drone_on"); return None; },
                    };
                    let amp = match &processed.args[1] {
                        osc::Type::Float(f) => *f,
                        _ => { Logger::log_warn("Argumento de amplitud inválido para /drone_on"); return None; },
                    };
                    let dur = match &processed.args[2] {
                        osc::Type::Float(f) => *f,
                        _ => { Logger::log_warn("Argumento de duración inválido para /drone_on"); return None; },
                    };
                    Some(MusicalEvent::Drone {
                        frequency: freq,
                        amplitude: amp,
                        duration: dur,
                        instrument: "default".to_string(), // Podrías añadir un argumento para esto
                        start_time: timestamp,
                    })
                } else {
                    Logger::log_warn(&format!("Número incorrecto de argumentos para /drone_on: esperado 3, recibido {}", processed.args.len()));
                    None
                }
            },
            "/cluster" => {
                if processed.args.len() == 4 { // Tu SC envía 4 args: freq, amp, dur, density
                    let freq = match &processed.args[0] {
                        osc::Type::Float(f) => *f,
                        osc::Type::Int(i) => *i as f32,
                        _ => { Logger::log_warn("Argumento de frecuencia inválido para /cluster"); return None; },
                    };
                    let amp = match &processed.args[1] {
                        osc::Type::Float(f) => *f,
                        _ => { Logger::log_warn("Argumento de amplitud inválido para /cluster"); return None; },
                    };
                    let dur = match &processed.args[2] {
                        osc::Type::Float(f) => *f,
                        _ => { Logger::log_warn("Argumento de duración inválido para /cluster"); return None; },
                    };
                    let density = match &processed.args[3] {
                        osc::Type::Float(f) => *f,
                        osc::Type::Int(i) => *i as f32,
                        _ => { Logger::log_warn("Argumento de densidad inválido para /cluster"); return None; },
                    };
                    Some(MusicalEvent::Cluster {
                        center_freq: freq, // Usamos freq como center_freq para este mapeo simple
                        freq_width: 0.0, // Puedes añadir un argumento para esto si SC lo envía
                        density: density,
                        amplitude: amp,
                        duration: dur,
                    })
                } else {
                    Logger::log_warn(&format!("Número incorrecto de argumentos para /cluster: esperado 4, recibido {}", processed.args.len()));
                    None
                }
            },
            "/clear" => {
                // Para manejar /clear, necesitarás una forma de enviar un comando al modelo
                // que no sea a través de MusicalEvent, o un MusicalEvent.Clear.
                // Por ahora, simplemente no generaremos un MusicalEvent para este mensaje.
                Logger::log_info("Mensaje /clear recibido. La lógica de limpieza debe ser manejada en el modelo.");
                None
            },
            "/test" => {
                if !processed.args.is_empty() {
                    Logger::log_info(&format!("Mensaje de prueba recibido desde SuperCollider: {:?}", processed.args));
                }
                None // Los mensajes de prueba no se convierten en eventos musicales
            },
            _ => {
                Logger::log_warn(&format!("Dirección OSC desconocida recibida: {}", processed.addr));
                None
            }
        }
    })
}
