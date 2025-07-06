# 🚀 Guía de Primer Uso - SC Score Visualizer

## 📋 Requisitos Previos

### Sistema
- **macOS** (funciona también en Linux/Windows con ajustes menores)
- **Rust** instalado (https://rustup.rs/)
- **SuperCollider** instalado (https://supercollider.github.io/)
- **Python 3** para tests opcionales

### Verificar Instalaciones
```bash
# Verificar Rust
cargo --version

# Verificar SuperCollider (debería estar en /Applications/ en macOS)
ls /Applications/SuperCollider*

# Verificar Python
python3 --version
```

---

## 🎯 PASO 1: Preparación del Proyecto

### 1.1 Compilar el Visualizador
```bash
cd /Users/tu_usuario/Documents/github/sc-score/sc_score_visualizer
cargo build --release
```
**Resultado esperado**: Compilación exitosa (pueden aparecer warnings, son normales)

### 1.2 Verificar Archivos Clave
```bash
ls -la
```
**Archivos importantes**:
- `src/main.rs` - Código principal del visualizador
- `realtime_analysis.scd` - Analizador de audio SuperCollider
- `config.toml` - Configuración OSC
- `target/release/sc_score_visualizer` - Ejecutable compilado

---

## 🎵 PASO 2: Primera Ejecución (Test Básico)

### 2.1 Iniciar el Visualizador
```bash
./target/release/sc_score_visualizer
```

**Resultado esperado**:
```
✅ Configuración 'config.toml' cargada exitosamente.
📡 Receptor OSC activo en puerto 57124
```

### 2.2 Verificar la Ventana Visual
- Se abrirá una ventana negra con título "SC Score Visualizer"
- En la esquina superior derecha verás un panel azul con información del sistema
- La ventana responde a eventos del mouse y teclado

---

## 🧪 PASO 3: Test de Comunicación OSC

### 3.1 Test con Python (Más Simple)
En otra terminal, ejecuta:
```bash
python3 test_glissando.py
```

**Resultado esperado**:
```
🎼 Iniciando glissando 220Hz -> 880Hz durante 4 segundos...
   Observa el visualizador para ver la línea curva aparecer!
  5% - 237Hz
  10% - 254Hz
  ...
✅ Glissando completado!
```

**En el visualizador deberías ver**:
- Una línea curva que se dibuja en tiempo real de abajo hacia arriba
- El panel de información actualizándose
- Mensajes OSC en la terminal del visualizador

### 3.2 Test de Onsets Rítmicos
```bash
python3 test_rhythm.py
```

**Resultado esperado**:
- 8 flashes precisos cada 0.5 segundos
- Diferentes colores/intensidades por onset
- Contador de eventos incrementándose

---

## 🎼 PASO 4: Integración con SuperCollider Real

### 4.1 Abrir SuperCollider IDE
1. Abre **SuperCollider.app**
2. Abre el archivo `realtime_analysis.scd`

### 4.2 Cargar el Analizador
En SuperCollider, ejecuta por bloques:

```supercollider
// 1. Cargar definiciones (Cmd+Return en este bloque)
(
"🎵 Inicializando Motor de Análisis de Audio en Tiempo Real...".postln;
// ... [todo el bloque del SynthDef]
)

// 2. Iniciar servidor (Cmd+Return)
Server.default.boot;

// 3. Iniciar analizador (Cmd+Return)
(
"🚀 Iniciando análisis en tiempo real...".postln;
~analyzer = Synth(\RealtimeAnalyzer);
"📡 Enviando datos OSC a puerto 57124...".postln;
)
```

### 4.3 Generar Sonidos de Prueba

**Test de Glissando**:
```supercollider
(
"🎼 Test con glissando estilo Xenakis...".postln;
~xenakisTest = {
    var freq = XLine.kr(200, 2000, 8);
    var amp = Line.kr(0.1, 0.8, 8) * Line.kr(1, 0, 8);
    var sig = SinOsc.ar(freq) * amp;
    Out.ar(0, sig ! 2);
}.play;
)
```

**Test de Onsets**:
```supercollider
(
"🥁 Test rítmico (onsets marcados)...".postln;
~rhythmTest = {
    var trig = Impulse.kr([2, 3, 5, 7].choose);
    var freq = TChoose.kr(trig, [220, 330, 440, 550]);
    var sig = SinOsc.ar(freq) * EnvGen.kr(Env.perc(0.01, 0.2), trig) * 0.3;
    Out.ar(0, sig ! 2);
}.play;
)
```

---

## 👀 PASO 5: Qué Esperar Ver

### 5.1 Estilo Xenakis (Glissandi)
- **Líneas curvas** que se dibujan progresivamente
- **Trayectorias ascendentes/descendentes** siguiendo el pitch
- **Grosor variable** según la amplitud
- **Colores cálidos** (rojizos, naranjas)

### 5.2 Estilo Ikeda (Onsets)
- **Flashes precisos** en momentos de onset
- **Líneas vectoriales** minimalistas
- **Sincronización exacta** con el audio
- **Colores fríos** (azules, blancos)

### 5.3 Panel de Información (HUD)
```
┌─────────────────────────────────────┐
│          🎵 SC Score Visualizer     │
│           📊 Eventos: 42            │
│           ⏱️  Tiempo: 8.3s           │
│           📡 OSC: 57124             │
└─────────────────────────────────────┘
```

---

## 🔧 PASO 6: Solución de Problemas Comunes

### 6.1 El Visualizador No Recibe Datos
```bash
# Verificar que el puerto 57124 está libre
lsof -i :57124

# Si hay otro proceso usando el puerto
sudo kill -9 [PID_DEL_PROCESO]
```

### 6.2 SuperCollider No Conecta
```supercollider
// Verificar configuración OSC en SuperCollider
NetAddr.langPort;          // Puerto de SC
thisProcess.openUDPPort;   // Abrir puerto si necesario

// Test directo
~visualizer = NetAddr("127.0.0.1", 57124);
~visualizer.sendMsg("/realtime_audio", 440, 0.5, 1.0, 1.0, 1000, 0.3, 2000, 0.2, 0.8, 0.2, -0.5);
```

### 6.3 Sin Audio en SuperCollider
```supercollider
// Verificar servidor de audio
Server.default.meter;      // Ver niveles
s.scope;                   // Ver forma de onda
Server.default.options.outDevice;  // Verificar dispositivo
```

---

## 🎨 PASO 7: Experimentación Creativa

### 7.1 Composiciones Generativas
```supercollider
// Masa sonora compleja
(
~massTest = {
    var freqs = {exprand(200, 2000)} ! 30;
    var amps = {LFNoise1.kr(exprand(0.1, 2)).max(0)} ! 30;
    var sig = Mix(SinOsc.ar(freqs) * amps) * 0.05;
    Out.ar(0, sig ! 2);
}.play;
)

// Secuencias algorítmicas
(
Pbind(
    \freq, Pwhite(200, 800, inf),
    \dur, Pwhite(0.1, 0.5, inf),
    \amp, 0.3
).play;
)
```

### 7.2 Audio Externo
- **Conectar micrófono**: Cambia `inputBus = 0` en el SynthDef
- **Audio de aplicaciones**: Usa Loopback o SoundFlower
- **Instrumentos reales**: Conecta interface de audio

---

## 🎯 PASO 8: Uso en Performance

### 8.1 Setup en Vivo
1. **Ejecutar visualizador** en pantalla principal/proyector
2. **SuperCollider en laptop** para control
3. **Audio interface** para entrada de instrumentos
4. **Script preparado** con SynthDefs y análisis

### 8.2 Script de Performance
```supercollider
// Script listo para performance
(
// Cargar analizador
load("realtime_analysis.scd");

// Auto-boot servidor
Server.default.waitForBoot {
    1.wait;
    ~analyzer = Synth(\RealtimeAnalyzer);
    "🎵 Sistema listo para performance!".postln;
};

Server.default.boot;
)
```

---

## ✅ CHECKLIST DE PRIMER USO

- [ ] Rust instalado y funcionando
- [ ] SuperCollider instalado y funcionando
- [ ] Proyecto compilado sin errores críticos
- [ ] Visualizador arranca y muestra puerto OSC
- [ ] Test de Python funciona (línea curva visible)
- [ ] SuperCollider conecta y envía datos
- [ ] Panel de información se actualiza
- [ ] Audio real genera visualizaciones
- [ ] Estilos Xenakis e Ikeda funcionan
- [ ] Sistema listo para uso creativo

---

## 🎵 ¡YA ESTÁS LISTO!

Una vez completados estos pasos, tienes un sistema completamente funcional de visualización audiovisual en tiempo real. Puedes:

- **Improvisar** con instrumentos y ver visualizaciones automáticas
- **Componer** con feedback visual inmediato
- **Presentar** en vivo con sincronización perfecta
- **Instalar** en espacios para experiencias interactivas

**¡Disfruta creando arte audiovisual con SC Score Visualizer!** 🎨🎵✨
