// Prueba de conexión OSC directa
// Este código prueba la comunicación con el visualizador

(
"=== PRUEBA DE CONEXIÓN OSC ===".postln;

// Crear conexión al puerto correcto
var testAddr = NetAddr.new("127.0.0.1", 57122);

"Enviando mensaje de prueba al puerto 57122...".postln;

// Enviar un mensaje de prueba
testAddr.sendMsg("/test", "hello", 123, 456.789);

"Enviando evento de prueba simple...".postln;

// Enviar un evento simple
testAddr.sendMsg("/event", "point", 440, 0.5, 1.0, 0.1, 0.1, 0.5, 0.0, 0.0, 220);

"Mensajes enviados. Verifica la terminal del visualizador.".postln;
"Si no aparecen mensajes en el visualizador, hay un problema de conexión.".postln;

)

// Verificar variables globales actuales
(
"=== VERIFICANDO CONFIGURACIÓN ACTUAL ===".postln;
if (~scvSendPoint.notNil, {
    "~scvSendPoint existe. Probando envío...".postln;
    ~scvSendPoint.(440, 0.5, 1.0);
}, {
    "~scvSendPoint NO existe. Necesitas recargar el código.".postln;
});
)
