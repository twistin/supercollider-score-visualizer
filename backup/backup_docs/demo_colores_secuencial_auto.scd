// demo_colores_secuencial_auto.scd
// Demostración secuencial con espaciado temporal y AUDIO
// VERSION CON ARRANQUE AUTOMÁTICO DEL SERVIDOR

(
"=== DEMO COLORES CON SECUENCIA TEMPORAL (AUDIOVISUAL AUTO) ===".postln;

// Función principal que maneja el servidor
var executeSequentialDemo;

executeSequentialDemo = {
    Routine({
        var addr, sendAVPoint, sendAVGliss, sendAVCluster, sendAVNoise;
        
        addr = NetAddr.new("127.0.0.1", 57122);
        
        // Función para enviar punto con audio y visual
        sendAVPoint = { |freq=440, amp=0.5, dur=1.0|
            addr.sendMsg("/event", "point", freq, amp, dur);
            if(s.serverRunning, {
                { 
                    var env = Env.perc(0.1, 0.4);
                    var sig = SinOsc.ar(freq, 0, amp * EnvGen.kr(env, doneAction: 2));
                    Out.ar(0, sig.dup);
                }.play;
            }, {
                ("Visual: punto freq=" ++ freq ++ " (sin audio)").postln;
            });
        };
        
        // Función para enviar glissando con audio y visual
        sendAVGliss = { |startFreq=100, endFreq=400, amp=0.8, dur=3.0|
            addr.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur);
            if(s.serverRunning, {
                {
                    var freqLine = XLine.kr(startFreq, endFreq, dur);
                    var env = Env.linen(0.1, dur - 0.2, 0.1);
                    var sig = SinOsc.ar(freqLine, 0, amp * EnvGen.kr(env, doneAction: 2));
                    Out.ar(0, sig.dup);
                }.play;
            }, {
                ("Visual: gliss " ++ startFreq ++ "→" ++ endFreq ++ " (sin audio)").postln;
            });
        };
        
        // Función para enviar cluster con audio y visual
        sendAVCluster = { |centerFreq=500, freqSpread=100, numVoices=4, amp=0.6, dur=4.0|
            addr.sendMsg("/event", "cluster", centerFreq, freqSpread, numVoices, amp, dur);
            if(s.serverRunning, {
                {
                    var freqs = Array.fill(numVoices, { centerFreq + rrand(freqSpread.neg, freqSpread) });
                    var env = Env.linen(0.2, dur - 0.4, 0.2);
                    var sig = Mix.ar(freqs.collect({ |freq| SinOsc.ar(freq, 0, amp / numVoices) }));
                    sig = sig * EnvGen.kr(env, doneAction: 2);
                    Out.ar(0, sig.dup);
                }.play;
            }, {
                ("Visual: cluster freq=" ++ centerFreq ++ " (sin audio)").postln;
            });
        };
        
        // Función para enviar ruido con audio y visual
        sendAVNoise = { |centerFreq=1000, bandwidth=200, amp=0.7, dur=3.0|
            addr.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur);
            if(s.serverRunning, {
                {
                    var env = Env.linen(0.3, dur - 0.6, 0.3);
                    var sig = WhiteNoise.ar(amp);
                    sig = BPF.ar(sig, centerFreq, bandwidth / centerFreq);
                    sig = sig * EnvGen.kr(env, doneAction: 2);
                    Out.ar(0, sig.dup);
                }.play;
            }, {
                ("Visual: ruido freq=" ++ centerFreq ++ " (sin audio)").postln;
            });
        };
        
        "🎨 Iniciando demostración secuencial audiovisual de colores...".postln;
        if(s.serverRunning, {
            "🔊 Audio habilitado".postln;
        }, {
            "👁️  Solo visual (para audio ejecuta s.boot; primero)".postln;
        });
        
        // 1. Puntos con diferentes frecuencias
        "1. Puntos coloridos por frecuencia...".postln;
        sendAVPoint.value(220, 0.9, 2.0);
        1.0.wait;
        sendAVPoint.value(440, 0.8, 2.0);
        1.0.wait;
        sendAVPoint.value(880, 0.7, 2.0);
        1.5.wait;
        
        // 2. Glissandos coloridos
        "2. Glissandos con colores estructurales...".postln;
        sendAVGliss.value(100, 400, 0.8, 3.0);
        1.5.wait;
        sendAVGliss.value(200, 800, 0.9, 3.0);
        2.0.wait;
        
        // 3. Clusters dinámicos
        "3. Clusters con densidades variables...".postln;
        sendAVCluster.value(500, 100, 4, 0.6, 4.0);
        1.0.wait;
        sendAVCluster.value(700, 200, 8, 0.8, 4.0);
        1.0.wait;
        sendAVCluster.value(900, 300, 12, 0.9, 4.0);
        2.0.wait;
        
        // 4. Ruido colorido
        "4. Texturas de ruido...".postln;
        sendAVNoise.value(1000, 200, 0.7, 3.0);
        1.5.wait;
        sendAVNoise.value(1500, 500, 0.8, 3.0);
        2.0.wait;
        
        // 5. Explosión final
        "5. ¡EXPLOSIÓN FINAL DE COLORES Y SONIDOS!".postln;
        sendAVPoint.value(330, 0.8, 3.0);
        sendAVPoint.value(660, 0.7, 3.0);
        sendAVPoint.value(990, 0.9, 3.0);
        sendAVGliss.value(150, 600, 0.8, 4.0);
        sendAVGliss.value(800, 300, 0.7, 4.0);
        sendAVCluster.value(450, 180, 10, 0.9, 5.0);
        sendAVNoise.value(1200, 400, 0.6, 5.0);
        
        "🎉 ¡Secuencia completa!".postln;
        if(s.serverRunning, {
            "Observa todos los colores y efectos MIENTRAS escuchas el audio sincronizado.".postln;
        }, {
            "Solo visualización (para escuchar audio, reinicia con servidor funcionando)".postln;
        });
        
    }).play;
};

// Verificar servidor y ejecutar
if(s.serverRunning.not, {
    "⚠️  Servidor de audio no está funcionando.".postln;
    "Intentando arrancar servidor...".postln;
    s.boot;
    s.waitForBoot({
        "✓ Servidor arrancado - ejecutando demo secuencial completo...".postln;
        executeSequentialDemo.value;
    }, onFailure: {
        "✗ No se pudo arrancar el servidor - solo demostración visual".postln;
        executeSequentialDemo.value;
    });
}, {
    "✓ Servidor funcionando - ejecutando demo secuencial completo...".postln;
    executeSequentialDemo.value;
});
)
