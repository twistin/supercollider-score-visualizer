// SC Score Visualizer - VERSIÓN AUDIOVISUAL
// Este script produce SONIDO y VISUALES al mismo tiempo.

(
var osc;

"LOADING Cargando script AUDIOVISUAL...".postln;

// Iniciar el servidor de audio si no está activo
s.waitForBoot {
    "Server booted".postln;

    // Configuración OSC
    osc = NetAddr.new("127.0.0.1", 57124);
    "OSC configurado en puerto 57124".postln;

    // ========== SINTETIZADORES (DEFINICIONES DE SONIDO) ==========

    SynthDef(\point, {
        |out=0, freq=440, amp=0.1, dur=1.0|
        var env = Env.perc(0.01, dur);
        var sig = SinOsc.ar(freq, 0, amp) * EnvGen.kr(env, doneAction: 2);
        Out.ar(out, Pan2.ar(sig));
    }).add;

    SynthDef(\gliss, {
        |out=0, startFreq=220, endFreq=880, amp=0.1, dur=2.0|
        var env = Env.linen(0.01, dur, 0.1, curve: 'linear');
        var freq = XLine.kr(startFreq, endFreq, dur);
        var sig = Saw.ar(freq, amp) * EnvGen.kr(env, doneAction: 2);
        Out.ar(out, Pan2.ar(sig));
    }).add;

    SynthDef(\cluster, {
        |out=0, centerFreq=500, spread=100, amp=0.1, dur=3.0| // 'voices' ya no es un argumento aquí
        var numVoices = 8; // Se usa un número fijo de voces para el audio para evitar el error
        var env = Env.linen(0.5, dur, 0.5);
        var freqs = Array.fill(numVoices, { centerFreq + spread.rand2 });
        var sig = Mix.fill(numVoices, { |i|
            SinOsc.ar(freqs[i], 0, 1.0 / numVoices)
        }) * amp * EnvGen.kr(env, doneAction: 2);
        Out.ar(out, Pan2.ar(sig));
    }).add;

    // ========== FUNCIONES AUDIOVISUALES (GLOBALES) ==========

    ~avSendPoint = { |freq=440, amp=0.5, dur=1.0|
        // Visual
        osc.sendMsg("/event", "point", freq, amp, dur);
        // Sonido
        Synth(\point, [\freq, freq, \amp, amp * 0.2, \dur, dur]);
        "AV Point: Freq=%, Amp=%".format(freq, amp).postln;
    };

    ~avSendGliss = { |startFreq=220, endFreq=880, amp=0.5, dur=2.0|
        // Visual
        osc.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur, 0.0);
        // Sonido
        Synth(\gliss, [\startFreq, startFreq, \endFreq, endFreq, \amp, amp * 0.2, \dur, dur]);
        "AV Gliss: Freqs=%-% Hz".format(startFreq, endFreq).postln;
    };

    ~avSendCluster = { |centerFreq=500, spread=100, voices=8, amp=0.5, dur=3.0|
        // Visual
        osc.sendMsg("/event", "cluster", centerFreq, spread, voices, amp, dur);
        // Sonido
        Synth(\cluster, [\centerFreq, centerFreq, \spread, spread, \voices, voices, \amp, amp * 0.2, \dur, dur]);
        "AV Cluster: Center=%, Voices=%".format(centerFreq, voices).postln;
    };


    // ========== DEMO AUDIOVISUAL ==========

    ~avTest = {
        "TEST: Iniciando prueba AUDIOVISUAL".postln;
        ~avSendPoint.value(440, 0.8, 1.0);
        { 1.wait; ~avSendGliss.value(220, 880, 0.6, 2.5); }.fork;
        { 3.wait; ~avSendCluster.value(600, 300, 12, 0.5, 4.0); }.fork;
        { 7.wait; "TEST: Prueba Audiovisual Completada".postln; }.fork;
    };

    0.2.wait;
    "\n✅ LISTO PARA PRUEBA AUDIOVISUAL. Ejecuta: ~avTest.value".postln;
};

)