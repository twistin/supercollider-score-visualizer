// ✅ SC Score Visualizer - VERSIÓN CORREGIDA DEFINITIVA
// Puerto OSC: 57124

// =====================================================================
// 🎯 INSTRUCCIONES SIMPLES:
// =====================================================================
// 1. EJECUTA TODO EL BLOQUE ( ... ) DE UNA VEZ
// 2. ESPERA el mensaje "🎉 LISTO"
// 3. EJECUTA: scvTest()
// =====================================================================

(
// Variables globales del sistema
var osc, sendPoint, sendGliss, sendCluster, sendNoise, sendMass;
var testBasic, compositionXenakis, quickDemo;

"🚀 Iniciando SC Score Visualizer...".postln;

// Conexión OSC
osc = NetAddr.new("127.0.0.1", 57124);
"📡 OSC configurado en puerto 57124".postln;

// ========== FUNCIONES DE ENVÍO ==========

sendPoint = { |freq=440, amp=0.5, dur=1.0|
    osc.sendMsg("/event", "point", freq, amp, dur, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
    ("✅ Punto: " ++ freq ++ "Hz, " ++ dur ++ "s").postln;
};

sendGliss = { |startFreq=220, endFreq=880, amp=0.7, dur=3.0|
    osc.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur, 0, 0.1, 0.1, 0.7, 0.2, 0.1, 120);
    ("✅ Glissando: " ++ startFreq ++ " → " ++ endFreq ++ "Hz").postln;
};

sendCluster = { |centerFreq=500, spread=100, voices=6, amp=0.6, dur=2.0|
    osc.sendMsg("/event", "cluster", centerFreq, spread, voices, amp, dur, 0.1, 0.1, 0.8, 0.3, 0.4, 180);
    ("✅ Cluster: " ++ centerFreq ++ "Hz ±" ++ spread ++ ", " ++ voices ++ " voces").postln;
};

sendNoise = { |centerFreq=1000, bandwidth=500, amp=0.4, dur=2.0|
    osc.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur, 1.0, 0, 0.1, 0.1, 0.6, 0.8, 0.4, 60);
    ("✅ Ruido: " ++ centerFreq ++ "Hz ±" ++ bandwidth).postln;
};

sendMass = { |components=4, amp=0.5, dur=3.0|
    var freqs = Array.fill(components, { 200 + 1500.rand });
    var amps = Array.fill(components, { 0.3 + 0.7.rand });
    var profile = [freqs, amps].flop.flatten;
    var args = ["mass", components, amp, dur, 0.3, 0.2, 0.1, 0.1, 0.9, 0.5, 0.7, 280] ++ profile;
    osc.sendMsg("/event", *args);
    ("✅ Masa sonora: " ++ components ++ " componentes").postln;
};

// ========== DEMOS Y PRUEBAS ==========

testBasic = {
    "🧪 === PRUEBA BÁSICA (5 eventos) ===".postln;
    
    sendPoint.(440, 0.7, 2.0);
    
    { 2.5.wait; sendGliss.(220, 880, 0.8, 3.0); }.fork;
    { 4.wait; sendCluster.(660, 150, 8, 0.6, 4.0); }.fork;
    { 6.wait; sendNoise.(1000, 500, 0.5, 2.5); }.fork;
    { 8.wait; sendMass.(6, 0.6, 3.0); 
      { 3.5.wait; "✅ === PRUEBA COMPLETADA ===".postln; }.fork; 
    }.fork;
};

compositionXenakis = {
    "🎼 === COMPOSICIÓN ESTILO XENAKIS ===".postln;
    
    // Puntos dispersos (Metastasis)
    {
        20.do { |i|
            var freq = 100 + (3000 * (i/20)) + 100.rand2;
            var amp = 0.3 + 0.4.rand;
            var dur = 0.5 + 1.5.rand;
            sendPoint.(freq, amp, dur);
            (0.2 + 0.8.rand).wait;
        };
    }.fork;
    
    // Glissandi (Pithoprakta)
    {
        3.wait;
        8.do { |i|
            var start = 200 + 1000.rand;
            var end = 200 + 1000.rand;
            var dur = 2.0 + 3.0.rand;
            sendGliss.(start, end, 0.5 + 0.3.rand, dur);
            (dur * 0.3).wait;
        };
    }.fork;
    
    // Clusters estocásticos
    {
        8.wait;
        6.do { |i|
            var center = 300 + 1200.rand;
            var spread = 50 + 150.rand;
            var voices = 4 + 8.rand;
            sendCluster.(center, spread, voices, 0.4 + 0.3.rand, 3.0 + 2.0.rand);
            (1.0 + 2.0.rand).wait;
        };
    }.fork;
    
    "🎵 Duración total: ~30 segundos".postln;
};

