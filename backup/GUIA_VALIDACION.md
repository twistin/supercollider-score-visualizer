# 🧪 Guía de Validación - Integración Tiempo Real

## 🎯 Objetivo
Validar que la integración entre SuperCollider y el visualizador Rust funciona correctamente en tiempo real, con sincronización precisa y mapeo visual efectivo.

## 📋 Lista de Verificación

### ✅ Compilación y Setup
- [ ] El visualizador Rust compila sin errores críticos
- [ ] SuperCollider puede cargar el archivo `realtime_analysis.scd`
- [ ] Los puertos OSC están configurados correctamente (57124)

### 🔊 Análisis de Audio
- [ ] El SynthDef `RealtimeAnalyzer` se instancia correctamente
- [ ] Se detectan cambios de amplitud en tiempo real
- [ ] Se detectan onsets (inicio de sonidos)
- [ ] Se tracking de pitch funciona con tonos claros
- [ ] Los parámetros espectrales responden a cambios de timbre

### 🎨 Visualización
- [ ] El visualizador recibe mensajes OSC de `/realtime_audio`
- [ ] Los datos se parsean correctamente (11 parámetros)
- [ ] El buffer de análisis se actualiza cada frame
- [ ] Las visualizaciones responden a los cambios de audio

### 🎼 Estilos Visuales

#### Estilo Xenakis
- [ ] Los glissandi se visualizan como líneas curvas
- [ ] Las masas sonoras generan redes de partículas
- [ ] La sincronización temporal es precisa

#### Estilo Ikeda  
- [ ] Los onsets generan flashes/pulsos precisos
- [ ] Las líneas vectoriales siguen el ritmo
- [ ] El estilo minimalista es coherente

#### HUD de Análisis
- [ ] Los medidores muestran valores en tiempo real
- [ ] Los gráficos espectrales se actualizan suavemente
- [ ] La información numérica es legible

## 🚀 Métodos de Prueba

### 1. Test Automático
```bash
./test_integration.sh
```

### 2. Test Manual
1. Abrir SuperCollider IDE
2. Cargar `test_manual.scd`
3. Ejecutar bloques paso a paso
4. Iniciar visualizador: `./target/release/sc_score_visualizer`

### 3. Tests Específicos

#### Test de Glissando (Xenakis)
```supercollider
// En SuperCollider
~xenakisTest = {
    var freq = XLine.kr(200, 2000, 8);
    var sig = SinOsc.ar(freq) * Line.kr(0.8, 0, 8);
    Out.ar(0, sig ! 2);
}.play;
```
**Esperado**: Línea curva que asciende de abajo hacia arriba durante 8 segundos.

#### Test de Onsets (Ikeda)
```supercollider
~rhythmTest = {
    var trig = Impulse.kr(4);
    var sig = SinOsc.ar(440) * EnvGen.kr(Env.perc(0.01, 0.1), trig);
    Out.ar(0, sig ! 2);
}.play;
```
**Esperado**: Flashes precisos cada 0.25 segundos.

#### Test de Masa Sonora
```supercollider
~clusterTest = {
    var freqs = {exprand(200, 2000)} ! 20;
    var sig = Mix(SinOsc.ar(freqs)) * 0.1;
    Out.ar(0, sig ! 2);
}.play;
```
**Esperado**: Nube de partículas dispersas, alta planitud espectral.

## 📊 Métricas de Performance

### Latencia
- **Target**: < 50ms desde sonido hasta visualización
- **Medición**: Usar clap/palmada y cronómetro visual

### CPU Usage
- **SuperCollider**: < 10% CPU en análisis continuo
- **Rust Visualizer**: < 20% CPU con 60 FPS

### Precisión OSC
- **Frecuencia**: 50 Hz (cada 20ms)
- **Parámetros**: 11 valores por mensaje
- **Pérdida**: < 1% de mensajes

## 🐛 Troubleshooting

### SuperCollider no envía OSC
```supercollider
// Verificar configuración OSC
NetAddr.langPort;          // Puerto de SC
thisProcess.openUDPPort;   // Abrir puerto si necesario
```

### Visualizador no recibe datos
```bash
# Test de conectividad OSC
echo "Verificando puerto 57124..."
lsof -i :57124
```

### Sin detección de audio
```supercollider
// Verificar entrada de audio
Server.default.meter;      // Ver niveles
SoundIn.ar(0).poll;       // Ver valores en consola
```

## ✨ Criterios de Éxito

### Funcionalidad Básica ✅
- [x] Comunicación OSC estable
- [x] Parsing correcto de parámetros
- [x] Visualización en tiempo real

### Sincronización ⏱️
- [ ] Latencia visual < 50ms
- [ ] Sin glitches o saltos
- [ ] Coherencia temporal mantenida

### Mapeo Artístico 🎨
- [ ] Glissandi visualmente convincentes
- [ ] Onsets claramente marcados  
- [ ] Texturas tímbricas diferenciables
- [ ] Estilo visual coherente con referencias

## 📈 Próximos Pasos

1. **Optimización**: Reducir latencia y mejorar performance
2. **Estilos Adicionales**: Implementar motores de Henke y Anadol
3. **Análisis Avanzado**: MFCC, análisis armónico, detección de acordes
4. **3D Rendering**: Espacialización y profundidad visual
5. **Machine Learning**: Reconocimiento de patrones y adaptación automática
