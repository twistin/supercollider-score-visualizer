// demo_sincronizado_perfecto.scd
// DEMO AUDIOVISUAL CON SINCRONIZACIÓN PERFECTA
// Corrige todos los problemas: dirección, duración, artefactos visuales

(
"=== DEMO SINCRONIZACIÓN AUDIOVISUAL PERFECTA ===".postln;

// Función principal que maneja el servidor
var executeDemo;

executeDemo = {
    Routine({
        var addr, sendPerfectGliss, sendPerfectPoint, sendPerfectCluster, sendPerfectNoise;
        
        addr = NetAddr.new("127.0.0.1", 57122);
        
        // GLISSANDO PERFECTAMENTE SINCRONIZADO
        sendPerfectGliss = { |startFreq=100, endFreq=400, amp=0.8, dur=3.0|
            var visualDur, audioDur;
            
            // Duración IDÉNTICA para audio y visual
            visualDur = dur;
            audioDur = dur;
            
            // Enviar evento visual
            addr.sendMsg("/event", "gliss", startFreq, endFreq, amp, visualDur);
            
            if(s.serverRunning, {
                {
                    // Audio con EXACTAMENTE la misma duración y dirección
                    var freqLine = XLine.kr(startFreq, endFreq, audioDur);
                    var env = Env.linen(0.05, audioDur - 0.1, 0.05); // Mínima envolvente
                    var sig = SinOsc.ar(freqLine, 0, amp * EnvGen.kr(env, doneAction: 2));
                    Out.ar(0, sig.dup);
                }.play;
                ("🎯 GLISS SINCRONIZADO: " ++ startFreq ++ "→" ++ endFreq ++ " dur=" ++ dur).postln;
            }, {
                ("👁️  Visual: gliss " ++ startFreq ++ "→" ++ endFreq ++ " (sin audio)").postln;
            });
        };
        
        // PUNTO PERFECTAMENTE SINCRONIZADO
        sendPerfectPoint = { |freq=440, amp=0.5, dur=1.0|
            // Enviar evento visual
            addr.sendMsg("/event", "point", freq, amp, dur);
            
            if(s.serverRunning, {
                {
                    // Audio con EXACTAMENTE la misma duración
                    var env = Env.linen(0.05, dur - 0.1, 0.05);
                    var sig = SinOsc.ar(freq, 0, amp * EnvGen.kr(env, doneAction: 2));
                    Out.ar(0, sig.dup);
                }.play;
                ("🎯 PUNTO SINCRONIZADO: freq=" ++ freq ++ " dur=" ++ dur).postln;
            }, {
                ("👁️  Visual: punto freq=" ++ freq ++ " (sin audio)").postln;
            });
        };
        
        // CLUSTER PERFECTAMENTE SINCRONIZADO
        sendPerfectCluster = { |centerFreq=500, freqSpread=100, numVoices=4, amp=0.6, dur=4.0|
            // Enviar evento visual
            addr.sendMsg("/event", "cluster", centerFreq, freqSpread, numVoices, amp, dur);
            
            if(s.serverRunning, {
                {
                    // Audio con EXACTAMENTE la misma duración
                    var freqs = Array.fill(numVoices, { centerFreq + rrand(freqSpread.neg, freqSpread) });
                    var env = Env.linen(0.1, dur - 0.2, 0.1);
                    var sig = Mix.ar(freqs.collect({ |freq| SinOsc.ar(freq, 0, amp / numVoices) }));
                    sig = sig * EnvGen.kr(env, doneAction: 2);
                    Out.ar(0, sig.dup);
                }.play;
                ("🎯 CLUSTER SINCRONIZADO: freq=" ++ centerFreq ++ " voices=" ++ numVoices ++ " dur=" ++ dur).postln;
            }, {
                ("👁️  Visual: cluster freq=" ++ centerFreq ++ " (sin audio)").postln;
            });
        };
        
        // RUIDO PERFECTAMENTE SINCRONIZADO
        sendPerfectNoise = { |centerFreq=1000, bandwidth=200, amp=0.7, dur=3.0|
            // Enviar evento visual
            addr.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur);
            
            if(s.serverRunning, {
                {
                    // Audio con EXACTAMENTE la misma duración
                    var env = Env.linen(0.1, dur - 0.2, 0.1);
                    var sig = WhiteNoise.ar(amp);
                    sig = BPF.ar(sig, centerFreq, bandwidth / centerFreq);
                    sig = sig * EnvGen.kr(env, doneAction: 2);
                    Out.ar(0, sig.dup);
                }.play;
                ("🎯 RUIDO SINCRONIZADO: freq=" ++ centerFreq ++ " dur=" ++ dur).postln;
            }, {
                ("👁️  Visual: ruido freq=" ++ centerFreq ++ " (sin audio)").postln;
            });
        };
        
        "🎨 INICIANDO DEMOSTRACIÓN DE SINCRONIZACIÓN PERFECTA...".postln;
        if(s.serverRunning, {
            "🔊 Audio habilitado - observa la sincronización perfecta".postln;
        }, {
            "👁️  Solo visual (ejecuta s.boot; primero para audio)".postln;
        });
        
        // PRUEBAS DE SINCRONIZACIÓN
        
        "1. 🎯 TESTE DE DIRECCIÓN: glissando ascendente".postln;
        sendPerfectGliss.value(200, 800, 0.9, 4.0);
        1.5.wait;
        
        "2. 🎯 TESTE DE DIRECCIÓN: glissando descendente".postln;
        sendPerfectGliss.value(800, 200, 0.9, 4.0);
        1.5.wait;
        
        "3. 🎯 TESTE DE DURACIÓN: puntos de diferentes duraciones".postln;
        sendPerfectPoint.value(440, 0.8, 1.0);
        0.5.wait;
        sendPerfectPoint.value(550, 0.8, 2.0);
        0.5.wait;
        sendPerfectPoint.value(660, 0.8, 3.0);
        1.0.wait;
        
        "4. 🎯 TESTE COMPLEJO: múltiples eventos simultáneos".postln;
        sendPerfectGliss.value(100, 400, 0.8, 5.0);
        sendPerfectGliss.value(500, 200, 0.7, 5.0);
        sendPerfectCluster.value(300, 80, 6, 0.6, 5.0);
        sendPerfectNoise.value(1000, 300, 0.5, 5.0);
        
        3.0.wait;
        
        "5. 🎯 TESTE FINAL: secuencia compleja".postln;
        sendPerfectPoint.value(220, 0.9, 1.5);
        0.5.wait;
        sendPerfectGliss.value(150, 600, 0.8, 3.0);
        1.0.wait;
        sendPerfectCluster.value(450, 120, 8, 0.7, 4.0);
        1.0.wait;
        sendPerfectNoise.value(1200, 400, 0.6, 3.0);
        
        "🎉 ¡DEMOSTRACIÓN COMPLETA!".postln;
        "Si ves discrepancias entre audio y visual, repórtalo inmediatamente.".postln;
        "Este sistema está diseñado para live coding con sincronización perfecta.".postln;
        
    }).play;
};

