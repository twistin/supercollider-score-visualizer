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

    fn compute_rms(buffer: &[f32]) -> f32 {
        let sum_squares: f32 = buffer.iter().map(|x| x * x).sum();
        (sum_squares / buffer.len() as f32).sqrt()
    }
}
