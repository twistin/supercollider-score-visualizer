// ejemplos_live_coding.scd - Ejemplos completos para sesiones de live coding

// ====================================================================
// EJEMPLO 1: SECUENCIA BÁSICA
// ====================================================================

// Cargar configuración primero
"setup_visualizer.scd".load;

// Secuencia simple de notas
(
~ejemplo1 = Routine({
    var escala = [60, 62, 64, 65, 67, 69, 71, 72].midicps;
    "🎵 Ejemplo 1: Secuencia básica".postln;
    
    escala.do({ |freq|
        ~sendNote.value(freq, 0.6, 0.5);
        0.5.wait;
    });
    
    "✅ Ejemplo 1 completado".postln;
}).play;
)

// ====================================================================
// EJEMPLO 2: GENERACIÓN ALEATORIA
// ====================================================================

(
~ejemplo2 = Routine({
    "🎲 Ejemplo 2: Generación aleatoria (10 segundos)".postln;
    
    var endTime = Main.elapsedTime + 10;
    while { Main.elapsedTime < endTime } {
        var freq = rrand(200, 2000);
        var amp = rrand(0.2, 0.8);
        var dur = rrand(0.3, 2.0);
        
        ~sendNote.value(freq, amp, dur);
        rrand(0.1, 0.8).wait;
    };
    
    "✅ Ejemplo 2 completado".postln;
}).play;
)

// ====================================================================
// EJEMPLO 3: CAPAS MÚLTIPLES
// ====================================================================

(
"🎶 Ejemplo 3: Capas múltiples".postln;

// Melodía principal
~melodia = Routine({
    var notas = [60, 64, 67, 72, 76].midicps;
    loop {
        ~sendNote.value(notas.choose, 0.7, 0.8);
        0.8.wait;
    }
}).play;

// Bajo
~bajo = Routine({
    var bajos = [36, 40, 43].midicps;
    loop {
        ~sendNote.value(bajos.choose, 0.5, 1.5);
        1.5.wait;
    }
}).play;

// Texturas ambientales
~texturas = Routine({
    loop {
        if (0.4.coin) {
            ~sendDrone.value(rrand(200, 400), 0.3, 3.0);
        };
        if (0.2.coin) {
            ~sendCluster.value(rrand(800, 1600), 0.4, 2.0, 6);
        };
        2.0.wait;
    }
}).play;

// Detener después de 20 segundos
SystemClock.sched(20, {
    ~melodia.stop;
    ~bajo.stop; 
    ~texturas.stop;
    "✅ Ejemplo 3 completado".postln;
});
)

// ====================================================================
// EJEMPLO 4: CONTROL DINÁMICO
// ====================================================================

(
"🎛️ Ejemplo 4: Control dinámico".postln;

// Variables que cambiarán durante la ejecución
~velocidad = 0.5;
~intensidad = 0.5;
~rango = 500;

~controlDinamico = Routine({
    loop {
        var freq = rrand(440 - ~rango, 440 + ~rango);
        ~sendNote.value(freq, ~intensidad, ~velocidad);
        ~velocidad.wait;
    }
}).play;

// Cambios automáticos cada 3 segundos
~cambios = Routine({
    loop {
        // Cambiar velocidad
        ~velocidad = rrand(0.2, 1.0);
        
        // Cambiar intensidad
        ~intensidad = rrand(0.3, 0.8);
        
        // Cambiar rango
        ~rango = rrand(200, 1000);
        
        ("⚡ Cambios: velocidad=" ++ ~velocidad.round(0.1) ++ 
         ", intensidad=" ++ ~intensidad.round(0.1) ++ 
         ", rango=" ++ ~rango).postln;
        
        3.0.wait;
    }
}).play;

// Detener después de 15 segundos
SystemClock.sched(15, {
    ~controlDinamico.stop;
    ~cambios.stop;
    "✅ Ejemplo 4 completado".postln;
});
)

// ====================================================================
// EJEMPLO 5: PATRONES RÍTMICOS
// ====================================================================

(
"🥁 Ejemplo 5: Patrones rítmicos".postln;

// Patrón de kick
~kick = Routine({
    loop {
        ~sendNote.value(60, 0.8, 0.2);
        1.0.wait;
    }
}).play;

// Patrón de snare
~snare = Routine({
    0.5.wait; // Offset
    loop {
        ~sendNote.value(200, 0.6, 0.1);
        1.0.wait;
    }
}).play;

// Hi-hats
~hihat = Routine({
    loop {
        4.do({
            ~sendNote.value(4000, 0.3, 0.05);
            0.25.wait;
        });
    }
}).play;

// Detener después de 12 segundos
SystemClock.sched(12, {
    ~kick.stop;
    ~snare.stop;
    ~hihat.stop;
    "✅ Ejemplo 5 completado".postln;
});
)

// ====================================================================
// EJEMPLO 6: AMBIENT DRONE
// ====================================================================

(
"🌊 Ejemplo 6: Ambient drone".postln;

// Drones principales
~drone1 = {
    ~sendDrone.value(110, 0.4, 8.0);
    8.0.wait;
};

~drone2 = {
    ~sendDrone.value(164.81, 0.3, 6.0); // E3
    6.0.wait;
};

~drone3 = {
    ~sendDrone.value(220, 0.2, 10.0); // A3
    10.0.wait;
};

// Ejecutar drones en paralelo
~ambientPattern = Routine({
    fork { loop { ~drone1.value } };
    fork { 2.0.wait; loop { ~drone2.value } };
    fork { 4.0.wait; loop { ~drone3.value } };
    
    // Agregar notas esporádicas
    loop {
        if (0.3.coin) {
            var freq = [330, 440, 523.25, 659.25].choose;
            ~sendNote.value(freq, 0.5, 3.0);
        };
        rrand(2.0, 6.0).wait;
    }
}).play;

// Detener después de 25 segundos
SystemClock.sched(25, {
    ~ambientPattern.stop;
    "✅ Ejemplo 6 completado".postln;
});
)

