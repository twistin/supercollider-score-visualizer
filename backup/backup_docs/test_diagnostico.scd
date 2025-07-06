// Diagnóstico rápido para SC Score Visualizer
// Ejecuta este código línea por línea para diagnosticar problemas

// Paso 1: Verificar que SuperCollider está funcionando
"SuperCollider está funcionando correctamente".postln;

// Paso 2: Verificar conectividad OSC
(
var testAddr = NetAddr.new("127.0.0.1", 57120);
"Enviando mensaje de prueba OSC...".postln;
testAddr.sendMsg("/test", "hello");
"Mensaje OSC enviado".postln;
)

// Paso 3: Cargar funciones básicas mínimas
(
var visualizer, testFunction;

"Cargando funciones básicas...".postln;

// Configurar visualizador
visualizer = NetAddr.new("127.0.0.1", 57120);

// Función de prueba simple
testFunction = {
    "Enviando evento de prueba...".postln;
    visualizer.sendMsg("/event", "point", 440, 0.5, 1.0, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
    "Evento enviado".postln;
};

// Exportar a global
~scvTest = testFunction;

"Función ~scvTest cargada exitosamente".postln;
"Ejecuta: ~scvTest.() para probar".postln;
)

// Paso 4: Verificar que la función está disponible
(
if (~scvTest.notNil, {
    "✓ ~scvTest está definida".postln;
}, {
    "✗ ~scvTest NO está definida".postln;
});
)

// Paso 5: Listar todas las variables globales que empiecen con ~scv
(
"Variables globales ~scv*:".postln;
currentEnvironment.keys.do { |key|
    if (key.asString.beginsWith("scv"), {
        ("  " ++ key).postln;
    });
};
)
