# INSTRUCCIONES RÁPIDAS - SC Score Visualizer

## 🚀 Inicio Rápido (3 pasos)

### 1. Iniciar el Visualizador
```bash
./start_visualizer.sh
```
O manualmente:
```bash
cargo run --release
```

### 2. Iniciar SuperCollider y cargar ejemplos
En SuperCollider:
```supercollider
s.boot;  // Iniciar servidor
("supercollider_examples_fixed.scd").loadRelative;  // Cargar ejemplos CORREGIDOS
```

### 3. Probar la conexión
En SuperCollider:
```supercollider
~scvTestBasicEvents.();  // Prueba básica de 5 eventos (formato corregido)
```

## ⚠️ IMPORTANTE - Formato de Funciones Actualizado

**Las funciones ahora usan el prefijo `~scv` para evitar conflictos:**
- ✅ Usa: `~scvTestBasicEvents.()`  
- ❌ No uses: `scvTestBasicEvents.()`

## 🎹 Ejemplos Listos para Usar

```supercollider
// Composición completa estilo Xenakis (15-20 minutos)
~scvXenakisComposition.();

// Elementos individuales
~scvMetastasisPoints.();      // Puntos dispersos como Metastaseis
~scvPithopraktaGliss.();      // Glissandi como Pithoprakta  
~scvStochasticClouds.();      // Nubes de clusters estocásticos
~scvNoiseTextures.();         // Texturas de ruido evolutivas
~scvSpectralMasses.();        // Masas sonoras complejas
~scvRhythmicPatterns.();      // Patrones rítmicos complejos
```

## 🎛️ Controles del Visualizador

- **G**: Mostrar/ocultar grilla
- **Z/X**: Zoom in/out
- **S**: Capturar pantalla
- **R**: Reset/limpiar pantalla
- **Esc**: Salir

## 🔧 Verificación de Problemas

Si algo no funciona:

1. **Verificar conexión OSC:**
   ```bash
   python3 verify_integration.py
   ```

2. **Verificar que SuperCollider esté funcionando:**
   ```supercollider
   s.boot;
   // Probar envío directo (usando funciones del archivo corregido)
   ~scvSendPoint.(440, 0.5, 2.0);
   ```

3. **Reiniciar todo:**
   - Cerrar visualizador (Esc)
   - Reiniciar SuperCollider
   - Ejecutar `./start_visualizer.sh` nuevamente

## 📝 Eventos Personalizados

Estructura básica (usando funciones del archivo corregido):
```supercollider
~scvSendPoint.(freq, amp, dur, attack, decay, density, texture, spread, hue);
~scvSendGliss.(startFreq, endFreq, amp, dur, curvature, density, texture, spread, hue);
~scvSendCluster.(centerFreq, freqSpread, numVoices, amp, dur, density, texture, spread, hue);
~scvSendNoise.(centerFreq, bandwidth, amp, dur, grainSize, spectralTilt, density, texture, spread, hue);
~scvSendSoundMass.(numComponents, amp, dur, evolutionRate, turbulence, density, texture, spread, hue);
```

## 🎨 Ejemplos Creativos

### Secuencia Simple
```supercollider
// Arpeggio ascendente colorido
(1..12).do { |i|
    var freq = 220 * (2 ** (i/12));
    var hue = i * 30;
    ~scvSendPoint.(freq, 0.6, 1.0, 0.1, 0.2, 0.7, 0.1, 0.2, hue);
    0.5.wait;
};
```

### Patrón Continuo
```supercollider
// Patrón infinito de eventos aleatorios
(
var randomPattern = {
    inf.do {
        var eventType = ["point", "gliss", "cluster"].choose;
        
        switch(eventType,
            "point", { ~scvSendPoint.(200 + 1000.rand, 0.3 + 0.4.rand, 1.0 + 2.0.rand) },
            "gliss", { ~scvSendGliss.(200 + 500.rand, 400 + 800.rand, 0.5, 2.0 + 3.0.rand) },
            "cluster", { ~scvSendCluster.(300 + 700.rand, 50 + 150.rand, 4 + 8.rand, 0.4, 3.0) }
        );
        
        (0.5 + 1.5.rand).wait;
    };
};

randomPattern.fork;  // Iniciar patrón
)

## 📊 Parámetros de los Eventos

| Parámetro | Rango | Descripción |
|-----------|-------|-------------|
| freq | 20-20000 | Frecuencia en Hz |
| amp | 0.0-1.0 | Amplitud |
| dur | 0.1-∞ | Duración en segundos |
| density | 0.0-1.0 | Densidad visual |
| texture | 0.0-1.0 | Rugosidad |
| spread | 0.0-1.0 | Dispersión espacial |
| hue | 0-360 | Color en grados |

## 🎯 Objetivos de Uso

1. **Exploración**: Usa `~scvTestBasicEvents.()` para familiarizarte
2. **Composición**: Combina elementos con `~scvXenakisComposition.()`
3. **Experimentación**: Modifica parámetros y crea tus propias funciones
4. **Presentación**: Usa la composición completa para demos

## 📁 Archivos Importantes

- `src/main.rs` - Código principal del visualizador
- `supercollider_examples_fixed.scd` - ✅ Ejemplos CORREGIDOS de SuperCollider (usar este)
- `supercollider_examples.scd` - ⚠️ Archivo original (puede causar errores ProxySpace)
- `start_visualizer.sh` - Script de inicio rápido
- `verify_integration.py` - Verificación de conexión OSC
- `GUIA_USO.md` - Documentación detallada
- `README.md` - Información del proyecto

¡Disfruta creando partituras gráficas interactivas estilo Xenakis! 🎼✨
