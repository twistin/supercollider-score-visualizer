/// Enum que representa los distintos contextos musicales posibles para una nota o evento.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    Melodic,
    Rhythmic,
    Cluster,
    Noise,
    Chord,
}
