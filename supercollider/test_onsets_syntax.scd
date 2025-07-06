// =============================================================================
// 🧪 TEST ESPECÍFICO PARA ONSETS - SuperCollider
// =============================================================================
// Validación de diferentes sintaxis para el UGen Onsets
// =============================================================================

(
"🧪 TEST ESPECÍFICO PARA SINTAXIS DE ONSETS".postln;
"============================================".postln;

// Test 1: Sintaxis con argumentos con nombre (keyword arguments)
"".postln;
"📊 Test 1: Onsets con keyword arguments".postln;
"======================================".postln;

try {
    SynthDef(\TestOnsets1, {
        var in, fft, onset;
        in = SinOsc.ar(440) * 0.1;
        fft = FFT(LocalBuf(1024), in);
        
        onset = Onsets.kr(
            chain: fft,
            odftype: \rcomplex,
            threshold: 0.3,
            relaxtime: 1.0,
            floor: 0.1,
            mingap: 10
        );
        
        onset.poll(1, "Onset_Keyword");
        Out.ar(0, Silent.ar(2));
    }).add;
    
    "✅ Test 1: EXITOSO - Keyword arguments funcionan".postln;
    
} { |error|
    ("❌ Test 1: Error - " ++ error.errorString).postln;
};

// Test 2: Sintaxis posicional simplificada
"".postln;
"📊 Test 2: Onsets sintaxis posicional mínima".postln;
"==========================================".postln;

try {
    SynthDef(\TestOnsets2, {
        var in, fft, onset;
        in = SinOsc.ar(440) * 0.1;
        fft = FFT(LocalBuf(1024), in);
        
        // Solo argumentos esenciales
        onset = Onsets.kr(fft, \rcomplex, 0.3);
        
        onset.poll(1, "Onset_Simple");
        Out.ar(0, Silent.ar(2));
    }).add;
    
    "✅ Test 2: EXITOSO - Sintaxis posicional simple funciona".postln;
    
} { |error|
    ("❌ Test 2: Error - " ++ error.errorString).postln;
};

// Test 3: Sintaxis alternativa con diferentes algoritmos
"".postln;
"📊 Test 3: Onsets con diferentes algoritmos".postln;
"=========================================".postln;

[\rcomplex, \hfc, \mkl, \power].do { |algorithm|
    try {
        SynthDef(("TestOnsets_" ++ algorithm).asSymbol, {
            var in, fft, onset;
            in = SinOsc.ar(440) * 0.1;
            fft = FFT(LocalBuf(1024), in);
            
            onset = Onsets.kr(fft, algorithm, 0.3);
            
            onset.poll(1, ("Onset_" ++ algorithm).asSymbol);
            Out.ar(0, Silent.ar(2));
        }).add;
        
        ("✅ Algoritmo " ++ algorithm ++ ": EXITOSO").postln;
        
    } { |error|
        ("❌ Algoritmo " ++ algorithm ++ ": Error - " ++ error.errorString).postln;
    };
};

// Test 4: Verificar documentación de Onsets
"".postln;
"📊 Test 4: Información sobre Onsets".postln;
"==================================".postln;

try {
    Onsets.dumpClassSubtree;
    "✅ Clase Onsets disponible".postln;
    
    // Mostrar ayuda si está disponible
    if (Onsets.respondsTo(\help)) {
        "📖 Ayuda disponible: Onsets.help".postln;
    };
    
} { |error|
    ("❌ Error accediendo a clase Onsets: " ++ error.errorString).postln;
};

"".postln;
"🎯 RECOMENDACIÓN:".postln;
"Si Test 1 (keyword) funciona, usar esa sintaxis.".postln;
"Si Test 2 (simple) funciona, usar esa sintaxis.".postln;
"Si ninguno funciona, Onsets puede no estar disponible.".postln;
"".postln;
)
