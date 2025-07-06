# 🎨 SC Score Visualizer

**Herramienta audiovisual en tiempo real para SuperCollider**

Visualizador generativo que transforma cualquier audio de SuperCollider en representaciones gráficas sincronizadas, inspirado en los estilos de Xenakis, Ryoji Ikeda, Alva Noto, Refik Anadol y Robert Henke.

![SC Score Visualizer Demo](https://img.shields.io/badge/status-active-brightgreen) ![Rust](https://img.shields.io/badge/rust-1.70+-orange) ![SuperCollider](https://img.shields.io/badge/supercollider-3.11+-blue)

---

## ✨ **Características**

- 🎵 **Análisis en tiempo real**: Pitch, amplitud, onsets, centroide espectral
- 🎨 **Visualización sincronizada**: Frame-perfect con el audio
- 🚀 **Zero latencia**: Motor optimizado en Rust + wgpu
- 🎭 **Múltiples estilos**: Xenakis, Ikeda, Henke, Anadol
- 🔧 **Plug & Play**: Funciona con cualquier pieza de SuperCollider
- ⚡ **Ultra rápido**: 60 FPS con miles de eventos simultáneos

---

## 🚀 **Inicio Rápido**

### **Opción 1: Script Automático** 
```bash
./start.sh
```

### **Opción 2: Manual**

**Terminal 1: Iniciar Visualizador**
```bash
cargo run --release
```

**Terminal 2: Iniciar Análisis en SuperCollider**
```supercollider
// En SuperCollider IDE:
"realtime_analysis.scd".loadDocument.front.execute;
```

**Terminal 3: ¡Crear Sonidos!**
```supercollider
// Cualquier sonido será visualizado automáticamente
{ SinOsc.ar([440, 441], 0, 0.1) }.play;

// Glissandos épicos
{ SinOsc.ar(XLine.kr(200, 2000, 5), 0, 0.3) }.play;

// Clusters complejos
{ Mix.ar(SinOsc.ar({200 + 500.rand}!12, 0, 0.08)) }.play;
```

---

## 🎨 **Estilos Visuales**

### **🌊 Xenakis Mode**
```supercollider
// Glissandi como curvas fluidas
{ SinOsc.ar(XLine.kr(100, 1000, 8), 0, 0.2) }.play;
```
- Masas sonoras como clusters de partículas
- Redes de líneas interconectadas  
- Curvas paramétricas suaves

### **⚡ Ikeda Mode**
```supercollider
// Elementos binarios precisos
{ Impulse.ar([8, 13], 0, 0.1) }.play;
```
- Flashes estroboscópicos en onsets
- Minimalismo geométrico extremo
- Precisión matemática

### **🔆 Henke Mode**
```supercollider
// Líneas vectoriales tipo láser
{ LFSaw.ar([220, 440], 0, 0.15) }.play;
```
- Sincronización frame-perfect
- Estética de performance live
- Vectores de alta precisión

### **🌌 Anadol Mode**
```supercollider
// Sistemas de partículas complejas
{ Dust.ar([100, 200, 300], 0.1) }.play;
```
- Estructuras generativas 3D
- Respuesta adaptativa al timbre
- Morfología de datos sonoros

---

## 📦 **Instalación**

### **Prerequisitos**
- **Rust 1.70+**: [Instalar aquí](https://rustup.rs/)
- **SuperCollider 3.11+**: [Descargar aquí](https://supercollider.github.io/)
- **Git**: Para clonar el repositorio

### **Instalación Completa**
```bash
# 1. Clonar repositorio
git clone https://github.com/tu-usuario/sc-score-visualizer.git
cd sc-score-visualizer

# 2. Compilar optimizado
cargo build --release

# 3. Probar instalación
cargo test

# 4. Ejecutar
./start.sh
```

---

## 🔧 **Configuración Avanzada**

### **Cambiar Estilo Visual**
```toml
# config.toml
[visual]
style = "anadol"  # xenakis, ikeda, henke, anadol
```

### **Optimizar Rendimiento**
```toml
# config.toml
[performance]
max_events = 200        # Más eventos = más detalle
refresh_rate = 120.0    # Para monitores 120Hz
interpolation = true    # Suavizado extra
```

### **Personalizar OSC**
```toml
# config.toml
[audio]
osc_port = 57124
osc_host = "127.0.0.1"
```

---

## 🎼 **Ejemplos Musicales**

### **Ejemplo 1: Glissando Xenakis**
```supercollider
(
{
    var freq = XLine.kr(50, 2000, 15);
    var amp = XLine.kr(0.3, 0.01, 15);
    SinOsc.ar(freq, 0, amp);
}.play;
)
```

### **Ejemplo 2: Cluster Textural**
```supercollider
(
{
    var freqs = {200 + 800.rand}!20;
    var amps = {0.01 + 0.05.rand}!20;
    Mix.ar(SinOsc.ar(freqs, 0, amps));
}.play;
)
```

### **Ejemplo 3: Patrón Rítmico**
```supercollider
(
Pbind(
    \freq, Pseq([220, 330, 440, 550], inf),
    \dur, Pseq([0.25, 0.5, 0.25, 1], inf),
    \amp, 0.2
).play;
)
```

---

## 🛠 **Solución de Problemas**

### **❌ No se ve la ventana**
```bash
# En macOS: Buscar con Cmd+Tab
# La ventana puede estar minimizada o fuera de pantalla

# Verificar que el proceso está corriendo:
ps aux | grep sc_score
```

### **❌ No llegan datos OSC**
```supercollider
// En SuperCollider, verificar envío:
OSCFunc.trace(true);

// Debe mostrar mensajes como:
// OSC Message Received: [/realtime_audio, 440.0, 0.5, ...]
```

### **❌ Audio no detectado**
```supercollider
// Verificar entrada de audio
Server.local.options.numInputBusChannels = 2;
Server.local.reboot;

// Probar entrada:
{ SoundIn.ar(0) }.play;  // Debe escucharse el micrófono
```

### **❌ Baja performance**
```toml
# Reducir carga en config.toml:
[visual]
max_events = 50
refresh_rate = 30.0

[performance]
interpolation = false
```

---

## 📁 **Estructura del Proyecto**

```
sc_score_visualizer/
├── 📄 README.md                    # Este archivo
├── 📄 realtime_analysis.scd        # Motor de análisis SuperCollider
├── 📄 config.toml                  # Configuración principal
├── 📄 ARQUITECTURA_AUDIOVISUAL.md  # Documentación técnica detallada
├── 🚀 start.sh                     # Script de inicio rápido
├── 📁 src/                         # Código fuente Rust
│   ├── main.rs                     # Punto de entrada
│   ├── audio/                      # Procesamiento audio/OSC
│   ├── visual/                     # Motores de renderizado
│   └── config/                     # Gestión de configuración
├── 📁 tests/                       # Tests de integración
├── 📁 scripts/                     # Scripts de utilidad
├── 📁 supercollider/              # Archivos .scd adicionales
└── 📁 backup/                      # Archivos de desarrollo
```

---

## 🎯 **Casos de Uso**

### **🎭 Performance en Vivo**
```supercollider
// Tu set + visualización automática
~myLiveSet = {
    // Tu música aquí
};
~myLiveSet.play; // ← Se visualiza automáticamente
```

### **🎬 Instalación Audiovisual**
- Conectar a sistema de sonido multicanal
- Proyección en gran formato
- Control MIDI/OSC externo

### **🎓 Educación Musical**
- Visualizar conceptos teóricos
- Análisis de espectros en tiempo real
- Composición asistida visualmente

### **🔬 Investigación Sonora**
- Análisis visual de texturas
- Estudio de morfología espectral
- Documentación de procesos compositivos

---

## 🤝 **Contribuir**

1. **Fork** el proyecto
2. **Crear branch** para feature (`git checkout -b feature/nueva-funcionalidad`)
3. **Commit** cambios (`git commit -am 'Agregar nueva funcionalidad'`)
4. **Push** al branch (`git push origin feature/nueva-funcionalidad`)
5. **Crear Pull Request**

### **Ideas para Contribuir**
- 🎨 Nuevos motores visuales
- 🎵 Algoritmos de análisis avanzados
- 🔧 Optimizaciones de performance
- 📚 Documentación y ejemplos
- 🧪 Tests adicionales

---

## 📚 **Documentación Adicional**

- **[ARQUITECTURA_AUDIOVISUAL.md](ARQUITECTURA_AUDIOVISUAL.md)**: Especificación técnica completa
- **[tests/](tests/)**: Casos de prueba y ejemplos avanzados
- **[scripts/](scripts/)**: Herramientas de desarrollo y debug

---

## 📄 **Licencia**

```
MIT License

Copyright (c) 2025 SC Score Visualizer

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 🎵 **Créditos e Inspiración**

### **Artistas Visuales Inspiradores**
- **🎼 Iannis Xenakis** - Pionero de la música visual y estocástica
- **⚡ Ryoji Ikeda** - Maestro del minimalismo digital ultra-preciso
- **🔲 Alva Noto** - Geometría sonora y sincronía perfecta
- **🌊 Refik Anadol** - Visualización de datos generativa y AI
- **🔆 Robert Henke** - Performance audiovisual live y tecnología láser

### **Tecnologías Utilizadas**
- **🦀 Rust** - Lenguaje de sistemas ultra-rápido
- **🎮 wgpu** - API gráfica moderna y cross-platform
- **🎵 SuperCollider** - Plataforma de síntesis y análisis de audio
- **📡 OSC (Open Sound Control)** - Protocolo de comunicación musical

### **Agradecimientos Especiales**
- Comunidad SuperCollider por la plataforma increíble
- Comunidad Rust por las herramientas excepcionales
- Artistas pioneros del arte audiovisual generativo

---

## 🔗 **Enlaces**

- **📞 Reportar Bugs**: [Issues](../../issues)
- **💡 Proponer Ideas**: [Discussions](../../discussions)
- **📖 Wiki**: [Documentación Completa](../../wiki)
- **🐦 Updates**: [@sc_visualizer](https://twitter.com/sc_visualizer)

---

<div align="center">

**Construido con ❤️ usando Rust + SuperCollider**

*"Where sound becomes light, and time becomes space"*

[![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust)](https://rust-lang.org/)
[![SuperCollider](https://img.shields.io/badge/SuperCollider-Compatible-blue?style=for-the-badge)](https://supercollider.github.io/)

</div>
