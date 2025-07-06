// SC Score Visualizer - MÉTODO FUNCIÓN HÍBRIDO (MÁS CONFIABLE)
// Puerto OSC: 57124

// =====================================================================
// INSTRUCCIONES SIMPLES:
// =====================================================================
// 1. EJECUTA TODO ESTE CÓDIGO (Cmd+A, luego Cmd+Return)
// 2. ESPERA el mensaje "LISTO"
// 3. USA: scvTest() - SIN punto, sintaxis normal de función
// =====================================================================

(
// ========== DECLARACIÓN DE VARIABLES (DEBE IR PRIMERO) ==========
var osc;
"".postln;
"FUNCIONES PRINCIPALES DISPONIBLES:".postln;
"  • scvTest()          - Prueba básica (RECOMENDADO EMPEZAR AQUÍ)".postln;
"  • scvXenakis()       - Composición estilo Xenakis".postln;
"  • scvQuick()         - Demo rápido".postln;
"".postln;
"IMPORTANTE: Usa exactamente estos nombres: scvTest()".postln;
"".postln;
"FUNCIONES DE ENVÍO DIRECTO:".postln;
"  • sendPoint(freq, amp, dur)".postln;
"  • sendGliss(startFreq, endFreq, amp, dur)".postln;
"  • sendCluster(center, spread, voices, amp, dur)".postln;
"  • sendNoise(center, bandwidth, amp, dur)".postln;
"  • sendMass(components, amp, dur)".postln;
"".postln;
"PRUEBA AHORA: scvTest()".postln;// INSTRUCCIONES SIMPLES:
// =====================================================================
// 1. EJECUTA TODO ESTE CÓDIGO (Cmd+A, luego Cmd+Return)
// 2. ESPERA el mensaje "LISTO"
// 3. USA: scvTest() - SIN punto, sintaxis normal de función
// =====================================================================e Visualizer - MÉTODO PROXYSPACE CORREGIDO (MÁS CONFIABLE)
// Puerto OSC: 57124

// =====================================================================
// INSTRUCCIONES SIMPLES:
// =====================================================================
// 1. EJECUTA TODO ESTE CÓDIGO (Cmd+A, luego Cmd+Return)
// 2. ESPERA el mensaje "LISTO"
// 3. USA: scvTest() - NOTA EL PUNTO ANTES DEL PARÉNTESIS
// =====================================================================

(
// ========== DECLARACIÓN DE VARIABLES (DEBE IR PRIMERO) ==========
var osc;

// ========== INICIALIZACIÓN ==========
"LOADING Cargando SC Score Visualizer con ProxySpace...".postln;

// Configuración OSC (variable local, NO ProxySpace porque NetAddr no es compatible)
osc = NetAddr.new("127.0.0.1", 57124);
"OSC configurado en puerto 57124".postln;

// ========== FUNCIONES DE ENVÍO (Variables Globales, NO ProxySpace) ==========

// Estas funciones NO pueden ser ProxySpace porque contienen postln
// ProxySpace es solo para síntesis de audio
sendPoint = { |freq=440, amp=0.5, dur=1.0|
    osc.sendMsg("/event", "point", freq, amp, dur, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
    ("OK Punto: " ++ freq ++ "Hz, " ++ dur ++ "s").postln;
};

sendGliss = { |startFreq=220, endFreq=880, amp=0.7, dur=3.0|
    osc.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur, 0, 0.1, 0.1, 0.7, 0.2, 0.1, 120);
    ("OK Glissando: " ++ startFreq ++ " → " ++ endFreq ++ "Hz").postln;
};

sendCluster = { |centerFreq=500, spread=100, voices=6, amp=0.6, dur=2.0|
    osc.sendMsg("/event", "cluster", centerFreq, spread, voices, amp, dur, 0.1, 0.1, 0.8, 0.3, 0.4, 180);
    ("OK Cluster: " ++ centerFreq ++ "Hz ±" ++ spread ++ ", " ++ voices ++ " voces").postln;
};

sendNoise = { |centerFreq=1000, bandwidth=500, amp=0.4, dur=2.0|
    osc.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur, 1.0, 0, 0.1, 0.1, 0.6, 0.8, 0.4, 60);
    ("OK Ruido: " ++ centerFreq ++ "Hz ±" ++ bandwidth).postln;
};

sendMass = { |components=4, amp=0.5, dur=3.0|
    var freqs = Array.fill(components, { 200 + 1500.rand });
    var amps = Array.fill(components, { 0.3 + 0.7.rand });
    var profile = [freqs, amps].flop.flatten;
    var args = ["mass", components, amp, dur, 0.3, 0.2, 0.1, 0.1, 0.9, 0.5, 0.7, 280] ++ profile;
    osc.sendMsg("/event", *args);
    ("OK Masa sonora: " ++ components ++ " componentes").postln;
};

// ========== DEMOS Y PRUEBAS (ProxySpace) ==========

scvTest = {
    "TEST === PRUEBA BÁSICA (5 eventos) ===".postln;

    sendPoint.(440, 0.7, 2.0);

    { 2.5.wait; sendGliss.(220, 880, 0.8, 3.0); }.fork;
    { 4.wait; sendCluster.(660, 150, 8, 0.6, 4.0); }.fork;
    { 6.wait; sendNoise.(1000, 500, 0.5, 2.5); }.fork;
    { 8.wait; sendMass.(6, 0.6, 3.0);
      { 3.5.wait; "OK === PRUEBA COMPLETADA ===".postln; }.fork;
    }.fork;
};

scvXenakis = {
    "MUSIC === COMPOSICIÓN ESTILO XENAKIS ===".postln;

    // Puntos dispersos (Metastasis)
    {
        20.do { |i|
            var freq = 200 + (2000.rand);
            var dur = 0.5 + 2.5.rand;
            var amp = 0.3 + 0.4.rand;
            sendPoint.(freq, amp, dur);
            (0.2 + 0.8.rand).wait;
        };
    }.fork;

    // Glissandos cruzados (Pithoprakta)
    { 1.wait;
        8.do { |i|
            var startFreq = 150 + (300.rand);
            var endFreq = 800 + (1200.rand);
            var dur = 2 + 4.rand;
            var amp = 0.4 + 0.3.rand;
            sendGliss.(startFreq, endFreq, amp, dur);
            (1.5 + 2.5.rand).wait;
        };
    }.fork;

    // Clusters evolutivos (Terretektorh)
    { 3.wait;
        6.do { |i|
            var centerFreq = 300 + (800.rand);
            var spread = 50 + (200.rand);
            var voices = 4 + 8.rand;
            var dur = 3 + 5.rand;
            var amp = 0.5 + 0.3.rand;
            sendCluster.(centerFreq, spread, voices, amp, dur);
            (2 + 3.rand).wait;
        };
    }.fork;

    // Masas sonoras finales (Persephassa)
    { 8.wait;
        4.do { |i|
            var components = 6 + 8.rand;
            var dur = 4 + 6.rand;
            var amp = 0.6 + 0.3.rand;
            sendMass.(components, amp, dur);
            (3 + 4.rand).wait;
        };
    }.fork;
};

scvQuick = {
    "QUICK === DEMO RÁPIDO ===".postln;
    sendPoint.(220, 0.8, 1);
    { 1.5.wait; sendGliss.(440, 880, 0.7, 2); }.fork;
    { 3.wait; sendCluster.(500, 200, 6, 0.6, 2); }.fork;
};

// ========== MENSAJE FINAL ==========

0.5.wait;

"".postln;
"LISTO PARA USAR - READY TO USE".postln;
"".postln;
"FUNCIONES PRINCIPALES DISPONIBLES:".postln;
"  • scvTest()        - Prueba básica (RECOMENDADO EMPEZAR AQUÍ)".postln;
"  • scvXenakis()     - Composición estilo Xenakis".postln;
"  • scvQuick()       - Demo rápido".postln;
"".postln;
"IMPORTANTE: Usa exactamente estos nombres con PUNTO: scvTest()".postln;
"".postln;
"FUNCIONES DE ENVÍO DIRECTO:".postln;
"  • sendPoint.(freq, amp, dur)".postln;
"  • sendGliss.(startFreq, endFreq, amp, dur)".postln;
"  • sendCluster.(center, spread, voices, amp, dur)".postln;
"  • sendNoise.(center, bandwidth, amp, dur)".postln;
"  • sendMass.(components, amp, dur)".postln;
"".postln;
"PRUEBA AHORA: scvTest()".postln;
"Puerto OSC: 57124".postln;
"Asegúrate de que el visualizador Rust esté corriendo".postln;
"".postln;

// ========== PRUEBA FINAL ==========
"CHECKING Probando acceso a funciones globales...".postln;
if(scvTest.isFunction, {
    "OK scvTest() está lista para usar".postln;
}, {
    "ERROR: scvTest no se cargó".postln;
});

) // Fin del bloque principal

// =====================================================================
// GUÍA DE USO RÁPIDO:
// =====================================================================
//
// 1. Si todo salió bien, deberías ver "OK scvTest() está lista para usar"
// 2. Ejecuta: scvTest()
// 3. Sintaxis normal de función - SIN punto antes del paréntesis
// 4. Si hay errores, vuelve a ejecutar todo el bloque ( ... ) de arriba
//
// =====================================================================
