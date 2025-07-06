(
// =============================================================================
// 🧪 VALIDADOR DE SINTAXIS UGENS - SuperCollider
// =============================================================================

"🔧 Validando sintaxis de UGens específicos...".postln;

// Test 1: Onsets UGen
"".postln;
"📊 Test 1: Onsets UGen".postln;
"===================".postln;

try {
    // Sintaxis corregida para Onsets
    {
        var in, fft, onset;
        in = SinOsc.ar(440) * 0.1;
        fft = FFT(LocalBuf(1024), in);
        onset = Onsets.kr(
            fft,           // chain
            \rcomplex,     // odftype
            0.3,           // threshold
            1.0,           // relaxtime
            0.1,           // floor
            10             // mingap
        );
        onset.poll(1, "Onset");
        Silent.ar(2);
    }.play;
    
    "✅ Onsets: Sintaxis CORRECTA".postln;
    
} { |error|
    ("❌ Onsets: Error - " ++ error.errorString).postln;
};

// Test 2: Pitch UGen
"".postln;
"📊 Test 2: Pitch UGen".postln;
"==================".postln;

try {
    {
        var in, pitch, hasFreq;
        in = SinOsc.ar(440) * 0.1;
        # pitch, hasFreq = Pitch.kr(
            in,
            initFreq: 440,
            minFreq: 60,
            maxFreq: 4000,
            ampThreshold: 0.02,
            peakThreshold: 0.5,
            downSample: 1
        );
        pitch.poll(1, "Pitch");
        Silent.ar(2);
    }.play;
    
    "✅ Pitch: Sintaxis CORRECTA".postln;
    
} { |error|
    ("❌ Pitch: Error - " ++ error.errorString).postln;
};

// Test 3: Análisis espectral
"".postln;
"📊 Test 3: Análisis Espectral".postln;
"===========================".postln;

try {
    {
        var in, fft, centroid, flatness, rolloff;
        in = SinOsc.ar(440) * 0.1;
        fft = FFT(LocalBuf(2048), in);
        
        centroid = SpecCentroid.kr(fft);
        flatness = SpecFlatness.kr(fft);
        rolloff = SpecPcile.kr(fft, 0.85, 1);
        
        centroid.poll(1, "Centroid");
        Silent.ar(2);
    }.play;
    
    "✅ Análisis Espectral: Sintaxis CORRECTA".postln;
    
} { |error|
    ("❌ Análisis Espectral: Error - " ++ error.errorString).postln;
};

// Test 4: Compilación completa del SynthDef
"".postln;
"📊 Test 4: SynthDef Completo".postln;
"==========================".postln;

// Parar cualquier análisis previo
if (~analyzer.notNil) { ~analyzer.free; ~analyzer = nil; };

try {
    // Cargar y compilar el archivo completo
    thisProcess.interpreter.executeFile("realtime_analysis.scd".resolveRelative);
    "✅ SynthDef RealtimeAnalyzer: COMPILADO EXITOSAMENTE".postln;
    
} { |error|
    ("❌ SynthDef RealtimeAnalyzer: Error - " ++ error.errorString).postln;
    ("   Línea aproximada: " ++ error.protectedBacktrace).postln;
};

"".postln;
"🎯 RESUMEN DE VALIDACIÓN:".postln;
"========================".postln;
"Si todos los tests muestran ✅, el archivo está listo para uso.".postln;
"Si hay errores ❌, revisa la sintaxis de los UGens indicados.".postln;
"".postln;
)
