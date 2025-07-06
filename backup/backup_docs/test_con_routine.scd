// test_con_routine.scd
// Script de prueba con Routine para espaciar mensajes

(
"=== PRUEBA DE PARSING FLEXIBLE CON ROUTINE ===".postln;

// Crear una Routine que permita usar .wait
Routine({
    var addr = NetAddr.new("127.0.0.1", 57122);
    
    "Enviando evento con flotantes...".postln;
    addr.sendMsg("/event", "point", 440.0, 0.7, 2.0);
    
    0.5.wait;
    
    "Enviando evento con enteros...".postln;
    addr.sendMsg("/event", "point", 880, 1, 1);
    
    0.5.wait;
    
    "Enviando evento con tipos mixtos...".postln;
    addr.sendMsg("/event", "point", 220.5, 0, 3);
    
    0.5.wait;
    
    "Enviando glissando con tipos mixtos...".postln;
    addr.sendMsg("/event", "gliss", 100, 1000.0, 0.8, 4);
    
    0.5.wait;
    
    "Enviando cluster con tipos mixtos...".postln;
    addr.sendMsg("/event", "cluster", 660, 150.5, 6, 0.3, 4.0);
    
    0.5.wait;
    
    "Enviando evento de ruido...".postln;
    addr.sendMsg("/event", "noise", 1000, 500.0, 0.4, 2.5);
    
    "Secuencia de prueba completada.".postln;
}).play; // .play ejecuta la Routine

)
