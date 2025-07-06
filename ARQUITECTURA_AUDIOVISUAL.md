# 🎨 ARQUITECTURA DE VISUALIZACIÓN SONORA EN TIEMPO REAL

## 🎯 **VISIÓN DEL PROYECTO**

SC Score Visualizer evoluciona hacia una herramienta audiovisual generativa que representa gráficamente cualquier sonido de SuperCollider en tiempo real, inspirada en:

- **Xenakis**: Glissandi, masas sonoras, redes curvas
- **Ryoji Ikeda**: Precisión binaria, minimalismo estroboscópico  
- **Alva Noto**: Geometría digital, sincronía exacta
- **Refik Anadol**: Partículas y estructuras 3D reactivas
- **Robert Henke**: Líneas vectoriales láser, sincronía precisa

## 🧩 **COMPONENTES DEL SISTEMA**

### **1. Entrada de Audio Universal**
```
SuperCollider Audio → [Análisis en Tiempo Real] → OSC → Visualizador Rust
                                ↓
                    - Pitch tracking
                    - Amplitude analysis  
                    - Onset detection
                    - Spectral centroid
                    - Noise analysis
```

### **2. Buffer de Eventos Temporales**
```rust
struct AudioEvent {
    start_time: f64,
    duration: Option<f64>,
    event_type: AudioEventType,
    parameters: AudioParameters,
}

enum AudioEventType {
    Onset,
    Note { pitch: f32, confidence: f32 },
    Glissando { start_pitch: f32, end_pitch: f32 },
    Cluster { center_freq: f32, spread: f32 },
    Noise { spectral_centroid: f32, bandwidth: f32 },
    SoundMass { components: Vec<SpectralComponent> },
}
```

### **3. Mapeo Visual Genérico**

| Parámetro Sonoro | Variable Visual |
|------------------|-----------------|
| **Tiempo** | Posición horizontal (X) |
| **Pitch** | Posición vertical (Y) |
| **Amplitud** | Grosor, escala, intensidad |
| **Timbre** | Forma, dispersión, textura |
| **Onsets** | Partículas, flashes |
| **Ruido** | Granularidad, turbulencia |

### **4. Motor de Sincronización Visual**
- Cada frame: `app.time` vs `event.start_time`
- Renderizado progresivo: eventos se "escriben" al sonar
- Buffer temporal: eventos activos en ventana de tiempo

## 🎨 **ESTILOS VISUALES IMPLEMENTADOS**

### **A. Modo Xenakis**
- Glissandi como curvas suaves
- Masas sonoras como clusters de partículas
- Redes de líneas interconectadas

### **B. Modo Ikeda**
- Elementos binarios precisos
- Flashes estroboscópicos en onsets
- Minimalismo geométrico

### **C. Modo Henke**
- Líneas vectoriales láser
- Sincronía frame-perfect
- Estética de performance en vivo

### **D. Modo Anadol**
- Sistemas de partículas 3D
- Estructuras generativas
- Respuesta adaptativa al análisis

---

## 🔧 **IMPLEMENTACIÓN TÉCNICA**

### **Fase 1: Análisis en SuperCollider**
- SynthDef de análisis en tiempo real
- Detección de pitch, amplitude, onsets
- Envío OSC optimizado (50 Hz)

### **Fase 2: Motor de Eventos en Rust** 
- Buffer circular de eventos audio
- Sistema de sincronización temporal
- Mapeo paramétrico configurable

### **Fase 3: Motores de Renderizado**
- Engine Xenakis: curvas y masas
- Engine Ikeda: precisión digital
- Engine Henke: vectores láser
- Engine Anadol: partículas 3D

### **Fase 4: Sincronización Ultra-Precisa**
- Compensación de latencia
- Buffer predictivo
- Interpolación temporal

---

## 🎼 **ARCHIVO DE ANÁLISIS SUPERCOLLIDER**

```supercollider
// realtime_analysis.scd - Motor de análisis universal ✅ CORREGIDO
SynthDef(\RealtimeAnalyzer, {
    var in, amp, pitch, hasFreq, onset, centroid, flux, fft;
    
    in = SoundIn.ar(0);
    
    // Análisis multi-paramétrico
    amp = Amplitude.kr(in, 0.01, 0.1);
    # pitch, hasFreq = Pitch.kr(in, ampThreshold: 0.02);
    
    // Análisis de onsets corregido
    onset = Onsets.kr(FFT(LocalBuf(1024), in), \rcomplex, 0.3, 1.0, 0.1, 10);
    
    // Análisis espectral
    fft = FFT(LocalBuf(2048), in);
    centroid = SpecCentroid.kr(fft);
    flux = LPZ1.kr(centroid).abs;  // Flujo espectral aproximado
    
    // Stream OSC ultra-rápido para sync preciso (50 Hz)
    SendReply.kr(Impulse.kr(50), "/realtime_audio", [
        pitch, amp, onset, hasFreq, centroid, flux,
        SpecPcile.kr(fft, 0.85), SpecFlatness.kr(fft),
        hasFreq * (1 - SpecFlatness.kr(fft)),  // harmonicity
        SpecFlatness.kr(fft) * (1 - hasFreq),  // noisiness
        (centroid - 1000) / 1000               // spectral slope
    ]);
}).add;
```

---

## 🚀 **HOJA DE RUTA DE IMPLEMENTACIÓN**

### **🎯 Milestone 1: Análisis de Audio**
- [x] SynthDef de análisis en tiempo real
- [x] Detección de pitch/amplitude/onsets
- [x] Calibración de latencia

### **🎯 Milestone 2: Motor de Eventos**
- [x] Buffer circular de AudioEvent
- [x] Sistema de timestamp preciso
- [x] Mapeo paramétrico básico

### **🎯 Milestone 3: Primer Motor Visual (Xenakis)**
- [x] Renderizado de glissandi en tiempo real
- [x] Masas sonoras como clusters
- [x] Curvas paramétricas

### **🎯 Milestone 4: Motores Adicionales**
- [x] Motor Ikeda (precisión digital)
- [ ] Motor Henke (vectores láser)
- [ ] Motor Anadol (partículas 3D)

### **🎯 Milestone 5: Optimización Final**
- [x] Sincronización frame-perfect
- [x] Performance en tiempo real
- [ ] Configuración por estilos

---

**🎵 OBJETIVO: Herramienta audiovisual universal que funciona automáticamente con cualquier pieza de SuperCollider, generando visualizaciones sincronizadas de nivel profesional.**