quickDemo = {
    "⚡ === DEMO RÁPIDO ===".postln;
    sendPoint.(330, 0.6, 1.0);
    { 1.2.wait; sendGliss.(440, 880, 0.7, 2.0); }.fork;
    { 2.5.wait; sendCluster.(550, 100, 6, 0.5, 2.5); }.fork;
};

// ========== EXPORTAR A VARIABLES GLOBALES ==========

// Funciones principales
scvTest = testBasic;
scvXenakis = compositionXenakis;
scvQuick = quickDemo;

// Funciones de envío directo
scvSendPoint = sendPoint;
scvSendGliss = sendGliss;
scvSendCluster = sendCluster;
scvSendNoise = sendNoise;
scvSendMass = sendMass;

// ========== MENSAJE FINAL ==========

0.5.wait;

"".postln;
"🎉 ✅ ✅ ✅ LISTO PARA USAR ✅ ✅ ✅".postln;
"".postln;
"📋 FUNCIONES PRINCIPALES:".postln;
"  • scvTest()          - Prueba básica (RECOMENDADO)".postln;
"  • scvXenakis()       - Composición estilo Xenakis".postln;
"  • scvQuick()         - Demo rápido".postln;
"".postln;
"📦 FUNCIONES DE ENVÍO:".postln;
"  • scvSendPoint(freq, amp, dur)".postln;
"  • scvSendGliss(startFreq, endFreq, amp, dur)".postln;
"  • scvSendCluster(center, spread, voices, amp, dur)".postln;
"  • scvSendNoise(center, bandwidth, amp, dur)".postln;
"  • scvSendMass(components, amp, dur)".postln;
"".postln;
"🚀 EJECUTA AHORA: scvTest()".postln;
"📡 Puerto OSC: 57124".postln;
"🎯 Asegúrate de que el visualizador Rust esté corriendo".postln;
"".postln;

) // Fin del bloque principal

// Función para enviar un glissando
sendGliss = { |startFreq=440, endFreq=880, amp=0.7, dur=2.0, curvature=0.0, density=0.7, texture=0.0, spread=0.0, hue=30|
    visualizer.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur, curvature, density, texture, spread, hue);
    ("Enviado: glissando " ++ startFreq ++ " → " ++ endFreq).postln;
};

// Función para enviar un cluster
sendCluster = { |centerFreq=440, freqSpread=100, numVoices=8, amp=0.6, dur=3.0, density=0.8, texture=0.3, spread=0.5, hue=120|
    visualizer.sendMsg("/event", "cluster", centerFreq, freqSpread, numVoices, amp, dur, density, texture, spread, hue);
    ("Enviado: cluster centro=" ++ centerFreq ++ " voces=" ++ numVoices).postln;
};

// Función para enviar ruido
sendNoise = { |centerFreq=1000, bandwidth=500, amp=0.4, dur=2.0, grainSize=1.0, spectralTilt=0.0, density=0.6, texture=0.8, spread=0.4, hue=300|
    visualizer.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur, grainSize, spectralTilt, density, texture, spread, hue);
    ("Enviado: ruido centro=" ++ centerFreq ++ " ancho=" ++ bandwidth).postln;
};

// Función para enviar masa sonora
sendSoundMass = { |numComponents=5, amp=0.5, dur=4.0, evolutionRate=0.5, turbulence=0.3, density=0.9, texture=0.5, spread=0.7, hue=60|
    var msg = ["/event", "mass", numComponents, amp, dur, evolutionRate, turbulence, density, texture, spread, hue];

    // Añadir componentes espectrales
    numComponents.do { |i|
        var freq = 440 * (1 + (i * 0.618)); // Proporción áurea
        var componentAmp = 1.0 / (i + 1);
        msg = msg ++ [freq, componentAmp];
    };

    visualizer.sendMsg(*msg);
    ("Enviado: masa sonora con " ++ numComponents ++ " componentes").postln;
};

// --- Ejemplos de Composiciones Estilo Xenakis ---
"Definiendo funciones de composición...".postln;

// 1. Secuencia de puntos dispersos (como en Metastaseis)
metastasisPoints = {
    var numPoints = 50;
    var timeSpan = 10.0;

    numPoints.do { |i|
        var time = i / numPoints * timeSpan;
        var freq = 200 + (4000 * i / numPoints) + 200.rand;
        var amp = 0.3 + 0.4.rand;
        var density = i / numPoints; // Densidad creciente
        var texture = (i / numPoints).squared; // Textura creciente

        {
            time.wait;
            sendPoint.(freq, amp, 0.5 + 1.0.rand, 0.05, 0.1, density, texture, 0.2.rand, 360.rand);
        }.fork;
    };
};

