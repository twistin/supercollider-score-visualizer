// demo_definitivo_que_funciona.scd
// Demo que garantizadamente funciona - sintaxis ultra-conservadora

(
// Todas las variables en UNA sola línea var
var addr, executeDemo, sendPoint, sendGliss, sendCluster;

"=== DEMO DEFINITIVO (SINTAXIS GARANTIZADA) ===".postln;

addr = NetAddr.new("127.0.0.1", 57122);

// Función simple para punto
sendPoint = { |freq, amp, dur|
    addr.sendMsg("/event", "point", freq, amp, dur);
    if(s.serverRunning, {
        {
            var env, sig;
            env = Env.perc(0.1, 0.5);
            sig = SinOsc.ar(freq, 0, amp * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    }, {
        ("Visual: punto " ++ freq ++ "Hz").postln;
    });
};

// Función simple para gliss
sendGliss = { |startFreq, endFreq, amp, dur|
    addr.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur);
    if(s.serverRunning, {
        {
            var freqLine, env, sig;
            freqLine = XLine.kr(startFreq, endFreq, dur);
            env = Env.linen(0.1, dur - 0.2, 0.1);
            sig = SinOsc.ar(freqLine, 0, amp * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    }, {
        ("Visual: gliss " ++ startFreq ++ "→" ++ endFreq).postln;
    });
};

// Función simple para cluster
sendCluster = { |centerFreq, spread, voices, amp, dur|
    addr.sendMsg("/event", "cluster", centerFreq, spread, voices, amp, dur);
    if(s.serverRunning, {
        {
            var freqs, env, sig;
            freqs = Array.fill(voices, { centerFreq + rrand(spread.neg, spread) });
            env = Env.linen(0.2, dur - 0.4, 0.2);
            sig = Mix.ar(freqs.collect({ |freq| SinOsc.ar(freq, 0, amp / voices) }));
            sig = sig * EnvGen.kr(env, doneAction: 2);
            Out.ar(0, sig.dup);
        }.play;
    }, {
        ("Visual: cluster " ++ centerFreq ++ "Hz").postln;
    });
};

executeDemo = {
    "🎨 Iniciando demo definitivo...".postln;
    
    if(s.serverRunning, {
        "🔊 Con audio habilitado".postln;
    }, {
        "👁️ Solo visual (para audio: s.boot; primero)".postln;
    });
    
    "1. Punto 440Hz...".postln;
    sendPoint.value(440, 0.8, 2.0);
    
    "2. Glissando 220→880Hz...".postln;
    sendGliss.value(220, 880, 0.7, 3.0);
    
    "3. Cluster 660Hz...".postln;
    sendCluster.value(660, 100, 4, 0.6, 3.0);
    
    "🎉 Demo completado".postln;
};

// Verificar servidor y ejecutar
if(s.serverRunning.not, {
    "⚠️ Intentando arrancar servidor...".postln;
    s.boot;
    s.waitForBoot({
        "✓ Servidor OK - demo con audio".postln;
        executeDemo.value;
    }, onFailure: {
        "✗ Solo visual".postln;
        executeDemo.value;
    });
}, {
    "✓ Servidor OK - demo con audio".postln;
    executeDemo.value;
});
)
