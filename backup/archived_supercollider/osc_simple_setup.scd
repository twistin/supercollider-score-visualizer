// CONFIGURACION OSC ROBUSTA PARA SC SCORE VISUALIZER
// Para usar con tu script - Metodo ProxySpace robusto

// ===================================================
// CONFIGURACION OSC ROBUSTA (PROXYSPACE)
// ===================================================

(
// Configuracion OSC robusta usando ProxySpace
~oscOut = NetAddr("127.0.0.1", 57122);

// Funcion robusta para envio OSC con ProxySpace
~scvSendEvent = {|freq, amp, dur, type="point"|
    var message = "/xenakis/" ++ type;
    ~oscOut.sendMsg(message, freq, amp, dur, 0.1);
    ("OSC -> " ++ type ++ " f:" ++ freq.round(0.1) ++ " a:" ++ amp.round(0.01)).postln;
};

// Auto-envio para cualquier Pbind activo
~scvAutoSend = {|event|
    var freq = event[\freq] ? 440;
    var amp = event[\amp] ? 0.5;
    var dur = event[\dur] ? 1.0;
    
    // Enviar al visualizador
    ~scvSendEvent.(freq, amp, dur, "point");
};

// Version para glissandos
~scvGliss = {|startFreq, endFreq, dur, amp|
    ~oscOut.sendMsg("/xenakis/glissando", startFreq, endFreq, dur, amp);
    ("Glissando: " ++ startFreq ++ " -> " ++ endFreq).postln;
};

"Configuracion OSC robusta lista".postln;
"Agrega callback, Pfunc(~scvAutoSend) a tus Pbinds".postln;
)

// ===================================================
// EJEMPLO DE USO CON TU MUSICA
// ===================================================

// Version modificada de tu ~b1 con visualizacion
/*
~b1[0] = Pbind(
	\instrument, \blip1,
	\scale, Scale.yu,
	\degree, l,
	\octave, [2],
	\dur, 1/3,
	\amp, 1/3,
	\level, 1.5,
	\atk, 0,
	\sus, 0,
	\rel, 0.75,
	\curve, -6,
	\pan, 0,
	
	// ANADIR ESTA LINEA
	\callback, Pfunc(~scvAutoSend)
)
*/

// Version modificada de tu ~m1 con visualizacion
/*
~m1[0] = Pbind(
	\instrument, \plucking,
	\scale, Scale.yu,
	\octave, 4,
	\degree, Pseq([0, 3, 5, 7, 10], inf),
	\mtranspose, Pseq([Pseq([0],3),Pseq([-5],3)],inf),
	\gtranspose, Pseq([Pseq([0],36),Pseq([4],36),Pseq([[-3]],36),Pseq([2],36)],inf),
	\ctranspose, Pser([Pseries(-4,1,6) +.x Pseq([0], 144)],1),
	\coef, Pser([Prand([0.2,0.3,0.4,0.5],1), Prand([0.8,0.7,0.6,0.5],11)], inf),
	\dur, Prand([0.25,0.26,0.24,0.255,0.255,0.25], inf) * 0.9,
	\decay, Prand([1,2,3,4,5], inf),
	\out, b,
	\amp, 0.4,
	
	// ANADIR ESTA LINEA
	\callback, Pfunc(~scvAutoSend)
)
*/

"INSTRUCCIONES:".postln;
"1. Ejecuta este codigo primero".postln;
"2. Inicia el visualizador: ./target/release/sc_score_visualizer".postln;
"3. Agrega callback, Pfunc(~scvAutoSend) a cada Pbind".postln;
"4. Reproduce tu musica - veras la visualizacion automaticamente".postln;
