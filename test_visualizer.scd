// test_visualizer.scd
// Enviar eventos OSC para probar el visualizador Rust/Nannou
(
// Configura el destino OSC (ajusta el puerto si es necesario)
~host = "127.0.0.1";
~port = 6001; // Debe coincidir con el puerto del visualizador
~osc = NetAddr(~host, ~port);

// Enviar una nota (Note) como círculo
~osc.sendMsg("/note", 440, 0.8, 0.5); // frecuencia, amplitud, duración

// Enviar un drone (Drone) como rectángulo
~osc.sendMsg("/drone", 110, 0.5, 4.0); // frecuencia, amplitud, duración

// Enviar un cluster (Cluster) como línea
~osc.sendMsg("/cluster", 300, 0.7, 0.6, 2.0); // centro frecuencia, densidad, amplitud, duración

// Enviar varios eventos para ver animación
Routine({
    10.do { |i|
        ~osc.sendMsg("/note", 220 + i*30, 0.5 + 0.05*i, 0.7);
        0.2.wait;
    }
}).play;
)
