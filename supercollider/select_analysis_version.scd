(
// =============================================================================
// 🎯 SELECTOR AUTOMÁTICO DE VERSIÓN DE ANÁLISIS
// =============================================================================

"🎯 SELECCIONANDO VERSIÓN ÓPTIMA DE ANÁLISIS...".postln;

// Test rápido de disponibilidad de Onsets
try {
    // Intentar crear un SynthDef simple con Onsets
    SynthDef(\TestOnsetsAvailable, {
        var in, fft, onset;
        in = SinOsc.ar(440) * 0.1;
        fft = FFT(LocalBuf(1024), in);
        onset = Onsets.kr(fft, \rcomplex, 0.3);
        Out.ar(0, Silent.ar(2));
    }).add;
    
    "✅ Onsets UGen disponible - Cargando versión completa...".postln;
    thisProcess.interpreter.executeFile("realtime_analysis.scd".resolveRelative);
    
} {
    "⚠️ Onsets UGen no disponible - Cargando versión alternativa...".postln;
    thisProcess.interpreter.executeFile("realtime_analysis_alt.scd".resolveRelative);
};

"🎵 Sistema de análisis listo para usar.".postln;
)
