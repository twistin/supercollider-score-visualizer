



// setup_visualizer.scd - Configuración básica del visualizador SC Score Visualizer
// Cargar este archivo en SuperCollider para comenzar con live coding




//============================================================
// CONFIGURACIÓN INICIAL
// ====================================================================

// Configurar servidor de audio
s.boot;

// Salir de ProxySpace si estamos en uno para evitar conflictos
if(currentEnvironment.isKindOf(ProxySpace)) {
    "Saliendo de ProxySpace para configuración...".postln;
    currentEnvironment = topEnvironment;
};

// Definir un sintetizador simple para generar audio
SynthDef(\default, { |out = 0, freq = 440, amp = 0.5, dur = 1.0|
    var env = Env.perc(0.01, dur, amp, -4).kr(doneAction: 2);
    var sig = SinOsc.ar(freq, 0, 0.5);
    var pan = Pan2.ar(sig, 0);
    Out.ar(out, pan * env);
}).add;

// Configurar NetAddr para comunicación OSC con el visualizador
// Usar variable global directamente
~visualizer = NetAddr("127.0.0.1", 6000); // Asegúrate de que este puerto coincida con el que Nannou está escuchando

"🎵 Configurando SC Score Visualizer...".postln;

// ====================================================================
// FUNCIONES BÁSICAS (CON AUDIO)
// ====================================================================

// Función para enviar notas al visualizador y reproducir sonido
(
~sendNote = { |freq, amp, dur|
    Synth(\default, [freq: freq, amp: amp, dur: dur]);
    ~visualizer.sendMsg("/note_on", freq, amp, dur); // <--- CAMBIADO A /note_on
};
);

// Función para enviar drones y reproducir sonido
(
~sendDrone = { |freq, amp, dur|
    Synth(\default, [freq: freq, amp: amp, dur: dur]);
    ~visualizer.sendMsg("/drone_on", freq, amp, dur); // <--- CAMBIADO A /drone_on
};
);

// Función para enviar clusters y reproducir sonido
(
~sendCluster = { |freq, amp, dur, density|
    // Tocar una nota representativa del clúster
    Synth(\default, [freq: freq, amp: amp, dur: dur]);
    ~visualizer.sendMsg("/cluster", freq, amp, dur, density); // Se mantiene /cluster por ahora
};
);

// Función para limpiar eventos
(
~clearAll = {
    ~visualizer.sendMsg("/clear"); // Se mantiene /clear por ahora
};
);

// ====================================================================
// FUNCIONES DE UTILIDAD
// ====================================================================

// Convertir nota MIDI a frecuencia
~midiToFreq = { |midiNote| midiNote.midicps };

// Escala mayor
~majorScale = [0, 2, 4, 5, 7, 9, 11];

// Escala menor
~minorScale = [0, 2, 3, 5, 7, 8, 10];

// Generar escala desde nota base
~generateScale = { |baseNote, scale|
    scale.collect({ |interval| ~midiToFreq.value(baseNote + interval) });
};

// ====================================================================
// FUNCIONES DE PRUEBA
// ====================================================================

// Prueba de conectividad
~testConnection = {
    "🔍 Probando conexión...".postln;
    ~sendNote.value(440, 0.5, 1.0);
    "✅ Mensaje enviado: nota A4 (440 Hz)".postln;
};

// Prueba de escala
~testScale = {
    var escala;
    "🎶 Probando escala...".postln;
    escala = ~generateScale.value(60, ~majorScale); // Do mayor
    Routine({
        escala.do({ |freq|
            ~sendNote.value(freq, 0.6, 0.5);
            0.5.wait;
        });
        "✅ Escala completada".postln;
    }).play;
};

// Prueba de drone
~testDrone = {
    "🎵 Probando drone...".postln;
    ~sendDrone.value(110, 0.3, 5.0);  // La2, suave, 5 segundos
    "✅ Drone enviado".postln;
};

// Prueba de cluster
~testCluster = {
    "💥 Probando cluster...".postln;
    ~sendCluster.value(330, 0.4, 3.0, 8);
    "✅ Cluster enviado".postln;
};

// ====================================================================
// GENERADORES PARA LIVE CODING
// ====================================================================

// Variables globales para modificar en tiempo real
~tempo = 0.5;
~baseFreq = 440;
~amplitude = 0.5;
~currentScale = ~majorScale;

// Generador aleatorio simple
~startRandom = {
    ~randomGenerator = Routine({
        loop {
            var freq = rrand(200, 2000);
            var amp = rrand(0.2, 0.8);
            var dur = rrand(0.5, 3.0);

            ~sendNote.value(freq, amp, dur);
            rrand(0.1, 1.0).wait;
        }
    }).play;
    "🎲 Generador aleatorio iniciado".postln;
};

// Detener generador aleatorio
~stopRandom = {
    if (~randomGenerator.notNil) {
        ~randomGenerator.stop;
        "⏹️ Generador aleatorio detenido".postln;
    };
};

// Pattern melódico simple
~startMelody = {
    ~melodyPattern = Routine({
        var notas = ~generateScale.value(60, ~currentScale);
        loop {
            ~sendNote.value(notas.choose, ~amplitude, ~tempo);
            ~tempo.wait;
        }
    }).play;
    "🎶 Melodía iniciada".postln;
};

