/// Enum que representa los distintos contextos musicales posibles para una nota o evento.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    /// Notas o frases melódicas individuales.
    Melodic,
    /// Patrones o elementos rítmicos.
    Rhythmic,
    /// Agrupaciones densas de sonidos simultáneos.
    Cluster,
    /// Material no tonal o percusivo.
    Noise,
    /// Acordes definidos o implícitos.
    Chord,
}

use std::fmt;

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Context::Melodic => "Melodic",
            Context::Rhythmic => "Rhythmic",
            Context::Cluster => "Cluster",
            Context::Noise => "Noise",
            Context::Chord => "Chord",
        };
        write!(f, "{}", s)
    }
}