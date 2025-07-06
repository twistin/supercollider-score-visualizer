// =============================================================================
// 🎼 REALTIME AUDIO ANALYZER - SuperCollider
// =============================================================================
// Análisis de audio en tiempo real para visualización sincronizada
// Envía datos por OSC cada 20ms (50 Hz) para sync ultra-preciso
// =============================================================================

(
"🎵 Inicializando Motor de Análisis de Audio en Tiempo Real...".postln;

// =============================================================================
// 🔧 SYNTHDEFS DE ANÁLISIS
// =============================================================================

SynthDef(\RealtimeAnalyzer, {
    |inputBus = 0, oscPort = 57124, analyzeRate = 50|
    
    // Todas las declaraciones var deben estar al inicio
    var in, amp, pitch, hasFreq, onset, centroid, flux, rolloff, flatness;
    var mfcc, spectralSlope, noisiness, harmonicity, fft;
    
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
    
    // === ANÁLISIS DE ONSETS ===
    onset = Onsets.kr(
        chain: FFT(LocalBuf(1024), in), 
        odftype: \rcomplex,
        threshold: 0.3,
        relaxtime: 1.0,
        floor: 0.1,
        mingap: 10
    );
    
    // === ANÁLISIS ESPECTRAL ===
    fft = FFT(LocalBuf(2048), in);
    
    // Centroide espectral (timbre "brillantez")
    centroid = SpecCentroid.kr(fft);
    
    // Flujo espectral aproximado (cambio temporal del centroide)
    flux = LPZ1.kr(centroid).abs;  // Primera diferencia del centroide
    
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
    // Valores altos = más energía en frecuencias altas = pendiente positiva
    // Valores bajos = más energía en frecuencias bajas = pendiente negativa
    spectralSlope = (centroid - 1000) / 1000;  // Normalizado alrededor de 1kHz
    
    // === ENVÍO OSC ULTRA-RÁPIDO ===
    SendReply.kr(
        Impulse.kr(analyzeRate), 
        "/realtime_audio", 
        [
            // Parámetros básicos
            pitch,           // [0] Frecuencia fundamental
            amp,             // [1] Amplitud RMS
            onset,           // [2] Detección de onset (0 o 1)
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
// 🎛️ CONTROLADOR DE ANÁLISIS
// =============================================================================

~startAnalysis = {
    |inputBus = 0, rate = 50|
    
    // Detener análisis previo si existe
    if (~analyzer.notNil) { 
        ~analyzer.free; 
        "⏹️ Análisis previo detenido.".postln;
    };
    
    // Iniciar nuevo análisis
    ~analyzer = Synth(\RealtimeAnalyzer, [
        \inputBus, inputBus,
        \analyzeRate, rate
    ]);
    
    "🎵 Análisis de audio iniciado:".postln;
    ("   📍 Input Bus: " ++ inputBus).postln;
    ("   ⚡ Rate: " ++ rate ++ " Hz").postln;
    ("   📡 OSC Port: 57124").postln;
    ("   🎯 OSC Address: /realtime_audio").postln;
};

~stopAnalysis = {
    if (~analyzer.notNil) { 
        ~analyzer.free; 
        ~analyzer = nil;
        "⏹️ Análisis de audio detenido.".postln;
    } {
        "⚠️ No hay análisis activo.".postln;
    };
};

// =============================================================================
// 🧪 RECEPTOR OSC PARA DEBUGGING
// =============================================================================

OSCdef(\analyzerDebug, {
    |msg|
    var pitch, amp, onset, hasFreq, centroid, flux, rolloff, flatness;
    var harmonicity, noisiness, spectralSlope;
    
    if (msg.size >= 14) {
        pitch = msg[3];
        amp = msg[4];
        onset = msg[5];
        hasFreq = msg[6];
        centroid = msg[7];
        flux = msg[8];
        rolloff = msg[9];
        flatness = msg[10];
        harmonicity = msg[11];
        noisiness = msg[12];
        spectralSlope = msg[13];
        
        // Solo mostrar cuando hay actividad significativa
        if ((amp > 0.01) || (onset > 0)) {
            [
                "🎵 ANÁLISIS:",
                "Pitch:", pitch.round(0.1), "Hz",
                "Amp:", amp.round(0.001),
                "Onset:", onset,
                "Centroid:", centroid.round(1), "Hz",
                "Harmonicity:", harmonicity.round(0.01)
            ].postln;
        };
    };
}, "/realtime_audio");

// =============================================================================
// 🚀 FUNCIONES DE PRUEBA RÁPIDA
// =============================================================================

~testAnalysis = {
    "🧪 Iniciando prueba de análisis...".postln;
    ~startAnalysis.(0, 50);  // Input bus 0, 50 Hz
    
    // Generar tono de prueba
    ~testTone = {
        var freq = SinOsc.kr(0.2).range(220, 880);  // Glissando lento
        var amp = SinOsc.kr(0.5).range(0.1, 0.8);   // Amplitud variable
        SinOsc.ar(freq) * amp * 0.3;
    }.play;
    
    "🎼 Tono de prueba generado. Verifica la recepción OSC.".postln;
    "⏱️ Ejecuta ~stopTest.() para detener.".postln;
};

~stopTest = {
    if (~testTone.notNil) { ~testTone.free; ~testTone = nil; };
    ~stopAnalysis.();
    "🛑 Prueba detenida.".postln;
};

// =============================================================================
// 🎯 INSTRUCCIONES DE USO
// =============================================================================

"".postln;
"🎵 MOTOR DE ANÁLISIS DE AUDIO LISTO".postln;
"".postln;
"📋 COMANDOS DISPONIBLES:".postln;
"   ~startAnalysis.()     - Iniciar análisis".postln;
"   ~stopAnalysis.()      - Detener análisis".postln;
"   ~testAnalysis.()      - Prueba con tono generado".postln;
"   ~stopTest.()          - Detener prueba".postln;
"".postln;
"🎯 PRÓXIMO PASO: Ejecutar ~testAnalysis.() y verificar".postln;
"   que el visualizador Rust reciba los datos OSC.".postln;
"".postln;

)
