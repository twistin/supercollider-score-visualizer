// SC Score Visualizer - VERSIÓN ROBUSTA (Variables de Entorno)
// Puerto OSC: 57124

// =====================================================================
// INSTRUCCIONES (MÉTODO ROBUSTO):
// =====================================================================
// 1. RECOMPILA LA LIBRERÍA: En SuperCollider, ve a Language > Recompile Class Library (o Cmd+Shift+L)
// 2. EJECUTA TODO ESTE CÓDIGO: Selecciona todo (Cmd+A) y ejecuta (Cmd+Return)
// 3. ESPERA el mensaje "LISTO"
// 4. USA: ~scvTest.value
// =====================================================================

(
// ========== DECLARACIÓN DE VARIABLES (DEBE IR PRIMERO) ==========
var osc;

// ========== INICIALIZACIÓN ==========
"LOADING Cargando SC Score Visualizer...".postln;

// Configuración OSC (variable local)
osc = NetAddr.new("127.0.0.1", 57124);
"OSC configurado en puerto 57124".postln;

// ========== FUNCIONES DE ENVÍO (Variables de Entorno ~) ==========

~sendPoint = { |freq=440, amp=0.5, dur=1.0|
    osc.sendMsg("/event", "point", freq, amp, dur, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
    ("OK Punto enviado").postln;
};

~sendGliss = { |startFreq=220, endFreq=880, amp=0.7, dur=3.0|
    osc.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur, 0, 0.1, 0.1, 0.7, 0.2, 0.1, 120);
    ("OK Glissando enviado").postln;
};

~sendCluster = { |centerFreq=500, spread=100, voices=6, amp=0.6, dur=2.0|
    osc.sendMsg("/event", "cluster", centerFreq, spread, voices, amp, dur, 0.1, 0.1, 0.8, 0.3, 0.4, 180);
    ("OK Cluster enviado").postln;
};

~sendNoise = { |centerFreq=1000, bandwidth=500, amp=0.4, dur=2.0|
    osc.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur, 1.0, 0, 0.1, 0.1, 0.6, 0.8, 0.4, 60);
    ("OK Ruido enviado").postln;
};

~sendMass = { |components=4, amp=0.5, dur=3.0|
    var freqs = Array.fill(components, { 200 + 1500.rand });
    var amps = Array.fill(components, { 0.3 + 0.7.rand });
    var profile = [freqs, amps].flop.flatten;
    var args = ["mass", components, amp, dur, 0.3, 0.2, 0.1, 0.1, 0.9, 0.5, 0.7, 280] ++ profile;
    osc.sendMsg("/event", *args);
    ("OK Masa sonora enviada").postln;
};

// ========== DEMOS Y PRUEBAS ==========

~scvTest = {
    "TEST Iniciando prueba básica".postln;

    ~sendPoint.value(440, 0.7, 2.0);

    { 2.5.wait; ~sendGliss.value(220, 880, 0.8, 3.0); }.fork;
    { 4.wait; ~sendCluster.value(660, 150, 8, 0.6, 4.0); }.fork;
    { 6.wait; ~sendNoise.value(1000, 500, 0.5, 2.5); }.fork;
    { 8.wait; ~sendMass.value(6, 0.6, 3.0);
      { 3.5.wait; "TEST Prueba completada".postln; }.fork;
    }.fork;
};

~scvXenakis = {
    "MUSIC Iniciando composición estilo Xenakis".postln;

    // Puntos dispersos
    {
        20.do { |i|
            var freq = 200 + (2000.rand);
            var dur = 0.5 + 2.5.rand;
            var amp = 0.3 + 0.4.rand;
            ~sendPoint.value(freq, amp, dur);
            (0.2 + 0.8.rand).wait;
        };
    }.fork;

    // Glissandos cruzados
    { 1.wait;
        8.do { |i|
            var startFreq = 150 + (300.rand);
            var endFreq = 800 + (1200.rand);
            var dur = 2 + 4.rand;
            var amp = 0.4 + 0.3.rand;
            ~sendGliss.value(startFreq, endFreq, amp, dur);
            (1.5 + 2.5.rand).wait;
        };
    }.fork;

    // Clusters evolutivos
    { 3.wait;
        6.do { |i|
            var centerFreq = 300 + (800.rand);
            var spread = 50 + (200.rand);
            var voices = 4 + 8.rand;
            var dur = 3 + 5.rand;
            var amp = 0.5 + 0.3.rand;
            ~sendCluster.value(centerFreq, spread, voices, amp, dur);
            (2 + 3.rand).wait;
        };
    }.fork;

    // Masas sonoras finales
    { 8.wait;
        4.do { |i|
            var components = 6 + 8.rand;
            var dur = 4 + 6.rand;
            var amp = 0.6 + 0.3.rand;
            ~sendMass.value(components, amp, dur);
            (3 + 4.rand).wait;
        };
    }.fork;
};

~scvQuick = {
    "QUICK Demo rápido iniciado".postln;
    ~sendPoint.value(220, 0.8, 1);
    { 1.5.wait; ~sendGliss.value(440, 880, 0.7, 2); }.fork;
    { 3.wait; ~sendCluster.value(500, 200, 6, 0.6, 2); }.fork;
};

// ========== MENSAJE FINAL ==========

"".postln;
"LISTO PARA USAR - READY TO USE".postln;
"".postln;
"FUNCIONES PRINCIPALES:".postln;
"  • ~scvTest.value          - Prueba básica".postln;
"  • ~scvXenakis.value       - Composición Xenakis".postln;
"  • ~scvQuick.value         - Demo rápido".postln;
"".postln;
"FUNCIONES DE ENVÍO:".postln;
"  • ~sendPoint.value(freq, amp, dur)".postln;
"  • ~sendGliss.value(startFreq, endFreq, amp, dur)".postln;
"  • ~sendCluster.value(center, spread, voices, amp, dur)".postln;
"  • ~sendNoise.value(center, bandwidth, amp, dur)".postln;
"  • ~sendMass.value(components, amp, dur)".postln;
"".postln;
"PRUEBA AHORA: ~scvTest.value".postln;
"Puerto OSC: 57124".postln;
"".postln;

// ========== VERIFICACIÓN FINAL ==========
"CHECKING Verificando funciones...".postln;
if(~scvTest.isFunction, {
    "OK ~scvTest.value lista para usar".postln;
}, {
    "ERROR: ~scvTest no se cargó correctamente".postln;
});

) // Fin del bloque principal

// =====================================================================
// GUÍA DE USO (MÉTODO ROBUSTO):
// =====================================================================
//
// 1. Si ves "OK ~scvTest.value lista para usar", todo está bien
// 2. Ejecuta: ~scvTest.value
// 3. Si hay errores, RECOMPILA (Cmd+Shift+L) y vuelve a ejecutar todo
//
// =====================================================================
