// demo_colores_avanzado_auto.scd
// Demostración completa de colores y efectos visuales CON AUDIO
// VERSION CON ARRANQUE AUTOMÁTICO DEL SERVIDOR

(
"=== DEMO AVANZADO DE COLORES Y EFECTOS (AUDIOVISUAL AUTO) ===".postln;

// Variables
var addr, sendAVPoint, sendAVGliss, sendAVCluster;
var sendAVNoise, sendAVMass, executeDemo;

// Crear conexión OSC
addr = NetAddr.new("127.0.0.1", 57122);

// Función para enviar punto con audio y visual
sendAVPoint = { |freq=440, amp=0.5, dur=1.0, attack=0.1, decay=0.1, density=0.8, texture=0.3, spread=0.4, hue=0|
    addr.sendMsg("/event", "point", freq, amp, dur, attack, decay, density, texture, spread, hue);
    if(s.serverRunning, {
        { 
            var env = Env.perc(attack, decay);
            var sig = SinOsc.ar(freq, 0, amp * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    }, {
        ("Enviado visual para punto freq=" ++ freq ++ " (sin audio - servidor no funciona)").postln;
    });
};

// Función para enviar glissando con audio y visual
sendAVGliss = { |startFreq=100, endFreq=400, amp=0.8, dur=3.0, curvature=0.3, density=0.8, texture=0.2, spread=0.3, hue=180|
    addr.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur, curvature, density, texture, spread, hue);
    if(s.serverRunning, {
        {
            var freqLine = XLine.kr(startFreq, endFreq, dur);
            var env = Env.linen(0.1, dur - 0.2, 0.1);
            var sig = SinOsc.ar(freqLine, 0, amp * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    }, {
        ("Enviado visual para gliss " ++ startFreq ++ "→" ++ endFreq ++ " (sin audio)").postln;
    });
};

// Función para enviar cluster con audio y visual
sendAVCluster = { |centerFreq=500, freqSpread=100, numVoices=4, amp=0.6, dur=4.0, density=0.5, texture=0.6, spread=0.8, hue=300|
    addr.sendMsg("/event", "cluster", centerFreq, freqSpread, numVoices, amp, dur, density, texture, spread, hue);
    if(s.serverRunning, {
        {
            var freqs = Array.fill(numVoices, { centerFreq + rrand(freqSpread.neg, freqSpread) });
            var env = Env.linen(0.2, dur - 0.4, 0.2);
            var sig = Mix.ar(freqs.collect({ |freq| SinOsc.ar(freq, 0, amp / numVoices) }));
            sig = sig * EnvGen.kr(env, doneAction: 2);
            Out.ar(0, sig.dup);
        }.play;
    }, {
        ("Enviado visual para cluster freq=" ++ centerFreq ++ " (sin audio)").postln;
    });
};

// Función para enviar ruido con audio y visual
sendAVNoise = { |centerFreq=1000, bandwidth=200, amp=0.7, dur=3.0, density=0.8, texture=0.9, spread=0.7, filter=0.5, hue=120|
    addr.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur, density, texture, spread, filter, hue);
    if(s.serverRunning, {
        {
            var env = Env.linen(0.3, dur - 0.6, 0.3);
            var sig = WhiteNoise.ar(amp);
            sig = BPF.ar(sig, centerFreq, bandwidth / centerFreq);
            sig = sig * EnvGen.kr(env, doneAction: 2);
            Out.ar(0, sig.dup);
        }.play;
    }, {
        ("Enviado visual para ruido freq=" ++ centerFreq ++ " (sin audio)").postln;
    });
};

// Función principal de demo
executeDemo = {
    "🎨 Iniciando demostración audiovisual de colores y efectos...".postln;

    // 1. Puntos con diferentes frecuencias (colores rojos/naranjas)
    "1. Puntos coloridos por frecuencia...".postln;
    sendAVPoint.value(220, 0.9, 2.0, 0.1, 0.1, 0.8, 0.3, 0.4, 0);
    sendAVPoint.value(440, 0.8, 2.0, 0.1, 0.1, 0.9, 0.5, 0.2, 60);
    sendAVPoint.value(880, 0.7, 2.0, 0.1, 0.1, 0.7, 0.8, 0.6, 120);

    "2. Glissandos con colores estructurales...".postln;
    sendAVGliss.value(100, 400, 0.8, 3.0, 0.3, 0.8, 0.2, 0.3, 180);
    sendAVGliss.value(200, 800, 0.9, 3.0, -0.5, 0.9, 0.4, 0.1, 240);

    "3. Clusters con densidades dinámicas...".postln;
    sendAVCluster.value(500, 100, 4, 0.6, 4.0, 0.5, 0.6, 0.8, 300);
    sendAVCluster.value(700, 200, 8, 0.8, 4.0, 0.8, 0.4, 0.5, 0);
    sendAVCluster.value(900, 300, 12, 0.9, 4.0, 0.9, 0.7, 0.9, 60);

    "4. Texturas de ruido coloridas...".postln;
    sendAVNoise.value(1000, 200, 0.7, 3.0, 0.8, 0.2, 0.9, 0.7, 120);
    sendAVNoise.value(1500, 500, 0.8, 3.0, 1.2, -0.3, 0.8, 0.9, 180);

    "5. Masas sonoras multi-color...".postln;
    // Para las masas sonoras, usamos múltiples clusters simultáneos
    if(s.serverRunning, {
        {
            addr.sendMsg("/event", "mass", 3, 0.8, 4.0, 0.7, 0.4, 0.9, 0.8, 0.6, 240, 440, 1.0, 660, 0.7, 880, 0.5);
            var freqs = [440, 660, 880];
            var amps = [1.0, 0.7, 0.5] * 0.3;
            var env = Env.linen(0.3, 4.0 - 0.6, 0.3);
            var sig = Mix.ar(freqs.collect({ |freq, i| SinOsc.ar(freq, 0, amps[i]) }));
            sig = sig * EnvGen.kr(env, doneAction: 2);
            Out.ar(0, sig.dup);
        }.play;
        
        {
            addr.sendMsg("/event", "mass", 5, 0.9, 4.0, 0.8, 0.5, 0.95, 0.9, 0.8, 300, 200, 1.0, 350, 0.8, 550, 0.6, 750, 0.4, 950, 0.3);
            var freqs = [200, 350, 550, 750, 950];
            var amps = [1.0, 0.8, 0.6, 0.4, 0.3] * 0.25;
            var env = Env.linen(0.3, 4.0 - 0.6, 0.3);
            var sig = Mix.ar(freqs.collect({ |freq, i| SinOsc.ar(freq, 0, amps[i]) }));
            sig = sig * EnvGen.kr(env, doneAction: 2);
            Out.ar(0, sig.dup);
        }.play;
    }, {
        addr.sendMsg("/event", "mass", 3, 0.8, 4.0, 0.7, 0.4, 0.9, 0.8, 0.6, 240, 440, 1.0, 660, 0.7, 880, 0.5);
        addr.sendMsg("/event", "mass", 5, 0.9, 4.0, 0.8, 0.5, 0.95, 0.9, 0.8, 300, 200, 1.0, 350, 0.8, 550, 0.6, 750, 0.4, 950, 0.3);
        "Enviadas masas sonoras (solo visual - sin audio)".postln;
    });

    "🎉 ¡Demostración completa!".postln;
    if(s.serverRunning, {
        "Deberías escuchar Y ver una explosión de colores y efectos.".postln;
    }, {
        "Solo visualización (para audio, ejecuta s.boot; primero)".postln;
    });
};

// Verificar servidor y ejecutar
if(s.serverRunning.not, {
    "⚠️  Servidor de audio no está funcionando.".postln;
    "Intentando arrancar servidor...".postln;
    s.boot;
    s.waitForBoot({
        "✓ Servidor arrancado - ejecutando demo completo...".postln;
        executeDemo.value;
    }, onFailure: {
        "✗ No se pudo arrancar el servidor - solo demostración visual".postln;
        executeDemo.value;
    });
}, {
    "✓ Servidor funcionando - ejecutando demo completo...".postln;
    executeDemo.value;
});
)
