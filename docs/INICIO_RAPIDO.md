# 🚀 Inicio Rápido - Live Coding con SC Score Visualizer

## ⚡ Configuración en 5 minutos

### 1. Verificar Sistema
```bash
# Ejecutar verificación automática
./verificar_configuracion.sh
```

### 2. Iniciar Visualizador
```bash
# Ejecutar el visualizador
./start_visualizer.sh
```

### 3. Configurar SuperCollider
```supercollider
// En SuperCollider, cargar configuración
"setup_visualizer.scd".load;
```

### 4. Prueba Rápida
```supercollider
// Probar conexión
"test_conexion.scd".load;
```

### 5. Comenzar Live Coding
```supercollider
// Cargar ejemplos
"ejemplos_live_coding.scd".load;

// Ver menú de comandos
~showHelp.value;
```

## 🎵 Comandos Básicos

### Envío de Notas
```supercollider
// Nota simple
~sendNote.value(440, 0.5, 2.0);  // frecuencia, amplitud, duración

// Drone
~sendDrone.value(220, 0.3, 5.0);

// Cluster
~sendCluster.value(660, 0.5, 2.0, 8);
```

### Generadores
```supercollider
~startRandom.value;    // Iniciar generador aleatorio
~startMelody.value;    // Iniciar melodía
~startBass.value;      // Iniciar bajo
~stopAll.value;        // Detener todo
```

### Control en Tiempo Real
```supercollider
// Cambiar parámetros durante ejecución
~tempo = 0.25;         // Más rápido
~amplitude = 0.8;      // Más fuerte
~baseFreq = 330;       // Frecuencia diferente
```

## 🎹 Controles del Visualizador

| Tecla | Acción |
|-------|--------|
| `G` | Mostrar/ocultar grilla |
| `D` | Mostrar/ocultar debug |
| `P` | Pausar/reanudar |
| `H` | Mostrar/ocultar ayuda |
| `F` | Pantalla completa |
| `R` | Reiniciar |
| `ESC` | Salir |

## 🔧 Solución Rápida de Problemas

### Sin Visualización
```bash
# Verificar puerto
netstat -an | grep 7777

# Verificar configuración
cat config.toml | grep listen_port
```

### Error de Compilación
```bash
# Instalar dependencias
sudo apt-get install libasound2-dev pkg-config
cargo clean && cargo build --release
```

### SuperCollider No Conecta
```supercollider
// Recrear conexión
~visualizer = NetAddr("127.0.0.1", 7777);
~testConnection.value;
```

## 📚 Documentación Completa

Para información detallada, consultar:
- **Guía completa:** `docs/GUIA_LIVE_CODING.md`
- **Arquitectura:** `ARCHITECTURE.md`
- **README:** `README.md`

## 🎨 Ejemplos Listos para Usar

### Secuencia Simple
```supercollider
(
var escala = [60, 62, 64, 65, 67, 69, 71, 72].midicps;
Routine({
    escala.do({ |freq|
        ~sendNote.value(freq, 0.6, 0.5);
        0.5.wait;
    });
}).play;
)
```

### Ambiente Aleatorio
```supercollider
(
Routine({
    loop {
        ~sendNote.value(rrand(200, 2000), rrand(0.2, 0.8), rrand(0.5, 3.0));
        rrand(0.1, 1.0).wait;
    }
}).play;
)
```

### Texturas con Drones
```supercollider
(
// Drone base
~sendDrone.value(110, 0.4, 8.0);

// Notas esporádicas
Routine({
    loop {
        if (0.3.coin) {
            ~sendNote.value([330, 440, 523, 659].choose, 0.5, 2.0);
        };
        2.0.wait;
    }
}).play;
)
```

---

**¡Comienza tu sesión de live coding ahora!** 🎵✨

Para más ejemplos y técnicas avanzadas, consulta `ejemplos_live_coding.scd` y la documentación completa.