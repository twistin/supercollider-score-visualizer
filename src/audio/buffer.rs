// 🗄️ Buffer de audio
// Manejo de buffers de audio para análisis (futuro)

#[derive(Debug, Default)]
pub struct AudioBuffer {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

impl AudioBuffer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            samples: Vec::new(),
            sample_rate,
        }
    }

    pub fn push_sample(&mut self, sample: f32) {
        self.samples.push(sample);
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_clear_buffer() {
        let mut buffer = AudioBuffer::new(44100);
        buffer.push_sample(0.5);
        assert_eq!(buffer.len(), 1);
        buffer.clear();
        assert!(buffer.is_empty());
    }
}