// Verificar servidor y ejecutar
if(s.serverRunning.not, {
    "⚠️  Servidor de audio no está funcionando.".postln;
    "Intentando arrancar servidor...".postln;
    s.boot;
    s.waitForBoot({
        "✓ Servidor arrancado - ejecutando demo de sincronización perfecta...".postln;
        executeDemo.value;
    }, onFailure: {
        "✗ No se pudo arrancar el servidor - solo demostración visual".postln;
        executeDemo.value;
    });
}, {
    "✓ Servidor funcionando - ejecutando demo de sincronización perfecta...".postln;
    executeDemo.value;
});
)

// PARA LIVE CODING - FUNCIONES INDEPENDIENTES:
(
// Glissando con sincronización perfecta
~perfectGliss = { |startFreq=100, endFreq=400, amp=0.8, dur=3.0|
    var addr = NetAddr.new("127.0.0.1", 57122);
    addr.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur);
    if(s.serverRunning, {
        {
            var freqLine = XLine.kr(startFreq, endFreq, dur);
            var env = Env.linen(0.05, dur - 0.1, 0.05);
            var sig = SinOsc.ar(freqLine, 0, amp * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    });
};

// Punto con sincronización perfecta
~perfectPoint = { |freq=440, amp=0.5, dur=1.0|
    var addr = NetAddr.new("127.0.0.1", 57122);
    addr.sendMsg("/event", "point", freq, amp, dur);
    if(s.serverRunning, {
        {
            var env = Env.linen(0.05, dur - 0.1, 0.05);
            var sig = SinOsc.ar(freq, 0, amp * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    });
};

"✅ Funciones de live coding cargadas: ~perfectGliss, ~perfectPoint".postln;
"Ejemplo de uso: ~perfectGliss.(200, 800, 0.9, 4.0);".postln;
)
