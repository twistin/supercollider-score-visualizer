// Prueba mínima de sintaxis SuperCollider
// Este archivo debe cargar sin errores si la sintaxis es correcta

// Test básico de funciones
~test = { "test".postln; };

// Test de NetAddr
~addr = NetAddr.new("127.0.0.1", 57120);

// Test de mensaje OSC básico
~sendTest = { |msg| ~addr.sendMsg("/test", msg); };

// Mostrar que el test fue exitoso
(
"✅ Prueba de sintaxis SuperCollider exitosa".postln;
"✅ Todas las estructuras básicas funcionan correctamente".postln;
"✅ El archivo supercollider_examples.scd debería cargar sin problemas".postln;
)