// ====================================================================
// EJEMPLO 7: INTERACTIVO
// ====================================================================

(
"🎮 Ejemplo 7: Sesión interactiva".postln;
"Usa las siguientes teclas para controlar:".postln;
"  q - Nota grave    w - Nota media    e - Nota aguda".postln;
"  a - Drone bajo    s - Drone medio   d - Drone alto".postln;
"  z - Cluster       x - Limpiar       c - Detener".postln;

// Mapeo de teclas (simulado con variables)
~tocarGrave = { ~sendNote.value(220, 0.6, 1.0); "🎵 Grave".postln; };
~tocarMedio = { ~sendNote.value(440, 0.6, 1.0); "🎵 Medio".postln; };
~tocarAgudo = { ~sendNote.value(880, 0.6, 1.0); "🎵 Agudo".postln; };

~droneBajo = { ~sendDrone.value(110, 0.4, 3.0); "🎵 Drone bajo".postln; };
~droneMedio = { ~sendDrone.value(220, 0.4, 3.0); "🎵 Drone medio".postln; };
~droneAlto = { ~sendDrone.value(440, 0.4, 3.0); "🎵 Drone alto".postln; };

~clusterFx = { ~sendCluster.value(660, 0.5, 2.0, 8); "💥 Cluster".postln; };

// Ejemplo de uso automático
Routine({
    5.do({ |i|
        ("Demostración automática " ++ (i+1)).postln;
        [~tocarGrave, ~tocarMedio, ~tocarAgudo].choose.value;
        1.0.wait;
    });
    
    "🎭 Modo interactivo: ejecuta las funciones manualmente".postln;
    "Ejemplo: ~tocarGrave.value".postln;
}).play;
)

// ====================================================================
// EJEMPLO 8: COMPOSICIÓN COMPLETA
// ====================================================================

(
"🎼 Ejemplo 8: Composición completa".postln;

// Estructura: Intro -> Desarrollo -> Clímax -> Outro
~composicion = Routine({
    
    // INTRO (8 segundos)
    "🎵 Intro...".postln;
    fork {
        4.do({
            ~sendNote.value([220, 277.18, 329.63].choose, 0.4, 2.0);
            2.0.wait;
        });
    };
    8.0.wait;
    
    // DESARROLLO (12 segundos)
    "🎵 Desarrollo...".postln;
    fork {
        var melodia = Routine({
            var notas = [440, 493.88, 523.25, 587.33].midicps;
            12.do({
                ~sendNote.value(notas.choose, 0.6, 0.8);
                1.0.wait;
            });
        }).play;
        
        var bajo = Routine({
            6.do({
                ~sendNote.value([110, 146.83].choose, 0.5, 2.0);
                2.0.wait;
            });
        }).play;
    };
    12.0.wait;
    
    // CLÍMAX (8 segundos)
    "🎵 Clímax...".postln;
    fork {
        4.do({
            ~sendCluster.value(rrand(400, 800), 0.7, 1.5, 10);
            ~sendNote.value(rrand(600, 1200), 0.8, 0.5);
            2.0.wait;
        });
    };
    8.0.wait;
    
    // OUTRO (6 segundos)
    "🎵 Outro...".postln;
    fork {
        ~sendDrone.value(220, 0.4, 6.0);
        3.0.wait;
        ~sendNote.value(440, 0.6, 3.0);
    };
    6.0.wait;
    
    "✅ Composición completada".postln;
}).play;
)

// ====================================================================
// FUNCIONES DE UTILIDAD PARA EJEMPLOS
// ====================================================================

// Detener todos los ejemplos
~detenerTodo = {
    [~ejemplo1, ~ejemplo2, ~melodia, ~bajo, ~texturas, 
     ~controlDinamico, ~cambios, ~kick, ~snare, ~hihat, 
     ~ambientPattern, ~composicion].do({ |routine|
        if (routine.notNil) { routine.stop };
    });
    ~clearAll.value;
    "🛑 Todos los ejemplos detenidos".postln;
};

// Menú de ejemplos
~menuEjemplos = {
    "".postln;
    "🎵 EJEMPLOS DE LIVE CODING".postln;
    "=========================".postln;
    "".postln;
    "Ejecuta el código de cada ejemplo seleccionando las líneas".postln;
    "y presionando Shift+Enter en SuperCollider".postln;
    "".postln;
    "📋 LISTA DE EJEMPLOS:".postln;
    "  1. Secuencia básica".postln;
    "  2. Generación aleatoria".postln;
    "  3. Capas múltiples".postln;
    "  4. Control dinámico".postln;
    "  5. Patrones rítmicos".postln;
    "  6. Ambient drone".postln;
    "  7. Interactivo".postln;
    "  8. Composición completa".postln;
    "".postln;
    "🛑 Para detener todo: ~detenerTodo.value".postln;
    "".postln;
};

// Mostrar menú al cargar
~menuEjemplos.value;

"📝 Ejemplos de live coding cargados. Ejecuta ~menuEjemplos.value para ver la lista.".postln;