// =================================================================
// SC SCORE VISUALIZER - SISTEMA DE AUTO-VISUALIZACIÓN PROFESIONAL
// =================================================================
// 
// Script autocontenido para live coding con visualización automática
// Compatible con ProxySpace, Pbind, Synth y NodeProxy
// 
// INSTALACIÓN: Ejecutar paso a paso los bloques marcados con números
// USO DIARIO: Solo ejecutar bloque 6 para activar el sistema
// 
// =================================================================

// =================================================================
// 1. CONFIGURACIÓN GLOBAL
// =================================================================
(
// Configuración del visualizador
~visualizerConfig = (
    addr: NetAddr("127.0.0.1", 57124),
    enabled: true,
    debugMode: false,
    analyzerSynth: nil
);

// Configuración lista - la conexión se verificará al enviar mensajes
"✅ Configuración OSC lista para SC Score Visualizer (puerto 57124)".postln;

"🎯 Configuración cargada. Siguiente paso: ejecutar bloque 2".postln;
)

// =================================================================
// 2. SYNTHDEF DEL ANALIZADOR EN TIEMPO REAL
// =================================================================
(
SynthDef(\sc_visualizer_analyzer, {
    var input, amp, freq, centroid, flatness;
    var onset, hasOnset, localAmp;
    var trig, data;
    
    // Capturar TODO el audio del servidor (estéreo -> mono)
    input = In.ar(0, 2).sum;
    
    // Análisis básico
    amp = Amplitude.kr(input, 0.01, 0.1);
    freq = Pitch.kr(input, ampThreshold: 0.02, median: 7)[0];
    
    // Análisis espectral
    centroid = SpecCentroid.kr(FFT(LocalBuf(2048), input));
    flatness = SpecFlatness.kr(FFT(LocalBuf(2048), input));
    
    // Detección de onset básica
    localAmp = Amplitude.kr(input, 0.001, 0.01);
    onset = localAmp > (DelayN.kr(localAmp, 0.1, 0.1) * 1.5);
    hasOnset = Trig1.kr(onset, 0.1);
    
    // Trigger a 30 FPS para datos continuos
    trig = Impulse.kr(30);
    
    // Envío de datos de análisis en tiempo real
    SendReply.kr(trig, '/realtime_audio', [
        freq.clip(20, 20000),
        amp.clip(0, 1),
        centroid.clip(20, 20000),
        flatness.clip(0, 1),
        hasOnset
    ]);
    
    // Silenciar la salida (solo análisis)
    Out.ar(0, Silent.ar(2));
}).add;

"🔬 SynthDef del analizador cargado. Siguiente paso: ejecutar bloque 3".postln;
)

