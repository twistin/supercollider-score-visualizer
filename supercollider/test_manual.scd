// =============================================================================
// 🎼 TEST MANUAL - Análisis de Audio en Tiempo Real
// =============================================================================
// Ejecuta este archivo en SuperCollider IDE para probar el sistema
// =============================================================================

(
"🎵 === INICIANDO TEST MANUAL ===".postln;
"📋 Instrucciones:".postln;
"   1. Ejecuta este bloque completo".postln;
"   2. Inicia el visualizador Rust en la terminal".postln;
"   3. Reproduce música o haz sonidos".postln;
"   4. Observa las visualizaciones sincronizadas".postln;
"";

// Cargar el SynthDef de análisis
load(Platform.userHomeDir ++ "/Documents/github/sc-score/sc_score_visualizer/realtime_analysis.scd");
)

// Iniciar servidor (evaluar por separado si no está iniciado)
(
Server.default.boot;
)

// Una vez que el servidor esté listo, ejecutar el analizador
(
"🚀 Iniciando análisis en tiempo real...".postln;
~analyzer = Synth(\RealtimeAnalyzer);

"📡 Enviando datos OSC a puerto 57124 cada 20ms".postln;
"🎨 Inicia el visualizador Rust ahora!".postln;
"";
"💡 Comandos útiles:".postln;
"   ~analyzer.free;  // Para detener el análisis".postln;
"   s.meter;         // Para ver niveles de audio".postln;
"   s.scope;         // Para ver forma de onda".postln;
)

// Opcional: Generar sonido de prueba si no hay entrada de audio
(
"🎵 Generando sonido de prueba (sintetizador simple)".postln;
~testSynth = {
    var sig = SinOsc.ar([220, 330] * LFNoise1.kr(0.5).range(0.8, 1.2)) * 
              EnvGen.kr(Env.perc(0.01, 1), Impulse.kr(2)) * 0.3;
    Out.ar(0, sig);
}.play;
)

// Para detener todo
(
"🛑 Deteniendo análisis y sonidos de prueba...".postln;
~analyzer.free;
~testSynth.free;
)

// Test con glissando (inspiración Xenakis)
(
"🎼 Test con glissando estilo Xenakis...".postln;
~xenakisTest = {
    var freq = XLine.kr(200, 2000, 8);
    var amp = Line.kr(0.1, 0.8, 8) * Line.kr(1, 0, 8);
    var sig = SinOsc.ar(freq) * amp;
    Out.ar(0, sig ! 2);
}.play;
)

// Test con masa sonora (clusters)
(
"🎼 Test con masa sonora (clusters)...".postln;
~clusterTest = {
    var freqs = {exprand(200, 2000)} ! 20;
    var sig = Mix(SinOsc.ar(freqs) * LFNoise1.kr({exprand(0.1, 2)} ! 20).max(0)) * 0.1;
    Out.ar(0, sig ! 2);
}.play;
)

// Test rítmico (onsets claros)
(
"🎼 Test rítmico (onsets marcados)...".postln;
~rhythmTest = {
    var trig = Impulse.kr([2, 3, 5, 7].choose);
    var freq = TChoose.kr(trig, [220, 330, 440, 550]);
    var sig = SinOsc.ar(freq) * EnvGen.kr(Env.perc(0.01, 0.2), trig) * 0.3;
    Out.ar(0, sig ! 2);
}.play;
)

// Detener tests
(
[~xenakisTest, ~clusterTest, ~rhythmTest].do(_.free);
)
