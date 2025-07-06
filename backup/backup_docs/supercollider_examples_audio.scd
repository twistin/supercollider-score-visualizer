// Versión con audio para SC Score Visualizer
// Esta versión envía eventos al visualizador Y produce sonido en SuperCollider

(
// Variables para el visualizador
var visualizer, sendPoint, sendGliss, sendCluster, sendNoise, sendSoundMass;
var testBasicEventsAudio;

"=== Iniciando SC Score Visualizer CON AUDIO ===".postln;

// Configurar la dirección del visualizador
visualizer = NetAddr.new("127.0.0.1", 57122);

// Función para enviar evento puntual CON sonido
sendPoint = { |freq=440, amp=0.5, dur=1.0, attack=0.1, decay=0.1, density=0.5, texture=0.0, spread=0.0, hue=220|
    // Enviar al visualizador
    visualizer.sendMsg("/event", "point", freq, amp, dur, attack, decay, density, texture, spread, hue);
    
    // Producir sonido
    {
        var env = Env.perc(attack, decay);
        var sig = SinOsc.ar(freq, 0, amp * EnvGen.kr(env, doneAction: 2));
        Out.ar(0, sig.dup);
    }.play;
    
    ("Enviado: punto freq=" ++ freq ++ " amp=" ++ amp ++ " + sonido").postln;
};

// Función para enviar glissando CON sonido
sendGliss = { |startFreq=440, endFreq=880, amp=0.7, dur=2.0, curvature=0.0, density=0.7, texture=0.0, spread=0.0, hue=30|
    // Enviar al visualizador
    visualizer.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur, curvature, density, texture, spread, hue);
    
    // Producir sonido
    {
        var freqLine = XLine.kr(startFreq, endFreq, dur);
        var env = Env.linen(0.1, dur - 0.2, 0.1);
        var sig = SinOsc.ar(freqLine, 0, amp * EnvGen.kr(env, doneAction: 2));
        Out.ar(0, sig.dup);
    }.play;
    
    ("Enviado: glissando " ++ startFreq ++ " → " ++ endFreq ++ " + sonido").postln;
};

// Función para enviar cluster CON sonido
sendCluster = { |centerFreq=440, freqSpread=100, numVoices=8, amp=0.6, dur=3.0, density=0.8, texture=0.3, spread=0.5, hue=120|
    // Enviar al visualizador
    visualizer.sendMsg("/event", "cluster", centerFreq, freqSpread, numVoices, amp, dur, density, texture, spread, hue);
    
    // Producir sonido
    {
        var freqs = Array.fill(numVoices, { centerFreq + (freqSpread * (0.5 - 1.0.rand)) });
        var env = Env.linen(0.2, dur - 0.4, 0.2);
        var sig = Mix.fill(numVoices, { |i|
            SinOsc.ar(freqs[i], 0, amp / numVoices)
        });
        sig = sig * EnvGen.kr(env, doneAction: 2);
        Out.ar(0, sig.dup);
    }.play;
    
    ("Enviado: cluster centro=" ++ centerFreq ++ " voces=" ++ numVoices ++ " + sonido").postln;
};

// Función para enviar ruido CON sonido
sendNoise = { |centerFreq=1000, bandwidth=500, amp=0.4, dur=2.0, grainSize=1.0, spectralTilt=0.0, density=0.6, texture=0.8, spread=0.4, hue=300|
    // Enviar al visualizador
    visualizer.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur, grainSize, spectralTilt, density, texture, spread, hue);
    
    // Producir sonido
    {
        var env = Env.linen(0.1, dur - 0.2, 0.1);
        var sig = BPF.ar(
            WhiteNoise.ar(amp), 
            centerFreq + (bandwidth * (0.5 - 1.0.rand)), 
            0.1
        );
        sig = sig * EnvGen.kr(env, doneAction: 2);
        Out.ar(0, sig.dup);
    }.play;
    
    ("Enviado: ruido centro=" ++ centerFreq ++ " ancho=" ++ bandwidth ++ " + sonido").postln;
};

// Función para enviar masa sonora CON sonido
sendSoundMass = { |numComponents=5, amp=0.5, dur=4.0, evolutionRate=0.5, turbulence=0.3, density=0.9, texture=0.5, spread=0.7, hue=60|
    var msg = ["/event", "mass", numComponents, amp, dur, evolutionRate, turbulence, density, texture, spread, hue];
    
    // Añadir componentes espectrales
    numComponents.do { |i|
        var freq = 440 * (1 + (i * 0.618));
        var componentAmp = 1.0 / (i + 1);
        msg = msg ++ [freq, componentAmp];
    };
    
    // Enviar al visualizador
    visualizer.sendMsg(*msg);
    
    // Producir sonido
    {
        var env = Env.linen(0.5, dur - 1.0, 0.5);
        var sig = Mix.fill(numComponents, { |i|
            var freq = 440 * (1 + (i * 0.618));
            var componentAmp = (1.0 / (i + 1)) * amp;
            SinOsc.ar(freq, 0, componentAmp / numComponents)
        });
        sig = sig * EnvGen.kr(env, doneAction: 2);
        Out.ar(0, sig.dup);
    }.play;
    
    ("Enviado: masa sonora con " ++ numComponents ++ " componentes + sonido").postln;
};

// Función de prueba CON audio
testBasicEventsAudio = {
    "=== Iniciando prueba CON AUDIO y visualización ===".postln;
    
    sendPoint.(440, 0.3, 2.0);
    
    {
        2.5.wait; 
        sendGliss.(220, 880, 0.4, 3.0, 0.5);
    }.fork;
    
    {
        6.wait; 
        sendCluster.(660, 150, 6, 0.3, 4.0);
    }.fork;
    
    {
        11.wait; 
        sendNoise.(1000, 500, 0.2, 2.5);
    }.fork;
    
    {
        14.wait; 
        sendSoundMass.(4, 0.2, 3.0);
    }.fork;
};

// Exportar funciones con audio
~scvTestBasicEventsAudio = testBasicEventsAudio;
~scvSendPointAudio = sendPoint;
~scvSendGlissAudio = sendGliss;
~scvSendClusterAudio = sendCluster;
~scvSendNoiseAudio = sendNoise;
~scvSendSoundMassAudio = sendSoundMass;

"".postln;
"✓ ✓ ✓ FUNCIONES CON AUDIO LISTAS ✓ ✓ ✓".postln;
"".postln;
"Para probar CON sonido Y visualización:".postln;
"~scvTestBasicEventsAudio.()".postln;
"".postln;
"IMPORTANTE: El visualizador debe estar ejecutándose en puerto 57122".postln;

) // Fin del bloque
