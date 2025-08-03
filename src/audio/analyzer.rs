// 🔍 Analizador de audio
// Funcionalidad para analizar características del audio

#[derive(Debug, Default)]
pub struct AudioAnalyzer {
    pub spectrum: Option<Vec<f32>>,
    pub pitch: Option<f32>,
    pub rms: Option<f32>,
}

impl AudioAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze_frame(&mut self, buffer: &[f32]) {
        self.rms = Some(Self::compute_rms(buffer));
        // TODO: análisis de espectro y pitch
    }

    /// Calcula el valor RMS (Root Mean Square) del buffer de audio,
    /// que se interpreta como una medida de la energía o volumen percibido.
    fn compute_rms(buffer: &[f32]) -> f32 {
        let sum_squares: f32 = buffer.iter().map(|x| x * x).sum();
        (sum_squares / buffer.len() as f32).sqrt()
    }

    /// TODO: Implementar análisis de espectro (por ejemplo con FFT)
    fn compute_spectrum(_buffer: &[f32]) -> Vec<f32> {
        vec![]
    }

    /// TODO: Implementar detección de pitch (frecuencia fundamental)
    fn compute_pitch(_buffer: &[f32]) -> f32 {
        0.0
    }
}
