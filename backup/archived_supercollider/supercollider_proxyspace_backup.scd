// SC Score Visualizer - MÉTODO PROXYSPACE (MÁS CONFIABLE)
// Puerto OSC: 57124

// =====================================================================
// INSTRUCCIONES SIMPLES:
// =====================================================================
// 1. EJECUTA TODO ESTE CÓDIGO (Cmd+A, luego Cmd+Return)
// 2. ESPERA el mensaje "LISTO"
// 3. USA: ~scvTest.() - NOTA EL PUNTO ANTES DEL PARÉNTESIS
// =====================================================================

"LOADING Cargando SC Score Visualizer con ProxySpace...".postln;

// Configuración OSC (variable global, NO ProxySpace)
var osc = NetAddr.new("127.0.0.1", 57124);
"OSC configurado en puerto 57124".postln;

// ========== FUNCIONES DE ENVÍO ==========

~sendPoint = { |freq=440, amp=0.5, dur=1.0|
    osc.sendMsg("/event", "point", freq, amp, dur, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
    ("OK Punto: " ++ freq ++ "Hz, " ++ dur ++ "s").postln;
};

~sendGliss = { |startFreq=220, endFreq=880, amp=0.7, dur=3.0|
    ~osc.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur, 0, 0.1, 0.1, 0.7, 0.2, 0.1, 120);
    ("OK Glissando: " ++ startFreq ++ " → " ++ endFreq ++ "Hz").postln;
};

~sendCluster = { |centerFreq=500, spread=100, voices=6, amp=0.6, dur=2.0|
    ~osc.sendMsg("/event", "cluster", centerFreq, spread, voices, amp, dur, 0.1, 0.1, 0.8, 0.3, 0.4, 180);
    ("OK Cluster: " ++ centerFreq ++ "Hz ±" ++ spread ++ ", " ++ voices ++ " voces").postln;
};

~sendNoise = { |centerFreq=1000, bandwidth=500, amp=0.4, dur=2.0|
    ~osc.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur, 1.0, 0, 0.1, 0.1, 0.6, 0.8, 0.4, 60);
    ("OK Ruido: " ++ centerFreq ++ "Hz ±" ++ bandwidth).postln;
};

~sendMass = { |components=4, amp=0.5, dur=3.0|
    var freqs = Array.fill(components, { 200 + 1500.rand });
    var amps = Array.fill(components, { 0.3 + 0.7.rand });
    var profile = [freqs, amps].flop.flatten;
    var args = ["mass", components, amp, dur, 0.3, 0.2, 0.1, 0.1, 0.9, 0.5, 0.7, 280] ++ profile;
    ~osc.sendMsg("/event", *args);
    ("OK Masa sonora: " ++ components ++ " componentes").postln;
};

// ========== DEMOS Y PRUEBAS ==========

~scvTest = {
    "🧪 === PRUEBA BÁSICA (5 eventos) ===".postln;
    
    ~sendPoint.(440, 0.7, 2.0);
    
    { 2.5.wait; ~sendGliss.(220, 880, 0.8, 3.0); }.fork;
    { 4.wait; ~sendCluster.(660, 150, 8, 0.6, 4.0); }.fork;
    { 6.wait; ~sendNoise.(1000, 500, 0.5, 2.5); }.fork;
    { 8.wait; ~sendMass.(6, 0.6, 3.0); 
      { 3.5.wait; "OK === PRUEBA COMPLETADA ===".postln; }.fork; 
    }.fork;
};

~scvXenakis = {
    "🎼 === COMPOSICIÓN ESTILO XENAKIS ===".postln;
    
    // Puntos dispersos (Metastasis)
    {
        20.do { |i|
            var freq = 100 + (3000 * (i/20)) + 100.rand2;
            var amp = 0.3 + 0.4.rand;
            var dur = 0.5 + 1.5.rand;
            ~sendPoint.(freq, amp, dur);
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
            ~sendGliss.(start, end, 0.5 + 0.3.rand, dur);
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
            ~sendCluster.(center, spread, voices, 0.4 + 0.3.rand, 3.0 + 2.0.rand);
            (1.0 + 2.0.rand).wait;
        };
    }.fork;
    
    "🎵 Duración total: ~30 segundos".postln;
};

~scvQuick = {
    "⚡ === DEMO RÁPIDO ===".postln;
    ~sendPoint.(330, 0.6, 1.0);
    { 1.2.wait; ~sendGliss.(440, 880, 0.7, 2.0); }.fork;
    { 2.5.wait; ~sendCluster.(550, 100, 6, 0.5, 2.5); }.fork;
};

// ========== ALIASES PARA FACILIDAD DE USO ==========

~scvSendPoint = ~sendPoint;
~scvSendGliss = ~sendGliss;
~scvSendCluster = ~sendCluster;
~scvSendNoise = ~sendNoise;
~scvSendMass = ~sendMass;

// ========== MENSAJE FINAL ==========

"".postln;
"LISTO PARA USAR - READY TO USE".postln;
"".postln;
"📋 FUNCIONES PRINCIPALES (usa EXACTAMENTE esta sintaxis):".postln;
"  • ~scvTest.()        - Prueba básica (RECOMENDADO)".postln;
"  • ~scvXenakis.()     - Composición estilo Xenakis".postln;
"  • ~scvQuick.()       - Demo rápido".postln;
"".postln;
"📦 FUNCIONES DE ENVÍO DIRECTO:".postln;
"  • ~scvSendPoint.(freq, amp, dur)".postln;
"  • ~scvSendGliss.(startFreq, endFreq, amp, dur)".postln;
"  • ~scvSendCluster.(center, spread, voices, amp, dur)".postln;
"  • ~scvSendNoise.(center, bandwidth, amp, dur)".postln;
"  • ~scvSendMass.(components, amp, dur)".postln;
"".postln;
"PRUEBA AHORA: ~scvTest.()".postln;
"Puerto OSC: 57124".postln;
"Asegúrate de que el visualizador Rust esté corriendo".postln;
"".postln;

// Verificación final
if(~scvTest.notNil, {
    "OK ~scvTest.() está lista para usar".postln;
}, {
    "ERROR: ~scvTest no se cargó".postln;
});

// =====================================================================
// 🎯 CÓMO USAR (DESPUÉS DE EJECUTAR TODO LO ANTERIOR):
// =====================================================================
//
// EJECUTA cualquiera de estas líneas:
// ~scvTest.()      // Prueba básica
// ~scvXenakis.()   // Composición avanzada
// ~scvQuick.()     // Demo rápido
//
// ⚠️  NOTA IMPORTANTE: 
// - USA ~scvTest.() NO scvTest()
// - USA ~scvXenakis.() NO scvXenakis()
// - USA ~scvQuick.() NO scvQuick()
//
// El símbolo ~ y el punto . son OBLIGATORIOS
// =====================================================================
