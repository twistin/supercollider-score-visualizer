/// Representa el tipo de contexto musical de un evento,
/// usado para determinar su tratamiento visual, sonoro o analítico.
#[derive(Debug, Clone, Copy)]
pub enum Context {
    /// Evento melódico: notas con altura definida en secuencia.
    Melodic,
    /// Evento rítmico: percusiones o elementos sin tono definido.
    Rhythmic,
    /// Agrupación de notas o sonidos densos sin función armónica clara.
    Cluster,
    /// Evento de ruido: material sonoro no tonal.
    Noise,
    /// Acordes estructurados con intención armónica.
    Chord,
}

// No changes requested to the file musical_events.rs; no output needed.