// =================================================================
// 3. FUNCIONES HELPER PARA AUTO-VISUALIZACIÓN
// =================================================================
(
// Helper para Pbind con visualización automática
~visualPbind = { |...args|
    var pbindArgs, hasCallback, callbackIndex;
    var visualCallback;
    
    pbindArgs = args ?? [];
    
    // Verificar si ya tiene callback
    hasCallback = false;
    callbackIndex = -1;
    pbindArgs.pairsDo({ |key, value, i|
        if(key == \callback, {
            hasCallback = true;
            callbackIndex = i + 1;
        });
    });
    
    // Crear callback de visualización
    visualCallback = { |event|
        var freq, amp, dur, rel, timbre, curvature, cf;
        
        if(~visualizerConfig.enabled, {
            freq = event[\freq] ?? 440;
            amp = event[\amp] ?? 0.5;
            dur = event[\dur] ?? 1.0;
            rel = event[\rel] ?? dur;
            timbre = event[\timbre] ?? 0.5;
            curvature = event[\curvature] ?? 0.0;
            cf = event[\cf] ?? freq;
            
            // Enviar evento OSC
            ~visualizerConfig.addr.sendMsg("/event", "pbind", 
                freq.asFloat, amp.asFloat, rel.asFloat, 
                timbre.asFloat, curvature.asFloat, cf.asFloat);
                
            if(~visualizerConfig.debugMode, {
                "📊 Visual event: freq:% amp:% dur:%".format(freq, amp, rel).postln;
            });
        });
    };
    
    // Si no tiene callback, agregarlo
    if(hasCallback.not, {
        pbindArgs = pbindArgs ++ [\callback, visualCallback];
    });
    
    // Crear y devolver el Pbind
    Pbind(*pbindArgs);
};

// Helper para Synth con visualización automática
~visualSynth = { |defName, args|
    var freq, amp, dur;
    var argsList, i, synth;
    
    argsList = args ?? [];
    
    // Extraer parámetros comunes usando búsqueda manual
    freq = 440;
    amp = 0.5;
    dur = 1.0;
    
    // Buscar freq, amp, dur en la lista de argumentos
    i = 0;
    while({ i < (argsList.size - 1) }, {
        if(argsList[i] == \freq, { freq = argsList[i + 1] });
        if(argsList[i] == \amp, { amp = argsList[i + 1] });
        if(argsList[i] == \dur, { dur = argsList[i + 1] });
        i = i + 2;
    });
    
    // Crear el synth
    synth = Synth(defName, argsList);
    
    // Enviar visualización si está habilitada
    if(~visualizerConfig.enabled, {
        ~visualizerConfig.addr.sendMsg("/event", "synth", 
            freq.asFloat, amp.asFloat, dur.asFloat, 0.5, 0.0, freq.asFloat);
            
        if(~visualizerConfig.debugMode, {
            "🎵 Visual synth: % freq:% amp:%".format(defName, freq, amp).postln;
        });
    });
    
    synth;
};

// Helper para NodeProxy con visualización automática
~visualProxy = { |proxy, freq, amp|
    var proxyFreq, proxyAmp;
    
    proxyFreq = freq ?? 440;
    proxyAmp = amp ?? 0.5;
    
    // Reproducir el proxy
    proxy.play;
    
    // Enviar visualización si está habilitada
    if(~visualizerConfig.enabled, {
        ~visualizerConfig.addr.sendMsg("/event", "proxy", 
            proxyFreq.asFloat, proxyAmp.asFloat, 2.0, 0.5, 0.0, proxyFreq.asFloat);
            
        if(~visualizerConfig.debugMode, {
            "🔗 Visual proxy: freq:% amp:%".format(proxyFreq, proxyAmp).postln;
        });
    });
    
    proxy;
};

"🛠️ Funciones helper cargadas. Siguiente paso: ejecutar bloque 4".postln;
)

// =================================================================
// 4. SISTEMA DE CONTROL Y UTILIDADES
// =================================================================
(
// Activar visualizador
~visualizerOn = {
    ~visualizerConfig.enabled = true;
    "✅ Visualizador ACTIVADO".postln;
};

// Desactivar visualizador
~visualizerOff = {
    ~visualizerConfig.enabled = false;
    "❌ Visualizador DESACTIVADO".postln;
};

// Modo debug
~visualizerDebug = { |enable|
    enable = enable ?? true;
    ~visualizerConfig.debugMode = enable;
    if(enable, {
        "🐛 Modo debug ACTIVADO".postln;
    }, {
        "🐛 Modo debug DESACTIVADO".postln;
    });
};

// Iniciar analizador en tiempo real
~startAnalyzer = {
    if(~visualizerConfig.analyzerSynth.notNil, {
        ~visualizerConfig.analyzerSynth.free;
    });
    
    ~visualizerConfig.analyzerSynth = Synth(\sc_visualizer_analyzer);
    
    "🔬 Analizador en tiempo real iniciado".postln;
};

// Limpiar recursos del visualizador
~cleanupVisualizer = {
    if(~visualizerConfig.analyzerSynth.notNil, {
        ~visualizerConfig.analyzerSynth.free;
        ~visualizerConfig.analyzerSynth = nil;
    });
    
    "🧹 Recursos del visualizador liberados".postln;
};

// Test de conectividad
~testVisualizer = {
    if(~visualizerConfig.enabled, {
        ~visualizerConfig.addr.sendMsg("/event", "test", 440.0, 0.5, 1.0, 0.5, 0.0, 440.0);
        "🧪 Mensaje de prueba enviado al visualizador".postln;
    }, {
        "⚠️ Visualizador desactivado. Usar ~visualizerOn.value() primero".postln;
    });
};

"🎛️ Sistema de control cargado. Siguiente paso: ejecutar bloque 5".postln;
)