// Detener melodía
~stopMelody = {
    if (~melodyPattern.notNil) {
        ~melodyPattern.stop;
        "⏹️ Melodía detenida".postln;
    };
};

// Bass line
~startBass = {
    ~bassPattern = Routine({
        var bajo = ~generateScale.value(36, ~currentScale);
        loop {
            ~sendNote.value(bajo.choose, ~amplitude * 0.8, ~tempo * 2);
            (~tempo * 2).wait;
        }
    }).play;
    "🔊 Línea de bajo iniciada".postln;
};

// Detener bajo
~stopBass = {
    if (~bassPattern.notNil) {
        ~bassPattern.stop;
        "⏹️ Línea de bajo detenida".postln;
    };
};

// ====================================================================
// EFECTOS VISUALES AVANZADOS
// ====================================================================

// Visualización compleja con múltiples capas
~startComplex = {
    ~complexVisual = Routine({
        loop {
            // Nota principal
            ~sendNote.value(rrand(300, 800), 0.7, 2.0);

            // Drone de fondo ocasional
            if (0.3.coin) {
                ~sendDrone.value(rrand(80, 120), 0.2, 4.0);
            };

            // Cluster esporádico
            if (0.2.coin) {
                ~sendCluster.value(rrand(500, 1500), 0.5, 1.5, rrand(5, 12));
            };

            rrand(0.5, 2.0).wait;
        }
    }).play;
    "🎨 Visualización compleja iniciada".postln;
};

// Detener visualización compleja
~stopComplex = {
    if (~complexVisual.notNil) {
        ~complexVisual.stop;
        "⏹️ Visualización compleja detenida".postln;
    };
};

// ====================================================================
// FUNCIONES DE CONTROL
// ====================================================================

// Detener todo
~stopAll = {
    ~stopRandom.value;
    ~stopMelody.value;
    ~stopBass.value;
    ~stopComplex.value;
    ~clearAll.value;
    "🛑 Todo detenido y limpiado".postln;
};

// Cambiar escala
~setMajor = {
    ~currentScale = ~majorScale;
    "✅ Escala cambiada a mayor".postln;
};

~setMinor = {
    ~currentScale = ~minorScale;
    "✅ Escala cambiada a menor".postln;
};

// ====================================================================
// MENÚ DE AYUDA
// ====================================================================

~showHelp = {
    "".postln;
    "🎵 SC Score Visualizer - Comandos Disponibles".postln;
    "===============================================".postln;
    "".postln;
    "🔧 CONFIGURACIÓN:".postln;
    "  ~testConnection.value    - Probar conexión".postln;
    "  ~testScale.value         - Probar escala".postln;
    "  ~testDrone.value         - Probar drone".postln;
    "  ~testCluster.value       - Probar cluster".postln;
    "".postln;
    "🎶 GENERADORES:".postln;
    "  ~startRandom.value       - Iniciar generador aleatorio".postln;
    "  ~stopRandom.value        - Detener generador aleatorio".postln;
    "  ~startMelody.value       - Iniciar melodía".postln;
    "  ~stopMelody.value        - Detener melodía".postln;
    "  ~startBass.value         - Iniciar línea de bajo".postln;
    "  ~stopBass.value          - Detener línea de bajo".postln;
    "".postln;
    "🎨 EFECTOS:".postln;
    "  ~startComplex.value      - Iniciar visualización compleja".postln;
    "  ~stopComplex.value       - Detener visualización compleja".postln;
    "".postln;
    "🛑 CONTROL:".postln;
    "  ~stopAll.value           - Detener todo".postln;
    "  ~clearAll.value          - Limpiar visualizador".postln;
    "  ~setMajor.value          - Cambiar a escala mayor".postln;
    "  ~setMinor.value          - Cambiar a escala menor".postln;
    "".postln;
    "🎛️ VARIABLES PARA MODIFICAR EN VIVO:".postln;
    "  ~tempo = 0.5             - Cambiar tempo".postln;
    "  ~baseFreq = 440          - Cambiar frecuencia base".postln;
    "  ~amplitude = 0.5         - Cambiar amplitud".postln;
    "".postln;
    "📝 ENVÍO DIRECTO:".postln;
    "  ~sendNote.value(freq, amp, dur)".postln;
    "  ~sendDrone.value(freq, amp, dur)".postln;
    "  ~sendCluster.value(freq, amp, dur, density)".postln;
    "".postln;
    "💡 Tip: Ejecuta ~showHelp.value para ver esta ayuda nuevamente".postln;
    "".postln;
};

// ====================================================================
// INICIALIZACIÓN
// ====================================================================

// Mostrar menú al cargar
//~menuEjemplos.value; // <-- Esta línea no está definida en este archivo, ¿quizás en otro?

"📝 Ejemplos de live coding cargados. Ejecuta ~menuEjemplos.value para ver la lista.".postln;
~sendNote.value(440.0, 0.5, 1.0); // Envío de nota al iniciar

~visualizer.sendMsg("/test", "SuperCollider está enviando!");
"Mensaje de prueba enviado desde SC".postln;

~testConnection.value // <-- Esta línea no está definida en este archivo, ¿quizás en otro?
