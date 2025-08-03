use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use std::sync::mpsc::Sender;
use thiserror::Error;
use num_traits::ToPrimitive;

#[derive(Error, Debug)]
pub enum AudioCaptureError {
    #[error("No se encontró dispositivo de entrada de audio")]
    NoInputDevice,
    #[error("Error de configuración de entrada de audio: {0}")]
    ConfigError(String),
    #[error("Error al iniciar el stream de entrada: {0}")]
    StreamBuildError(String),
    #[error("Error al reproducir stream: {0}")]
    StreamPlayError(String),
}

/// Inicia la captura de audio desde el micrófono.
/// Envía muestras de tipo `f32` por el canal proporcionado.
/// ⚠️ Importante: el `Stream` devuelto debe mantenerse vivo mientras quieras recibir datos del micrófono.
pub fn start_audio_capture(event_sender: Sender<f32>) -> Result<Stream, AudioCaptureError> {
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or(AudioCaptureError::NoInputDevice)?;
    let config = device
        .default_input_config()
        .map_err(|e| AudioCaptureError::ConfigError(e.to_string()))?;

    let sample_format = config.sample_format();
    let config = config.into();

    let stream_result = match sample_format {
        SampleFormat::F32 => build_stream::<f32>(&device, &config, event_sender),
        SampleFormat::I16 => build_stream::<i16>(&device, &config, event_sender),
        SampleFormat::U16 => build_stream::<u16>(&device, &config, event_sender),
        _ => {
            eprintln!("Unsupported sample format: {:?}", sample_format);
            return Err(AudioCaptureError::ConfigError("Unsupported sample format".to_string()));
        }
    };

    let stream = stream_result.map_err(|e| AudioCaptureError::StreamBuildError(e.to_string()))?;
    stream
        .play()
        .map_err(|e| AudioCaptureError::StreamPlayError(e.to_string()))?;
    Ok(stream)
}

// Función genérica para capturar cualquier formato de muestra
fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    sender: Sender<f32>,
) -> Result<Stream, cpal::BuildStreamError>
where
    T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f32> + dasp_sample::ToSample<f32> + ToPrimitive,
{
    device.build_input_stream(
        config,
        move |data: &[T], _| {
            for &sample in data {
                let val = sample.to_f32();
                if sender.send(val).is_err() {
                    break;
                }
            }
        },
        move |err| {
            eprintln!("⚠️ Error en el stream de audio: {}", err);
        },
        None,
    )
}