// =================================================================
// 5. CONFIGURACIÓN DE PROXYSPACE Y CLEANUP
// =================================================================
(
// Configurar ProxySpace si no está activo
if(currentEnvironment.isKindOf(ProxySpace).not, {
    p = ProxySpace.push(s);
    "🌐 ProxySpace activado".postln;
});

// Configurar cleanup automático al cerrar servidor
ServerQuit.add({
    ~cleanupVisualizer.value();
    "🔄 Cleanup automático ejecutado".postln;
});

"🔧 ProxySpace configurado. Siguiente paso: ejecutar bloque 6".postln;
)

// =================================================================
// 6. ACTIVACIÓN DEL SISTEMA (USAR PARA SESIONES DIARIAS)
// =================================================================
(
// Activar todo el sistema
~visualizerOn.value();
~startAnalyzer.value();

// Test de conectividad
{
    1.wait;
    ~testVisualizer.value();
}.fork;

"🚀 SISTEMA DE AUTO-VISUALIZACIÓN ACTIVADO";
"";
"📋 FUNCIONES DISPONIBLES:";
"   • ~visualPbind.value(...) - Pbind con visualización";
"   • ~visualSynth.value(defName, args) - Synth con visualización";
"   • ~visualProxy.value(proxy, freq, amp) - NodeProxy con visualización";
"";
"🎛️ CONTROLES:";
"   • ~visualizerOn.value() - Activar";
"   • ~visualizerOff.value() - Desactivar";
"   • ~visualizerDebug.value(true/false) - Debug";
"   • ~testVisualizer.value() - Probar conexión";
"";
"🎵 ¡Listo para live coding con visualización automática!";
)

// =================================================================
// 7. EJEMPLOS DE USO PRÁCTICO
// =================================================================

/*
// EJEMPLO 1: Pbind con visualización automática
(
a = ~visualPbind.value(
    \instrument, \default,
    \freq, Pseq([220, 330, 440, 550], inf),
    \dur, 0.25,
    \amp, 0.3,
    \rel, 0.1
);
a.play;
)

// EJEMPLO 2: Synth individual con visualización
~visualSynth.value(\default, [\freq, 880, \amp, 0.4, \dur, 2]);

// EJEMPLO 3: NodeProxy con visualización
(
~pad = { SinOsc.ar([220, 223]) * 0.1 * SinOsc.kr(0.1).range(0.5, 1) };
~visualProxy.value(~pad, 220, 0.1);
)

// EJEMPLO 4: Secuencia compleja
(
// Patrón rítmico
b = ~visualPbind.value(
    \instrument, \default,
    \freq, Pseq([110, 165, 220], inf),
    \dur, Pseq([0.25, 0.125, 0.125], inf),
    \amp, Pseq([0.6, 0.3, 0.4], inf),
    \timbre, Pseq([0.2, 0.8, 0.5], inf)
);

// Pad atmosférico
~ambient = { 
    var freqs = [220, 220*1.5, 220*2];
    var oscs = freqs.collect({ |f| SinOsc.ar(f + SinOsc.kr(0.1).range(-2, 2)) });
    Splay.ar(oscs) * 0.05 * SinOsc.kr(0.05).range(0.3, 1);
};

// Iniciar todo
b.play;
~visualProxy.value(~ambient, 220, 0.05);
)

// CONTROL EN VIVO
~visualizerDebug.value(true);  // Ver eventos en el post window
~visualizerOff.value();        // Pausar visualización
~visualizerOn.value();         // Reanudar visualización

// CLEANUP MANUAL
~cleanupVisualizer.value();    // Liberar recursos
*/

// =================================================================
// 🎊 SISTEMA LISTO PARA LIVE CODING PROFESIONAL
// =================================================================
//
// MODO DE USO RÁPIDO PARA SESIONES DIARIAS:
// 1. Ejecutar solo el bloque 6 para activar todo
// 2. Usar ~visualPbind, ~visualSynth, ~visualProxy normalmente
// 3. Live coding como siempre, ¡pero con visualización automática!
//
// TROUBLESHOOTING:
// - Si no ves visualización: verificar que el visualizador esté ejecutándose
// - Si hay errores OSC: ejecutar ~testVisualizer.value()
// - Para debug: ~visualizerDebug.value(true)
//
// =================================================================
