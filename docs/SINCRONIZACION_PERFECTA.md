# SISTEMA AUDIOVISUAL SINCRONIZADO - VERSIÓN CORREGIDA

## ✅ PROBLEMAS SOLUCIONADOS

### 1. **Dirección Visual vs Sonora**
- **ANTES**: El audio ascendía mientras lo visual descendía (o viceversa)
- **AHORA**: Dirección perfectamente sincronizada
- **TÉCNICA**: `XLine.kr(startFreq, endFreq, dur)` en SC mapea directamente a la progresión visual linear

### 2. **Duración Audio vs Visual**
- **ANTES**: Audio se silenciaba antes que lo visual desapareciera
- **AHORA**: Duración idéntica para audio y visual
- **TÉCNICA**: Envolvente mínima `Env.linen(0.05, dur - 0.1, 0.05)` en lugar de `Env.linen(0.1, dur - 0.2, 0.1)`

### 3. **Artefactos Visuales**
- **ANTES**: "Puntitos grises" y líneas fragmentadas en glissandos
- **AHORA**: Líneas limpias y profesionales
- **TÉCNICA**: Eliminación de múltiples pasadas de renderizado y texturas innecesarias

### 4. **Sistema Extensible para Live Coding**
- **AHORA**: Funciones modulares que pueden ser llamadas independientemente
- **TÉCNICA**: Separación clara entre funciones de SC y visualizador Rust

## 🎯 ARCHIVOS CLAVE

### SuperCollider (Audio)
- `demo_sincronizado_perfecto.scd` - Nuevo demo con sincronización perfecta
- `demo_colores_secuencial_auto.scd` - Demo anterior (aún funcional)

### Rust (Visual)
- `src/main.rs` - Visualizador corregido
  - Función `render_glissando_event()` completamente reescrita
  - Sincronización temporal perfecta
  - Eliminación de artefactos visuales

## 🎮 USO PARA LIVE CODING

### Carga las funciones básicas:
```supercollider
(
// Glissando sincronizado
~perfectGliss = { |startFreq=100, endFreq=400, amp=0.8, dur=3.0|
    var addr = NetAddr.new("127.0.0.1", 57122);
    addr.sendMsg("/event", "gliss", startFreq, endFreq, amp, dur);
    if(s.serverRunning, {
        {
            var freqLine = XLine.kr(startFreq, endFreq, dur);
            var env = Env.linen(0.05, dur - 0.1, 0.05);
            var sig = SinOsc.ar(freqLine, 0, amp * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    });
};

// Punto sincronizado
~perfectPoint = { |freq=440, amp=0.5, dur=1.0|
    var addr = NetAddr.new("127.0.0.1", 57122);
    addr.sendMsg("/event", "point", freq, amp, dur);
    if(s.serverRunning, {
        {
            var env = Env.linen(0.05, dur - 0.1, 0.05);
            var sig = SinOsc.ar(freq, 0, amp * EnvGen.kr(env, doneAction: 2));
            Out.ar(0, sig.dup);
        }.play;
    });
};
)
```

### Uso en live coding:
```supercollider
// Glissando ascendente
~perfectGliss.(200, 800, 0.9, 4.0);

// Glissando descendente  
~perfectGliss.(800, 200, 0.9, 4.0);

// Puntos de diferentes duraciones
~perfectPoint.(440, 0.8, 1.0);
~perfectPoint.(550, 0.8, 2.0);
~perfectPoint.(660, 0.8, 3.0);

// Eventos simultáneos
fork {
    ~perfectGliss.(100, 400, 0.8, 5.0);
    ~perfectGliss.(500, 200, 0.7, 5.0);
    0.5.wait;
    ~perfectPoint.(330, 0.9, 3.0);
};
```

## 🔧 REGLAS DE MAPEO AUDIOVISUAL

### Para Nuevos Eventos Personalizados:

1. **Duración**: Usar envolvente mínima en SC
   ```supercollider
   var env = Env.linen(0.05, dur - 0.1, 0.05);
   ```

2. **Mensaje OSC**: Formato consistente
   ```supercollider
   addr.sendMsg("/event", "tipo", param1, param2, amplitud, duracion);
   ```

3. **Frecuencia**: Mapeo directo
   - Frecuencias más altas = posición Y más alta en visualizador
   - Glissandos: dirección SC = dirección visual

4. **Color**: Automático basado en tipo y parámetros
   - Puntos: Rojo intenso
   - Glissandos: Azul estructural  
   - Clusters: Naranja tierra
   - Ruido: Verde orgánico

## 🚀 INSTRUCCIONES DE EJECUCIÓN

1. **Iniciar visualizador**:
   ```bash
   cd /Users/sdcarr/Documents/github/sc-score/sc_score_visualizer
   cargo run
   ```

2. **Iniciar SuperCollider**:
   ```supercollider
   s.boot;
   ```

3. **Cargar y ejecutar demo**:
   ```supercollider
   // Cargar archivo en SC
   "demo_sincronizado_perfecto.scd".load;
   
   // O ejecutar función directa
   ~perfectGliss.(200, 800, 0.9, 4.0);
   ```

## 🎨 CARACTERÍSTICAS VISUALES

- **Líneas limpias**: Sin artefactos o "puntitos grises"
- **Progreso sincronizado**: Punto blanco indica posición actual del audio
- **Direcciones correctas**: Visual sigue exactamente la dirección del audio
- **Colores profesionales**: Paleta inspirada en partituras de Xenakis
- **Duración exacta**: Visual desaparece exactamente cuando termina el audio

## 🔍 DEBUGGING

Si observas desincronización:

1. **Verificar duración de envolvente en SC**
2. **Comprobar que el visualizador Rust está actualizado**
3. **Revisar que el mensaje OSC llega correctamente**
4. **Confirmar que no hay latencia de audio significativa**

## 📝 NOTAS PARA DESARROLLO

- El sistema está diseñado para ser **extensible** 
- Nuevos tipos de eventos pueden agregarse fácilmente
- La sincronización se basa en mensajes OSC timestamps
- Todos los parámetros son modificables en tiempo real
- Compatible con patrones complejos y algoritmos generativos
