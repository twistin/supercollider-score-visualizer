// demo_sin_multiples_var.scd
// Demo evitando completamente múltiples declaraciones var

(
var todo; // Una sola variable que contendrá todo

"=== DEMO SIN MÚLTIPLES VAR ===".postln;

todo = (
    addr: NetAddr.new("127.0.0.1", 57122),
    
    sendPoint: { |self, freq, amp, dur|
        self.addr.sendMsg("/event", "point", freq, amp, dur);
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
    },
    
    sendGliss: { |self, startFreq, endFreq, amp, dur|
        self.addr.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur);
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
    },
    
    ejecutar: { |self|
        "🎨 Demo iniciado...".postln;
        
        if(s.serverRunning, {
            "🔊 Audio habilitado".postln;
        }, {
            "👁️ Solo visual".postln;
        });
        
        "1. Punto...".postln;
        self.sendPoint(440, 0.8, 2.0);
        
        "2. Glissando...".postln;
        self.sendGliss(220, 880, 0.7, 3.0);
        
        "🎉 Completado".postln;
    }
);

// Verificar servidor y ejecutar
if(s.serverRunning.not, {
    "⚠️ Arrancando servidor...".postln;
    s.boot;
    s.waitForBoot({
        "✓ Con audio".postln;
        todo.ejecutar;
    }, onFailure: {
        "✗ Solo visual".postln;
        todo.ejecutar;
    });
}, {
    "✓ Con audio".postln;
    todo.ejecutar;
});
)