// 2. Glissandi convergentes/divergentes (como en Pithoprakta)
pithopraktaGliss = {
    var numGliss = 12;
    var centerFreq = 440;

    numGliss.do { |i|
        var startTime = i * 0.3;
        var startFreq = centerFreq * (0.5 + 1.5.rand);
        var endFreq = centerFreq * (0.5 + 1.5.rand);
        var curvature = -1.0 + 2.0.rand; // Curvaturas aleatorias
        var duration = 3.0 + 2.0.rand;

        {
            startTime.wait;
            sendGliss.(startFreq, endFreq, 0.4 + 0.3.rand, duration, curvature, 0.8, 0.2.rand, 0.3.rand, 30 + 60.rand);
        }.fork;
    };
};

// 3. Nube de clusters (masas sonoras estocásticas)
stochasticClouds = {
    var numClouds = 8;
    var timeSpan = 15.0;

    numClouds.do { |i|
        var time = timeSpan.rand;
        var centerFreq = 100 + 3000.rand;
        var spread = 50 + 200.rand;
        var numVoices = 4 + 12.rand;
        var duration = 2.0 + 3.0.rand;

        {
            time.wait;
            sendCluster.(centerFreq, spread, numVoices, 0.5 + 0.4.rand, duration, 0.7 + 0.3.rand, 0.5.rand, 0.6.rand, 360.rand);
        }.fork;
    };
};

// 4. Texturas de ruido evolutivas
noiseTextures = {
    var numTextures = 6;

    numTextures.do { |i|
        var time = i * 2.5;
        var centerFreq = 500 + 1500.rand;
        var bandwidth = 200 + 800.rand;
        var grainSize = 0.5 + 1.5.rand;
        var spectralTilt = -0.5 + 1.0.rand;

        {
            time.wait;
            sendNoise.(centerFreq, bandwidth, 0.3 + 0.3.rand, 3.0 + 2.0.rand, grainSize, spectralTilt, 0.6 + 0.4.rand, 0.8 + 0.2.rand, 0.4 + 0.6.rand, 270 + 90.rand);
        }.fork;
    };
};

// 5. Masas sonoras complejas con evolución espectral
spectralMasses = {
    var numMasses = 4;

    numMasses.do { |i|
        var time = i * 4.0;
        var numComponents = 6 + 4.rand;
        var evolutionRate = 0.2 + 0.6.rand;
        var turbulence = 0.1 + 0.4.rand;

        {
            time.wait;
            sendSoundMass.(numComponents, 0.4 + 0.4.rand, 5.0 + 3.0.rand, evolutionRate, turbulence, 0.8 + 0.2.rand, 0.4 + 0.4.rand, 0.7 + 0.3.rand, 60 + 120.rand);
        }.fork;
    };
};

// --- Función principal para composición automática ---
xenakisComposition = {
    "Iniciando composición estilo Xenakis...".postln;

    // Ejecutar las diferentes secciones con timing específico
    metastasisPoints.();

    {
        5.wait;
        pithopraktaGliss.();
    }.fork;

    {
        8.wait;
        stochasticClouds.();
    }.fork;

    {
        12.wait;
        noiseTextures.();
    }.fork;

    {
        15.wait;
        spectralMasses.();
    }.fork;
};

// --- Patrones rítmicos complejos ---
rhythmicPatterns = {
    var durations = [0.1, 0.15, 0.2, 0.3, 0.5, 0.8, 1.2]; // Serie de Fibonacci modificada
    var freqs = (1..12).collect { |i| 440 * (2 ** (i/12)) }; // Escala cromática

    100.do { |i|
        var dur = durations.choose;
        var freq = freqs.choose;
        var amp = 0.2 + 0.5.rand;
        var density = (i % 10) / 10; // Densidad cíclica

        sendPoint.(freq, amp, dur * 2, 0.02, 0.05, density, 0.1.rand, 0.15.rand, (i * 37) % 360);
        dur.wait;
    };
};

