// demo_colores_ultra_simple.scd
// Demostración con declaraciones de variables individuales para máxima compatibilidad

(
"=== DEMO ULTRA SIMPLE (SINTAXIS SEGURA) ===".postln;

var addr;
var executeDemo;

addr = NetAddr.new("127.0.0.1", 57122);

executeDemo = {
    "🎨 Iniciando demo ultra simple...".postln;
    
    // Punto simple
    "1. Punto simple...".postln;
    addr.sendMsg("/event", "point", 440, 0.8, 2.0);
    if(s.serverRunning, {
        {
            var env = Env.perc(0.1, 0.5);
            var sig = SinOsc.ar(440, 0, 0.8 * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    }, {
        "Visual: punto 440Hz (sin audio)".postln;
    });
    
    // Glissando simple
    "2. Glissando simple...".postln;
    addr.sendMsg("/event", "gliss", 220, 880, 0.7, 3.0);
    if(s.serverRunning, {
        {
            var freqLine = XLine.kr(220, 880, 3.0);
            var env = Env.linen(0.1, 2.8, 0.1);
            var sig = SinOsc.ar(freqLine, 0, 0.7 * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    }, {
        "Visual: gliss 220→880Hz (sin audio)".postln;
    });
    
    // Cluster simple
    "3. Cluster simple...".postln;
    addr.sendMsg("/event", "cluster", 660, 100, 4, 0.6, 3.0);
    if(s.serverRunning, {
        {
            var freqs = [620, 660, 700, 740];
            var env = Env.linen(0.2, 2.6, 0.2);
            var sig = Mix.ar(freqs.collect({ |freq| SinOsc.ar(freq, 0, 0.15) }));
            sig = sig * EnvGen.kr(env, doneAction: 2);
            Out.ar(0, sig.dup);
        }.play;
    }, {
        "Visual: cluster 660Hz (sin audio)".postln;
    });
    
    "🎉 Demo ultra simple completado".postln;
    if(s.serverRunning, {
        "Deberías ver Y escuchar 3 eventos".postln;
    }, {
        "Solo visualización (para audio: s.boot; primero)".postln;
    });
};

// Verificar servidor y ejecutar
if(s.serverRunning.not, {
    "⚠️  Servidor de audio no funcionando. Intentando arrancar...".postln;
    s.boot;
    s.waitForBoot({
        "✓ Servidor arrancado - ejecutando demo...".postln;
        executeDemo.value;
    }, onFailure: {
        "✗ No se pudo arrancar servidor - solo visual".postln;
        executeDemo.value;
    });
}, {
    "✓ Servidor funcionando - ejecutando demo completo...".postln;
    executeDemo.value;
});
)
