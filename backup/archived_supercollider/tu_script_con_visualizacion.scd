// 🎵 TU SCRIPT CON VISUALIZACIÓN OSC INTEGRADA
// Basado en tu script original + comunicación con SC Score Visualizer

("../Setup/Setup.scd").loadRelative;

Platform.userAppSupportDir.openOS;

s.options.sampleRate = 48000;

// ===================================================
// 🎯 CONFIGURACIÓN OSC PARA EL VISUALIZADOR
// ===================================================

// Crear conexión OSC al visualizador
~oscVisualizer = NetAddr("127.0.0.1", 57122); // Puerto del visualizador

// Función para enviar eventos al visualizador
~sendToVisualizer = {|type, freq, amp, dur, attack=0.1|
    ~oscVisualizer.sendMsg("/xenakis/" ++ type, freq, amp, dur, attack);
    ("📨 OSC -> " ++ type ++ " freq:" ++ freq.round(0.1) ++ " amp:" ++ amp.round(0.01) ++ " dur:" ++ dur).postln;
};

// ===================================================
// 🎵 TU MÚSICA ORIGINAL CON OSC INTEGRADO
// ===================================================

(
p = ProxySpace(
	server:s.boot,
	clock: t = TempoClock(90/60).permanent_(true)
).push;
)

///// buffer
b = ();
Buffer.freeAll;
b.glitch = PathName("/Users/sdcarr/Desktop/My-setup-sc-live-coding/samples/set1/glitch4/").entries.collect({|n|
	Buffer.readChannel(s, n.fullPath, channels:[0]);
});

~b1.play
~b1.stop
(
// Prewrite para ritmo con evolución recursiva
r = Prewrite(
    0.25,  // duración inicial (puede interpretarse como una corchea)
    (
        0.25: #[0.25, 0.25, 0.25, 0.5],
        0.5: #[0.25, 0.5, 0.125, 0.125, 0.125, 0.125],
        0.125: #[0.375, 0.125],
        0.375: #[0.375, 0.375, 1],
        1: #[0.75, 0.25],
        0.75: #[16]
    ),
    5  // número de generaciones
);

l = Prewrite(
    1, // grado inicial (1 = segundo grado de la escala)
    (
        1: #[1, 2],
        2: #[3],
        3: #[5],
        5: #[8],
        8: #[13],
        13: #[21],
        21: #[55],
        55: #[89],
		89: #[144],
		144:#[1]// vuelve al inicio para cerrar el ciclo
    ),
	10 // número de generaciones
);

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
	
	// ✅ AÑADIR VISUALIZACIÓN OSC
	\callback, Pfunc({|event|
		var freq = event[\freq];
		var amp = event[\amp];
		var dur = event[\dur];
		var attack = event[\atk];
		
		// Enviar evento de punto al visualizador
		~sendToVisualizer.("point", freq, amp, dur, attack);
	})
))

~m1.play;
~m1.stop
(
r = Prewrite(
    0.25,  // duración inicial (puede interpretarse como una corchea)
    (
        0.25: #[0.25, 0.25, 0.25, 0.5],
        0.5: #[0.25, 0.5, 0.125, 0.125, 0.125, 0.125],
        0.125: #[0.375, 0.125],
        0.375: #[0.375, 0.375, 1],
        1: #[0.75, 0.25],
        0.75: #[16]
    ),
    5  // número de generaciones
);

~m1[0] = Pbind(
	\instrument, \plucking,
	\scale, Scale.yu,
	\octave, 4,
	\degree, Pseq([0, 3, 5, 7, 10], inf), // four main notes
	\mtranspose, Pseq([Pseq([0],3),Pseq([-5],3)],inf), // switched mode every 3 notes (*2)
	\gtranspose, Pseq([Pseq([0],36),Pseq([4],36),Pseq([[-3]],36),Pseq([2],36)],inf),
	\ctranspose, Pser([Pseries(-4,1,6) +.x Pseq([0], 144)],1),
	\coef, Pser([Prand([0.2,0.3,0.4,0.5],1), Prand([0.8,0.7,0.6,0.5],11)], inf),
	\dur, Prand([0.25,0.26,0.24,0.255,0.255,0.25], inf) * 0.9,
	\decay, Prand([1,2,3,4,5], inf),
	\out, b,
	\amp, 0.4,
	
	// ✅ AÑADIR VISUALIZACIÓN OSC PARA PLUCKING
	\callback, Pfunc({|event|
		var freq = event[\freq];
		var amp = event[\amp];
		var dur = event[\dur];
		var decay = event[\decay];
		
		// Enviar evento de punto al visualizador
		~sendToVisualizer.("point", freq, amp, dur, 0.05);
		
		// Si hay mucha actividad, también enviar como cluster
		if(dur < 0.3, {
			~sendToVisualizer.("glissando", freq, freq * 1.02, dur * 2, amp * 0.5);
		});
	})
))

~m1[1] = \filter -> {|in|
	in * SinOsc.ar(150*[1,0.995])
};

~m1[1] = \filter -> {|in|
	CombL.ar(in, 0.2, 0.07, 0.3)
};

~m1[2] = \filter -> {|in|
	GVerb.ar(in, 3, 50);
}

~m1.set(\wet2, 0)

// ===================================================
// 🎛️ CONTROLES ADICIONALES PARA EL VISUALIZADOR
// ===================================================

// Función para enviar glissandos manuales
~visualGliss = {|startFreq=200, endFreq=800, dur=2.0, amp=0.7|
    ~sendToVisualizer.("glissando", startFreq, endFreq, dur, amp);
};

// Función para enviar clusters
~visualCluster = {|centerFreq=440, spread=0.2, voices=8, dur=3.0, amp=0.6|
    ~oscVisualizer.sendMsg("/xenakis/cluster", centerFreq, spread, voices, dur, amp);
};

// Ejemplos de uso:
// ~visualGliss.(220, 880, 3.0, 0.8);
// ~visualCluster.(330, 0.5, 12, 4.0, 0.7);

"🎵 Script con visualización OSC configurado".postln;
"🎯 Ahora ejecuta el visualizador y luego reproduce tu música".postln;
"📡 Los eventos se enviarán automáticamente al visualizador en puerto 57122".postln;