// --- Función de prueba básica ---
testBasicEvents = {
    "=== Iniciando prueba básica de eventos ===".postln;
    
    // Enviar punto
    "1/5: Enviando punto...".postln;
    sendPoint.(440, 0.5, 2.0);
    
    // Enviar glissando después de 1 segundo
    {
        1.wait; 
        "2/5: Enviando glissando...".postln;
        sendGliss.(220, 880, 0.7, 3.0, 0.5);
    }.fork;
    
    // Enviar cluster después de 2 segundos
    {
        2.wait; 
        "3/5: Enviando cluster...".postln;
        sendCluster.(660, 150, 8, 0.6, 4.0);
    }.fork;
    
    // Enviar ruido después de 3 segundos
    {
        3.wait; 
        "4/5: Enviando ruido...".postln;
        sendNoise.(1000, 500, 0.4, 2.5);
    }.fork;
    
    // Enviar masa sonora después de 4 segundos
    {
        4.wait; 
        "5/5: Enviando masa sonora...".postln;
        sendSoundMass.(6, 0.5, 3.0);
        {
            3.wait;
            "=== Prueba básica completada ===".postln;
        }.fork;
    }.fork;
};

// --- Hacer funciones disponibles globalmente ---
// Estas se asignan al entorno global para que persistan después de ejecutar este bloque

"Exportando funciones a variables globales...".postln;

// Asignar funciones de composición
~scvTestBasicEvents = testBasicEvents;
~scvXenakisComposition = xenakisComposition;
~scvMetastasisPoints = metastasisPoints;
~scvPithopraktaGliss = pithopraktaGliss;
~scvStochasticClouds = stochasticClouds;
~scvNoiseTextures = noiseTextures;
~scvSpectralMasses = spectralMasses;
~scvRhythmicPatterns = rhythmicPatterns;

// Asignar funciones de envío
~scvSendPoint = sendPoint;
~scvSendGliss = sendGliss;
~scvSendCluster = sendCluster;
~scvSendNoise = sendNoise;
~scvSendSoundMass = sendSoundMass;

// Verificar que las funciones se exportaron correctamente
"Verificando funciones exportadas...".postln;
functionNames = ["scvTestBasicEvents", "scvSendPoint", "scvSendGliss", "scvSendCluster", "scvSendNoise", "scvSendSoundMass"];
allOk = true;

functionNames.do { |name|
    var globalVar = currentEnvironment.at(name.asSymbol);
    if (globalVar.isNil, {
        ("✗ ERROR: " ++ name ++ " no está disponible").postln;
        allOk = false;
    }, {
        ("✓ " ++ name ++ " exportada correctamente").postln;
    });
};

// --- Instrucciones de uso ---
if (allOk, {
    "".postln;
    "✓ ✓ ✓ TODAS LAS FUNCIONES CARGADAS EXITOSAMENTE ✓ ✓ ✓".postln;
    "".postln;
    "SC Score Visualizer - Código cargado exitosamente!".postln;
    "".postln;
    "Funciones disponibles:".postln;
    "~scvTestBasicEvents.() - Prueba básica de eventos".postln;
    "~scvXenakisComposition.() - Composición completa estilo Xenakis".postln;
    "~scvMetastasisPoints.() - Secuencia de puntos dispersos".postln;
    "~scvPithopraktaGliss.() - Glissandi convergentes/divergentes".postln;
    "~scvStochasticClouds.() - Nube de clusters".postln;
    "~scvNoiseTextures.() - Texturas de ruido evolutivas".postln;
    "~scvSpectralMasses.() - Masas sonoras complejas".postln;
    "~scvRhythmicPatterns.() - Patrones rítmicos complejos".postln;
    "".postln;
    "Funciones de envío individuales:".postln;
    "~scvSendPoint.(freq, amp, dur, ...)".postln;
    "~scvSendGliss.(startFreq, endFreq, amp, dur, ...)".postln;
    "~scvSendCluster.(centerFreq, freqSpread, numVoices, ...)".postln;
    "~scvSendNoise.(centerFreq, bandwidth, amp, dur, ...)".postln;
    "~scvSendSoundMass.(numComponents, amp, dur, ...)".postln;
    "".postln;
    "IMPORTANTE: Asegúrate de que el visualizador Rust esté ejecutándose en el puerto 57124".postln;
    "Para una prueba rápida, ejecuta: ~scvTestBasicEvents.()".postln;
    "".postln;
    "🎉 LISTO PARA USAR - Ejecuta: ~scvTestBasicEvents.() 🎉".postln;
}, {
    "".postln;
    "❌ ERROR: ALGUNAS FUNCIONES NO SE CARGARON CORRECTAMENTE".postln;
    "Intenta ejecutar el bloque completo nuevamente.".postln;
    "Asegúrate de seleccionar TODO el código entre ( ... )".postln;
});

) // Cierre del bloque principal
