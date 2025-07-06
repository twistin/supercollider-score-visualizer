// =============================================================================
// 🎼 REALTIME AUDIO ANALYZER ALTERNATIVO - SuperCollider
// =============================================================================
// Versión sin Onsets para compatibilidad máxima
// Análisis de audio en tiempo real para visualización sincronizada
// =============================================================================

(
"🎵 Inicializando Motor de Análisis ALTERNATIVO (sin Onsets)...".postln;

// =============================================================================
// 🔧 SYNTHDEF ALTERNATIVO SIN ONSETS
// =============================================================================

SynthDef(\RealtimeAnalyzerAlt, {
    |inputBus = 0, oscPort = 57124, analyzeRate = 50|
    
    // Todas las declaraciones var al inicio
    var in, amp, pitch, hasFreq, onset, centroid, flux, rolloff, flatness;
    var mfcc, spectralSlope, noisiness, harmonicity, fft;
    var prevAmp, ampDiff;
    
    // Entrada de audio (configurable)
    in = SoundIn.ar(inputBus);
    
    // === ANÁLISIS TEMPORAL ===
    amp = Amplitude.kr(in, 0.01, 0.1);  // Amplitud con ataque/release rápido
    
    // === ANÁLISIS DE PITCH ===
    # pitch, hasFreq = Pitch.kr(
        in, 
        initFreq: 440, 
        minFreq: 60, 
        maxFreq: 4000,
        ampThreshold: 0.02,
        peakThreshold: 0.5,
        downSample: 1
    );
    
    // === DETECCIÓN DE ONSETS ALTERNATIVA ===
    // Usar diferencia de amplitud como aproximación a onsets
    prevAmp = Delay1.kr(amp);
    ampDiff = (amp - prevAmp).max(0);  // Solo diferencias positivas
    onset = (ampDiff > 0.1).lag(0.01); // Umbral y suavizado
    
    // === ANÁLISIS ESPECTRAL ===
    fft = FFT(LocalBuf(2048), in);
    
    // Centroide espectral (timbre "brillantez")
    centroid = SpecCentroid.kr(fft);
    
    // Flujo espectral aproximado (primera diferencia del centroide)
    flux = LPZ1.kr(centroid).abs;
    
    // Rolloff espectral (contenido de frecuencias altas)
    rolloff = SpecPcile.kr(fft, 0.85, 1);
    
    // Planitud espectral (medida de "ruidosidad")
    flatness = SpecFlatness.kr(fft);
    
    // === ANÁLISIS AVANZADO ===
    
    // Harmonicidad (qué tan tonal vs ruidoso)
    harmonicity = hasFreq * (1 - flatness);
    
    // Noisiness (inverso de harmonicidad)
    noisiness = flatness * (1 - hasFreq);
    
    // Pendiente espectral aproximada (usando centroide como proxy)
    spectralSlope = (centroid - 1000) / 1000;  // Normalizado alrededor de 1kHz
    
    // === ENVÍO OSC ULTRA-RÁPIDO ===
    SendReply.kr(
        Impulse.kr(analyzeRate), 
        "/realtime_audio", 
        [
            // Parámetros básicos
            pitch,           // [0] Frecuencia fundamental
            amp,             // [1] Amplitud RMS
            onset,           // [2] Detección de onset (alternativa)
            hasFreq,         // [3] Confianza de pitch (0-1)
            
            // Parámetros espectrales
            centroid,        // [4] Centroide espectral
            flux,            // [5] Flujo espectral
            rolloff,         // [6] Rolloff espectral
            flatness,        // [7] Planitud espectral
            
            // Parámetros derivados
            harmonicity,     // [8] Harmonicidad (0-1)
            noisiness,       // [9] Ruidosidad (0-1)
            spectralSlope    // [10] Pendiente espectral
        ]
    );
    
    // Salida silenciosa (solo análisis)
    Out.ar(0, Silent.ar(2));
    
}).add;

// =============================================================================
// 🎛️ CONTROLADOR DE ANÁLISIS ALTERNATIVO
// =============================================================================

~startAnalysisAlt = {
    |inputBus = 0, rate = 50|
    
    // Detener análisis previo si existe
    if (~analyzerAlt.notNil) { 
        ~analyzerAlt.free; 
        "⏹️ Análisis alternativo previo detenido.".postln;
    };
    
    // Iniciar nuevo análisis
    ~analyzerAlt = Synth(\RealtimeAnalyzerAlt, [
        \inputBus, inputBus,
        \analyzeRate, rate
    ]);
    
    "🎵 Análisis de audio ALTERNATIVO iniciado:".postln;
    ("   📍 Input Bus: " ++ inputBus).postln;
    ("   ⚡ Rate: " ++ rate ++ " Hz").postln;
    ("   📡 OSC Port: 57124").postln;
    ("   🎯 OSC Address: /realtime_audio").postln;
    ("   🔄 Método: Detección de onsets por diferencia de amplitud").postln;
};

~stopAnalysisAlt = {
    if (~analyzerAlt.notNil) { 
        ~analyzerAlt.free; 
        ~analyzerAlt = nil;
        "⏹️ Análisis alternativo detenido.".postln;
    } {
        "⚠️ No hay análisis alternativo activo.".postln;
    };
};

// =============================================================================
// 🚀 FUNCIÓN DE PRUEBA ALTERNATIVA
// =============================================================================

~testAnalysisAlt = {
    "🧪 Iniciando prueba de análisis ALTERNATIVO...".postln;
    ~startAnalysisAlt.(0, 50);  // Input bus 0, 50 Hz
    
    // Generar tono de prueba
    ~testToneAlt = {
        var freq = SinOsc.kr(0.2).range(220, 880);  // Glissando lento
        var amp = SinOsc.kr(0.5).range(0.1, 0.8);   // Amplitud variable
        SinOsc.ar(freq) * amp * 0.3;
    }.play;
    
    "🎼 Tono de prueba generado (versión alternativa).".postln;
    "⏱️ Ejecuta ~stopTestAlt.() para detener.".postln;
};

~stopTestAlt = {
    if (~testToneAlt.notNil) { ~testToneAlt.free; ~testToneAlt = nil; };
    ~stopAnalysisAlt.();
    "🛑 Prueba alternativa detenida.".postln;
};

// =============================================================================
// 🎯 INSTRUCCIONES ALTERNATIVAS
// =============================================================================

"".postln;
"🎵 MOTOR DE ANÁLISIS ALTERNATIVO LISTO".postln;
"(Sin dependencia de Onsets UGen)".postln;
"".postln;
"📋 COMANDOS DISPONIBLES:".postln;
"   ~startAnalysisAlt.()     - Iniciar análisis alternativo".postln;
"   ~stopAnalysisAlt.()      - Detener análisis alternativo".postln;
"   ~testAnalysisAlt.()      - Prueba con tono generado".postln;
"   ~stopTestAlt.()          - Detener prueba alternativa".postln;
"".postln;
"🎯 PRÓXIMO PASO: Ejecutar ~testAnalysisAlt.() y verificar".postln;
"   que el visualizador Rust reciba los datos OSC.".postln;
"".postln;

)
