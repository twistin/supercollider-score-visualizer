// SC Score Visualizer - Versión robusta con mejor manejo de errores
// Este archivo es una versión mejorada que garantiza la exportación correcta de funciones

// INSTRUCCIONES:
// 1. Ejecuta todo el bloque entre paréntesis ( ... )
// 2. Verifica que aparezca el mensaje "✓ Todas las funciones cargadas exitosamente"
// 3. Usa las funciones ~scv* para enviar eventos

(
// =========================================
// CONFIGURACIÓN INICIAL
// =========================================

"Iniciando carga de SC Score Visualizer...".postln;

// Configurar la dirección del visualizador
var visualizer = NetAddr.new("127.0.0.1", 57120);

// =========================================
// FUNCIONES DE ENVÍO DE EVENTOS
// =========================================

// Función para enviar un evento puntual
var sendPoint = { |freq=440, amp=0.5, dur=1.0, attack=0.1, decay=0.1, density=0.5, texture=0.0, spread=0.0, hue=220|
    visualizer.sendMsg("/event", "point", freq, amp, dur, attack, decay, density, texture, spread, hue);
    ("Enviado: punto freq=" ++ freq ++ " amp=" ++ amp).postln;
};

// Función para enviar un glissando
var sendGliss = { |startFreq=440, endFreq=880, amp=0.7, dur=2.0, curvature=0.0, density=0.7, texture=0.0, spread=0.0, hue=30|
    visualizer.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur, curvature, density, texture, spread, hue);
    ("Enviado: glissando " ++ startFreq ++ " → " ++ endFreq).postln;
};

// Función para enviar un cluster
var sendCluster = { |centerFreq=440, freqSpread=100, numVoices=8, amp=0.6, dur=3.0, density=0.8, texture=0.3, spread=0.5, hue=120|
    visualizer.sendMsg("/event", "cluster", centerFreq, freqSpread, numVoices, amp, dur, density, texture, spread, hue);
    ("Enviado: cluster centro=" ++ centerFreq ++ " voces=" ++ numVoices).postln;
};

// Función para enviar ruido
var sendNoise = { |centerFreq=1000, bandwidth=500, amp=0.4, dur=2.0, grainSize=1.0, spectralTilt=0.0, density=0.6, texture=0.8, spread=0.4, hue=300|
    visualizer.sendMsg("/event", "noise", centerFreq, bandwidth, amp, dur, grainSize, spectralTilt, density, texture, spread, hue);
    ("Enviado: ruido centro=" ++ centerFreq ++ " ancho=" ++ bandwidth).postln;
};

// Función para enviar masa sonora
var sendSoundMass = { |numComponents=5, amp=0.5, dur=4.0, evolutionRate=0.5, turbulence=0.3, density=0.9, texture=0.5, spread=0.7, hue=60|
    var msg = ["/event", "mass", numComponents, amp, dur, evolutionRate, turbulence, density, texture, spread, hue];
    
    // Añadir componentes espectrales
    numComponents.do { |i|
        var freq = 440 * (1 + (i * 0.618)); // Proporción áurea
        var componentAmp = 1.0 / (i + 1);
        msg = msg ++ [freq, componentAmp];
    };
    
    visualizer.sendMsg(*msg);
    ("Enviado: masa sonora con " ++ numComponents ++ " componentes").postln;
};

// =========================================
// FUNCIONES DE COMPOSICIÓN
// =========================================

// Función de prueba básica
var testBasicEvents = {
    "=== Iniciando prueba básica de eventos ===".postln;
    
    // Enviar punto
    "1/5: Enviando punto...".postln;
    sendPoint.(440, 0.5, 2.0);
    
    // Enviar glissando después de 1 segundo
    {
        1.wait; 
        "2/5: Enviando glissando...".postln;
        sendGliss.(220, 880, 0.7, 3.0, 0.5);
    }.fork;
    
    // Enviar cluster después de 2 segundos
    {
        2.wait; 
        "3/5: Enviando cluster...".postln;
        sendCluster.(660, 150, 8, 0.6, 4.0);
    }.fork;
    
    // Enviar ruido después de 3 segundos
    {
        3.wait; 
        "4/5: Enviando ruido...".postln;
        sendNoise.(1000, 500, 0.4, 2.5);
    }.fork;
    
    // Enviar masa sonora después de 4 segundos
    {
        4.wait; 
        "5/5: Enviando masa sonora...".postln;
        sendSoundMass.(6, 0.5, 3.0);
        {
            3.wait;
            "=== Prueba básica completada ===".postln;
        }.fork;
    }.fork;
};

// Secuencia de puntos dispersos (como en Metastaseis)
var metastasisPoints = {
    "=== Iniciando Metastasis Points ===".postln;
    var numPoints = 20; // Reducido para prueba más rápida
    var timeSpan = 8.0;

    numPoints.do { |i|
        var time = i / numPoints * timeSpan;
        var freq = 200 + (4000 * i / numPoints) + 200.rand;
        var amp = 0.3 + 0.4.rand;
        var density = i / numPoints;
        var texture = (i / numPoints).squared;

        {
            time.wait;
            sendPoint.(freq, amp, 0.5 + 1.0.rand, 0.05, 0.1, density, texture, 0.2.rand, 360.rand);
        }.fork;
    };
    
    {
        timeSpan.wait;
        "=== Metastasis Points completado ===".postln;
    }.fork;
};

