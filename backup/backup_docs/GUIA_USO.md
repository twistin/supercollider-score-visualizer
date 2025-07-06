# Guía de Uso: SC Score Visualizer + SuperCollider

## Flujo de Trabajo Completo

### Paso 1: Preparar el Visualizador Rust

1. **Abrir terminal** en el directorio del proyecto:
   ```bash
   cd /Users/sdcarr/Documents/github/sc-score/sc_score_visualizer
   ```

2. **Compilar en modo release** (recomendado para mejor rendimiento):
   ```bash
   cargo build --release
   ```

3. **Ejecutar el visualizador**:
   ```bash
   cargo run --release
   ```

   O alternativamente, ejecutar directamente el binario:
   ```bash
   ./target/release/sc_score_visualizer
   ```

4. **Verificar que está funcionando**:
   - Debe aparecer una ventana negra con controles
   - En la terminal verás: `"OSC server listening on 127.0.0.1:57120"`
   - Los controles disponibles son:
     - `G`: Alternar grilla
     - `Z`: Zoom in
     - `X`: Zoom out
     - `S`: Capturar pantalla
     - `R`: Reset/limpiar pantalla
     - `Esc`: Salir

### Paso 2: Preparar SuperCollider

1. **Abrir SuperCollider** y iniciar el servidor:
   ```supercollider
   s.boot;
   ```

2. **Cargar el archivo de ejemplos**:
   - Método 1: Abrir `supercollider_examples.scd` en SuperCollider y ejecutar todo (Ctrl+A, Ctrl+Enter)
   - Método 2: Ejecutar en el IDE:
   ```supercollider
   ("supercollider_examples.scd").loadRelative;
   ```

3. **Verificar la conexión OSC**:
   ```supercollider
   // Probar conexión básica
   ~visualizer = NetAddr.new("127.0.0.1", 57120);
   ~visualizer.sendMsg("/event", "point", 440, 0.5, 2.0, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
   ```

### Paso 3: Realizar Pruebas

#### Prueba Básica (Recomendada para empezar)
```supercollider
// Ejecutar eventos básicos de prueba
~testBasicEvents.();
```

Deberías ver:
- Un punto inicial
- Un glissando después de 1 segundo
- Un cluster después de 2 segundos
- Ruido después de 3 segundos
- Una masa sonora después de 4 segundos

#### Pruebas Individuales
```supercollider
// Probar elementos por separado
~sendPoint.(440, 0.7, 3.0);           // Punto simple
~sendGliss.(220, 880, 0.8, 4.0);      // Glissando
~sendCluster.(440, 200, 12, 0.6, 5.0); // Cluster
~sendNoise.(1000, 600, 0.5, 3.0);     // Ruido
~sendSoundMass.(8, 0.6, 6.0);         // Masa sonora
```

#### Composición Completa Estilo Xenakis
```supercollider
// Ejecutar composición completa (15-20 minutos)
~xenakisComposition.();
```

### Paso 4: Experimentación Avanzada

#### Crear Eventos Personalizados
```supercollider
// Ejemplo de evento personalizado
~sendPoint.(
    freq: 523.25,      // C5
    amp: 0.8,          // Amplitud alta
    dur: 5.0,          // 5 segundos
    attack: 0.2,       // Ataque lento
    decay: 0.3,        // Decay medio
    density: 0.9,      // Densidad alta
    texture: 0.4,      // Textura media
    spread: 0.6,       // Spread medio
    hue: 120           // Verde
);
```

#### Patrones Automáticos
```supercollider
// Crear un patrón que envía eventos cada segundo
~myPattern = {
    inf.do { |i|
        var freq = 220 * (1.5 ** (i % 8));  // Escala pentatónica
        var hue = (i * 45) % 360;           // Rotar colores
        
        ~sendPoint.(freq, 0.5 + 0.3.rand, 2.0, 0.05, 0.1, 0.7, 0.2.rand, 0.3.rand, hue);
        1.0.wait;
    };
};

// Ejecutar el patrón
~myPattern.fork;
```

### Paso 5: Controles Interactivos

Mientras el visualizador está activo, puedes usar:

- **G**: Mostrar/ocultar grilla de referencia
- **Z/X**: Zoom in/out para ver detalles
- **S**: Capturar pantalla (guarda en el directorio del proyecto)
- **R**: Reset para limpiar la pantalla y empezar de nuevo
- **Mouse**: Mover la cámara (si implementado)

### Paso 6: Resolución de Problemas

#### El visualizador no recibe eventos:
1. Verificar que el puerto sea correcto (57120)
2. Comprobar firewall/antivirus
3. Reiniciar ambas aplicaciones

#### SuperCollider no puede enviar mensajes:
```supercollider
// Verificar conexión de red
~visualizer.sendMsg("/test");

// Verificar que el servidor esté iniciado
s.boot;
```

#### Eventos no se visualizan correctamente:
1. Presionar 'R' para reset
2. Verificar que los parámetros estén en rangos válidos
3. Comprobar la consola de Rust para errores

### Ejemplos de Sesiones Típicas

#### Sesión de Exploración (5-10 minutos)
```supercollider
// 1. Cargar ejemplos
("supercollider_examples.scd").loadRelative;

// 2. Prueba básica
~testBasicEvents.();

// 3. Experimentar con elementos individuales
~metastasisPoints.();
// Esperar un momento...
~pithopraktaGliss.();
```

#### Sesión de Composición (20-30 minutos)
```supercollider
// 1. Empezar con textura de fondo
~noiseTextures.();

// 2. Añadir elementos estructurales
{5.wait; ~stochasticClouds.();}.fork;

// 3. Crear líneas melódicas
{10.wait; ~pithopraktaGliss.();}.fork;

// 4. Finalizar con masas complejas
{15.wait; ~spectralMasses.();}.fork;
```

#### Sesión de Presentación (Demo completa)
```supercollider
// Ejecutar la composición completa de Xenakis
~xenakisComposition.();
```

### Parámetros de los Eventos

Todos los eventos soportan estos parámetros base:
- **freq**: Frecuencia fundamental (Hz)
- **amp**: Amplitud (0.0 - 1.0)
- **dur**: Duración (segundos)
- **density**: Densidad visual (0.0 - 1.0)
- **texture**: Rugosidad de la textura (0.0 - 1.0)
- **spread**: Dispersión espacial (0.0 - 1.0)
- **hue**: Color en grados (0-360)

Parámetros específicos por tipo:
- **point**: attack, decay
- **gliss**: startFreq, endFreq, curvature
- **cluster**: centerFreq, freqSpread, numVoices
- **noise**: centerFreq, bandwidth, grainSize, spectralTilt
- **mass**: numComponents, evolutionRate, turbulence + componentes espectrales

### Consejos para la Creatividad

1. **Empezar simple**: Usa ~testBasicEvents.() para familiarizarte
2. **Experimentar con parámetros**: Modifica valores extremos para ver el efecto
3. **Combinar elementos**: Ejecuta varias funciones en paralelo
4. **Usar el zoom**: 'Z' y 'X' para ver detalles diferentes
5. **Capturar momentos**: 'S' para guardar imágenes interesantes
6. **Reset frecuente**: 'R' para empezar limpio

¡Disfruta explorando la visualización de partituras gráficas estilo Xenakis!
