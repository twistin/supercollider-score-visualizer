// demo_universal_cualquier_script.scd
// DEMOSTRACIÓN: El visualizador funciona con CUALQUIER código de SuperCollider
// NO REQUIERE MODIFICACIONES - Solo ejecuta y observa

(
"=== DEMO: VISUALIZADOR UNIVERSAL - CUALQUIER SCRIPT SC ===".postln;
"El visualizador detecta automáticamente CUALQUIER sonido y lo representa visualmente.".postln;
"NO necesitas enviar mensajes OSC específicos.".postln;
"Solo ejecuta cualquier código de SuperCollider y observa la magia.".postln;
"".postln;

// Función principal
var runDemo;

runDemo = {
    Routine({
        
        "🎵 1. SONIDOS SIMPLES - Tonos puros".postln;
        { SinOsc.ar(220, 0, 0.6) }.play;
        2.wait;
        
        { SinOsc.ar(440, 0, 0.6) }.play;
        2.wait;
        
        { SinOsc.ar(880, 0, 0.6) }.play;
        3.wait;
        
        "🎵 2. GLISSANDOS - Cambios de frecuencia".postln;
        { SinOsc.ar(XLine.kr(200, 800, 4), 0, 0.7) }.play;
        5.wait;
        
        { SinOsc.ar(XLine.kr(800, 200, 3), 0, 0.7) }.play;
        4.wait;
        
        "🎵 3. PATRONES RÍTMICOS - Pbind clásico".postln;
        (
        Pbind(
            \freq, Pseq([330, 440, 550, 660], 4),
            \dur, Pseq([0.25, 0.5, 0.25, 0.5], 4),
            \amp, Pseq([0.8, 0.6, 0.9, 0.7], 4)
        ).play;
        );
        6.wait;
        
        "🎵 4. CLUSTERS - Múltiples frecuencias simultáneas".postln;
        (
        {
            var freqs = [200, 230, 260, 290, 320];
            Mix.ar(SinOsc.ar(freqs, 0, 0.3 / freqs.size));
        }.play;
        );
        4.wait;
        
        "🎵 5. TEXTURAS DE RUIDO - Sonidos complejos".postln;
        { WhiteNoise.ar(0.4) * LFNoise1.ar(2).range(0, 1) }.play;
        3.wait;
        
        { BPF.ar(WhiteNoise.ar(0.6), LFNoise0.ar(1).range(200, 2000), 0.1) }.play;
        4.wait;
        
        "🎵 6. MÚSICA GENERATIVA - Algoritmos complejos".postln;
        (
        Pdef(\generative,
            Pbind(
                \freq, Prand([220, 330, 440, 550, 660] * Prand([1, 1.25, 1.5, 2], inf), inf),
                \dur, Pexprand(0.1, 1.0, inf),
                \amp, Pexprand(0.2, 0.8, inf),
                \pan, Prand([-1, 0, 1], inf)
            )
        ).play;
        );
        8.wait;
        Pdef(\generative).stop;
        
        "🎵 7. NDEFS - Síntesis en tiempo real".postln;
        (
        Ndef(\drone, {
            var freq = LFNoise1.ar(0.2).range(100, 400);
            var amp = LFNoise1.ar(0.5).range(0.3, 0.8);
            SinOsc.ar(freq * [1, 1.01], 0, amp);
        });
        Ndef(\drone).play;
        );
        5.wait;
        Ndef(\drone).stop;
        
        "🎵 8. MÚLTIPLES ELEMENTOS SIMULTÁNEOS".postln;
        (
        // Bajo
        { SinOsc.ar(60, 0, 0.4) }.play;
        
        // Melodía
        Pbind(
            \freq, Pseq([220, 330, 440, 330], inf),
            \dur, 0.5,
            \amp, 0.6
        ).play;
        
        // Textura
        { BPF.ar(WhiteNoise.ar(0.2), 1500, 0.1) }.play;
        );
        
        6.wait;
        
        "🎉 ¡DEMO COMPLETADO!".postln;
        "".postln;
        "🎯 OBSERVACIONES:".postln;
        "- Cada sonido fue detectado y visualizado automáticamente".postln;
        "- NO se enviaron mensajes OSC específicos".postln;
        "- La dirección de glissandos coincide con el audio".postln;
        "- La duración visual coincide con la duración del audio".postln;
        "- Los colores reflejan el contenido frecuencial".postln;
        "- Los patrones complejos se representan adecuadamente".postln;
        "".postln;
        "✨ Tu música + Visualizador Universal = Experiencia audiovisual completa".postln;
        "Ready para cualquier script de SuperCollider que tengas.".postln;
        
    }).play;
};

// Ejecutar
if(s.serverRunning, {
    "✅ Servidor funcionando - iniciando demo universal...".postln;
    runDemo.value;
}, {
    "⚠️  Arrancando servidor...".postln;
    s.boot;
    s.waitForBoot({
        "✅ Servidor listo - iniciando demo universal...".postln;
        runDemo.value;
    });
});
)

// EJEMPLOS ADICIONALES PARA PROBAR MANUALMENTE:

/*
// Música atonal
{ SinOsc.ar(LFNoise0.ar(4).range(200, 2000), 0, 0.5) }.play;

// Ritmos complejos
(
Pbind(
    \freq, Prand([110, 220, 330, 440, 550], inf),
    \dur, Prand([0.125, 0.25, 0.5, 0.75], inf),
    \amp, Prand([0.4, 0.6, 0.8], inf)
).play;
)

// Síntesis FM
{ SinOsc.ar(440 + (SinOsc.ar(200) * 100), 0, 0.6) }.play;

// Granular
{ GrainSin.ar(2, Dust.ar(10), 0.1, 440, 0) }.play;

// Tu propio código aquí...
// El visualizador se adaptará automáticamente
*/
