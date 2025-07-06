// demo_simple_audiovisual.scd
// Demo simple y robusto para verificar funcionalidad audiovisual

(
var addr;

"=== DEMO SIMPLE AUDIOVISUAL ===".postln;

// Crear conexión OSC
addr = NetAddr.new("127.0.0.1", 57122);

"🎨 Enviando eventos audiovisuales simples...".postln;

// 1. Punto simple con sonido
"1. Punto con sonido...".postln;
addr.sendMsg("/event", "point", 440, 0.8, 2.0);
{
    var env = Env.perc(0.1, 0.5);
    var sig = SinOsc.ar(440, 0, 0.8 * EnvGen.kr(env, doneAction: 2));
    Out.ar(0, sig.dup);
}.play;

// 2. Glissando con sonido
"2. Glissando con sonido...".postln;
addr.sendMsg("/event", "gliss", 220, 880, 0.7, 3.0);
{
    var freqLine = XLine.kr(220, 880, 3.0);
    var env = Env.linen(0.1, 2.8, 0.1);
    var sig = SinOsc.ar(freqLine, 0, 0.7 * EnvGen.kr(env, doneAction: 2));
    Out.ar(0, sig.dup);
}.play;

// 3. Cluster con sonido
"3. Cluster con sonido...".postln;
addr.sendMsg("/event", "cluster", 660, 100, 4, 0.6, 3.0);
{
    var freqs = [620, 660, 700, 740];
    var env = Env.linen(0.2, 2.6, 0.2);
    var sig = Mix.ar(freqs.collect({ |freq| SinOsc.ar(freq, 0, 0.15) }));
    sig = sig * EnvGen.kr(env, doneAction: 2);
    Out.ar(0, sig.dup);
}.play;

"🎉 Demo simple completado - deberías ver Y escuchar 3 eventos".postln;
"Si solo ves visualización sin audio, ejecuta s.boot; primero".postln;
)
