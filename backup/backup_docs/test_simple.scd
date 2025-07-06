// Test simple para verificar parsing OSC flexible
(
var addr = NetAddr.new("127.0.0.1", 57122);

"=== PRUEBA DE PARSING FLEXIBLE ===".postln;

// Enviar mensajes con tipos mixtos
"Enviando evento con flotantes...".postln;
addr.sendMsg("/event", "point", 440.0, 0.7, 2.0);

0.5.wait;

"Enviando evento con enteros...".postln;
addr.sendMsg("/event", "point", 880, 1, 1);

0.5.wait;

"Enviando evento con tipos mixtos...".postln;
addr.sendMsg("/event", "point", 220.5, 0, 3);

0.5.wait;

"Enviando glissando...".postln;
addr.sendMsg("/event", "gliss", 100, 1000, 0.8, 4.0);

"Mensajes enviados. Verifica el visualizador.".postln;
)
