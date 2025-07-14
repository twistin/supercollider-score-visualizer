// test_conexion.scd - Prueba rápida de conexión con el visualizador

// ====================================================================
// PRUEBA RÁPIDA DE CONEXIÓN
// ====================================================================

// Configurar NetAddr
~visualizer = NetAddr("127.0.0.1", 7777);

// Función de prueba
~probarConexion = {
    "🔍 Probando conexión con SC Score Visualizer...".postln;
    "".postln;
    
    // Prueba 1: Nota simple
    "1. Enviando nota simple (440 Hz)...".postln;
    ~visualizer.sendMsg("/note", 440, 0.5, 2.0);
    
    // Esperar un poco
    {
        // Prueba 2: Secuencia rápida
        "2. Enviando secuencia rápida...".postln;
        5.do({ |i|
            ~visualizer.sendMsg("/note", 440 + (i * 110), 0.6, 0.5);
            0.5.wait;
        });
        
        // Prueba 3: Drone
        "3. Enviando drone...".postln;
        ~visualizer.sendMsg("/drone", 220, 0.4, 3.0);
        
        // Prueba 4: Cluster
        2.0.wait;
        "4. Enviando cluster...".postln;
        ~visualizer.sendMsg("/cluster", 660, 0.5, 2.0, 8);
        
        // Finalizar
        3.0.wait;
        "✅ Prueba de conexión completada".postln;
        "".postln;
        "Si viste visualizaciones, la conexión funciona correctamente.".postln;
        "Si no viste nada, verifica que el visualizador esté ejecutándose.".postln;
        
    }.fork;
};

// Ejecutar prueba
~probarConexion.value;