// Glissandi convergentes/divergentes (como en Pithoprakta)
var pithopraktaGliss = {
    "=== Iniciando Pithoprakta Gliss ===".postln;
    var numGliss = 8; // Reducido para prueba
    var centerFreq = 440;

    numGliss.do { |i|
        var startTime = i * 0.5;
        var startFreq = centerFreq * (0.5 + 1.5.rand);
        var endFreq = centerFreq * (0.5 + 1.5.rand);
        var curvature = -1.0 + 2.0.rand;
        var duration = 2.0 + 1.0.rand;

        {
            startTime.wait;
            sendGliss.(startFreq, endFreq, 0.4 + 0.3.rand, duration, curvature, 0.8, 0.2.rand, 0.3.rand, 30 + 60.rand);
        }.fork;
    };
    
    {
        6.wait;
        "=== Pithoprakta Gliss completado ===".postln;
    }.fork;
};

// =========================================
// EXPORTAR FUNCIONES A VARIABLES GLOBALES
// =========================================

"Exportando funciones a variables globales...".postln;

// Funciones de envío individual
~scvSendPoint = sendPoint;
~scvSendGliss = sendGliss;
~scvSendCluster = sendCluster;
~scvSendNoise = sendNoise;
~scvSendSoundMass = sendSoundMass;

// Funciones de composición
~scvTestBasicEvents = testBasicEvents;
~scvMetastasisPoints = metastasisPoints;
~scvPithopraktaGliss = pithopraktaGliss;

// Variable para el visualizador (por si se necesita acceso directo)
~scvVisualizer = visualizer;

// =========================================
// VERIFICACIÓN Y MENSAJES FINALES
// =========================================

"".postln;
"✓ ✓ ✓ SC Score Visualizer cargado exitosamente! ✓ ✓ ✓".postln;
"".postln;
"Funciones disponibles:".postln;
"".postln;
"FUNCIONES DE PRUEBA:".postln;
"~scvTestBasicEvents.()       - Prueba básica de todos los tipos de eventos".postln;
"~scvMetastasisPoints.()      - Secuencia de puntos dispersos (estilo Xenakis)".postln;
"~scvPithopraktaGliss.()      - Glissandi convergentes/divergentes".postln;
"".postln;
"FUNCIONES DE ENVÍO INDIVIDUAL:".postln;
"~scvSendPoint.(freq, amp, dur, ...)      - Enviar punto individual".postln;
"~scvSendGliss.(startF, endF, amp, ...)   - Enviar glissando".postln;
"~scvSendCluster.(centerF, spread, ...)   - Enviar cluster".postln;
"~scvSendNoise.(centerF, bandwidth, ...)  - Enviar ruido/textura".postln;
"~scvSendSoundMass.(numComp, amp, ...)    - Enviar masa sonora".postln;
"".postln;
"EJEMPLO RÁPIDO:".postln;
"~scvSendPoint.(440, 0.5, 2.0);           - Punto simple de 440Hz".postln;
"~scvTestBasicEvents.();                  - Secuencia de prueba completa".postln;
"".postln;
"IMPORTANTE: Asegúrate de que el visualizador Rust esté ejecutándose".postln;
"en el puerto 57120 antes de enviar eventos.".postln;
"".postln;

// Verificar que las funciones están realmente disponibles
var functionNames = ["scvTestBasicEvents", "scvSendPoint", "scvSendGliss", "scvSendCluster", "scvSendNoise", "scvSendSoundMass"];
var allOk = true;

functionNames.do { |name|
    var globalVar = currentEnvironment.at(name.asSymbol);
    if (globalVar.isNil, {
        ("✗ ERROR: " ++ name ++ " no está disponible").postln;
        allOk = false;
    }, {
        ("✓ " ++ name ++ " está disponible").postln;
    });
};

if (allOk, {
    "".postln;
    "🎉 TODAS LAS FUNCIONES ESTÁN LISTAS PARA USAR 🎉".postln;
    "".postln;
    "Para comenzar, ejecuta: ~scvTestBasicEvents.()".postln;
}, {
    "".postln;
    "❌ ALGUNAS FUNCIONES NO SE CARGARON CORRECTAMENTE".postln;
    "Intenta ejecutar el bloque completo nuevamente.".postln;
});

) // Fin del bloque principal

// =========================================
// INSTRUCCIONES ADICIONALES
// =========================================

/*
SOLUCIÓN DE PROBLEMAS:

1. Si ves "Variable 'scvTestBasicEvents' not defined":
   - Asegúrate de ejecutar TODO el bloque ( ... ) de arriba
   - Verifica que aparezca el mensaje "TODAS LAS FUNCIONES ESTÁN LISTAS"

2. Si no recibes respuesta del visualizador:
   - Verifica que el programa Rust esté ejecutándose
   - Ejecuta: cargo run (en el directorio del proyecto)
   - El visualizador debe estar escuchando en puerto 57120

3. Para debug OSC:
   - ~scvVisualizer.sendMsg("/test", "hello");
   - Esto envía un mensaje de prueba

4. Para limpiar y recargar:
   - Ejecuta el bloque completo nuevamente
   - Las variables globales se sobrescribirán

*/
