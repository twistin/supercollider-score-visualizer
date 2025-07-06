# ✅ Checklist de Primer Uso - SC Score Visualizer

## 🎯 Lista de Verificación Rápida

### 📋 Preparación (5 minutos)
- [ ] **Rust instalado**: `cargo --version` funciona
- [ ] **Python 3 disponible**: `python3 --version` funciona  
- [ ] **SuperCollider instalado**: App en /Applications/ o ejecutable disponible
- [ ] **Proyecto compilado**: `cargo build --release` sin errores críticos

### 🚀 Primera Ejecución (2 minutos)
- [ ] **Visualizador arranca**: `./target/release/sc_score_visualizer` 
- [ ] **Ventana negra aparece** con título "SC Score Visualizer"
- [ ] **Panel azul visible** en esquina superior derecha
- [ ] **Información del sistema** mostrada en el panel
- [ ] **Puerto OSC activo**: "📡 Receptor OSC activo en puerto 57124"

### 🧪 Test de Comunicación (3 minutos)
- [ ] **Test de glissando**: `python3 test_glissando.py`
  - [ ] Línea curva se dibuja de abajo hacia arriba
  - [ ] Duración aproximada: 4 segundos
  - [ ] Contador de eventos se incrementa
- [ ] **Test de onsets**: `python3 test_rhythm.py`
  - [ ] 8 flashes/pulsos visibles
  - [ ] Sincronización cada ~0.5 segundos
  - [ ] Diferentes intensidades/colores

### 🎼 Integración SuperCollider (5 minutos)
- [ ] **SuperCollider abierto**: App funciona correctamente
- [ ] **Archivo cargado**: `realtime_analysis.scd` se abre sin errores
- [ ] **Servidor iniciado**: `Server.default.boot` exitoso
- [ ] **Analizador activo**: `~analyzer = Synth(\RealtimeAnalyzer)` funciona
- [ ] **Datos enviándose**: Visualizador recibe mensajes OSC

### 🎨 Visualización en Tiempo Real (3 minutos)
- [ ] **Audio externo detectado**: Micrófono o audio del sistema
- [ ] **Glissandi funcionan**: Sonidos continuos generan líneas curvas
- [ ] **Onsets funcionan**: Sonidos percusivos generan flashes
- [ ] **Panel actualizado**: Información cambia en tiempo real
- [ ] **Sincronización precisa**: Visual coherente con audio

---

## 🎯 Criterios de Éxito

### ✅ BÁSICO (Mínimo Funcional)
- Visualizador arranca sin errores
- Tests de Python funcionan
- Comunicación OSC establecida
- Visualizaciones aparecen

### ✅ COMPLETO (Totalmente Operativo) 
- SuperCollider conecta correctamente
- Audio real genera visualizaciones
- Sincronización temporal precisa
- Todos los estilos visuales activos

### ✅ AVANZADO (Listo para Producción)
- Latencia < 50ms perceptible
- Sin glitches o interrupciones
- Performance estable durante minutos
- Capacidad de procesamientos complejos

---

## 🔧 Solución Rápida de Problemas

### ❌ "Visualizador no arranca"
```bash
# Verificar compilación
cargo build --release

# Verificar permisos
chmod +x target/release/sc_score_visualizer

# Verificar dependencias del sistema
./target/release/sc_score_visualizer --help
```

### ❌ "No se ven visualizaciones en tests Python"
```bash
# Verificar puerto OSC
lsof -i :57124

# Reiniciar visualizador
pkill sc_score_visualizer
./target/release/sc_score_visualizer &

# Test simple
python3 -c "
import socket
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.sendto(b'test', ('127.0.0.1', 57124))
print('Mensaje enviado')
"
```

### ❌ "SuperCollider no conecta"
```supercollider
// Verificar configuración
NetAddr.langPort;
thisProcess.openUDPPort;

// Test directo
~test = NetAddr("127.0.0.1", 57124);
~test.sendMsg("/realtime_audio", 440, 0.5, 1, 1, 1000, 0.3, 2000, 0.2, 0.8, 0.2, 0);
```

### ❌ "No hay audio"
```supercollider
// Verificar servidor de audio
Server.default.meter;       // Ver niveles de entrada
s.scope;                    // Ver forma de onda
Server.default.options.inDevice;   // Verificar dispositivo de entrada
```

---

## 🎵 Uso Inmediato

Una vez completado el checklist, puedes usar SC Score Visualizer para:

### 🎭 Performance en Vivo
1. Conectar micrófono o instrumento
2. Ejecutar visualizador en pantalla/proyector
3. Tocar música → Ver visualizaciones automáticas

### 🎹 Composición Asistida
1. Ejecutar SuperCollider + visualizador
2. Escribir código musical
3. Ver feedback visual inmediato

### 🎨 Instalación Interactiva
1. Setup automático con audio ambiente
2. Respuesta visual a sonidos del entorno
3. Experiencia audiovisual continua

---

## 📊 Tiempo Estimado Total: ~15-20 minutos

- **Preparación**: 5 min
- **Tests básicos**: 5 min  
- **Integración SC**: 5 min
- **Validación completa**: 5 min

**✅ Al completar este checklist, tendrás un sistema de visualización audiovisual completamente funcional y listo para uso creativo profesional.**
