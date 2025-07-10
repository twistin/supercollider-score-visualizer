# 🎵 SC Score Visualizer - Guía de Pruebas

## 🚀 Cómo Probar las Funcionalidades

### 1. 🎨 Cambio de Temas
**Problema reportado**: "no tiene cambio de tema"

**Cómo probar**:
- Presiona tecla **1**: Tema Oscuro (azul oscuro)
- Presiona tecla **2**: Tema Claro (gris claro)
- Presiona tecla **3**: Tema Azul (azul medio)
- Presiona tecla **4**: Tema Clásico (gris oscuro)

**También desde el menú**:
- Presiona **TAB** para mostrar menú
- Click en **Display**
- Click en **Light Theme**, **Dark Theme**, **Blue Theme**, o **Classic Theme**

**Qué debe cambiar**:
- Color de fondo de la ventana
- Colores de la grilla
- Colores del texto en estadísticas
- Colores de la barra de menú

### 2. 🔲 Grilla Visible
**Problema reportado**: "la grilla desapareció"

**Verificar que esté activada**:
- Presiona **G** para mostrar/ocultar grilla
- Presiona **L** para mostrar/ocultar etiquetas
- En consola debe aparecer: "🔲 Rejilla: ON/OFF"

**También desde el menú**:
- Presiona **TAB** para mostrar menú
- Click en **Display**
- Click en **Show Grid** (debe tener ✓)
- Click en **Show Axis Labels** (debe tener ✓)

**Qué debe verse**:
- Líneas horizontales (frecuencias) en azul
- Líneas verticales (tiempo) en azul
- Etiquetas de frecuencia a la izquierda
- Etiquetas de tiempo abajo

### 3. 📋 Funcionalidades del Menú
**Problema reportado**: "los elementos del menú no tienen funcionalidades"

**Cómo probar cada menú**:

#### File Menu:
- **New Session**: Limpia eventos y resetea UI
- **Save Session**: Guarda en session.toml
- **Export Image**: Simula exportar PNG
- **Quit**: Cierra la aplicación

#### Edit Menu:
- **Clear Events**: Limpia todos los eventos
- **Reset Settings**: Restaura configuración por defecto

#### Display Menu:
- **Fullscreen**: Activa/desactiva pantalla completa
- **Performance Mode**: Activa modo rendimiento
- **Show Timer**: Muestra reloj en estadísticas
- **Show Grid**: Muestra/oculta grilla
- **Snap to X-Grid**: Activa snap horizontal
- **Grid Resolution +/-**: Ajusta resolución
- **Temas**: Cambia colores

#### View Menu:
- **Zoom In/Out**: Cambia nivel de zoom
- **Reset Zoom**: Restaura zoom 1x
- **Show Statistics**: Muestra panel de estadísticas
- **Musical Mode**: Cambia a divisiones musicales

## 🎹 Controles de Teclado Rápidos

### Temas:
- **1**: Tema Oscuro
- **2**: Tema Claro
- **3**: Tema Azul
- **4**: Tema Clásico

### Grilla:
- **G**: Mostrar/ocultar grilla
- **L**: Mostrar/ocultar etiquetas
- **↑/↓**: Ajustar resolución

### Menú:
- **TAB**: Mostrar/ocultar menú

### Zoom:
- **+/-**: Zoom in/out
- **0**: Reset zoom

### Modos:
- **5**: Performance mode
- **6**: Timer
- **7**: Alta resolución
- **F11**: Pantalla completa

## 🔍 Verificación Visual

### Al cambiar tema debe verse:
1. **Tema Claro**: Fondo gris claro, texto oscuro
2. **Tema Oscuro**: Fondo azul oscuro, texto claro
3. **Tema Azul**: Fondo azul medio, texto muy claro
4. **Tema Clásico**: Fondo gris oscuro, texto blanco

### La grilla debe mostrar:
1. **Líneas principales**: Más brillantes y gruesas
2. **Líneas menores**: Más tenues y finas
3. **Etiquetas de frecuencia**: A la izquierda (80Hz-2000Hz)
4. **Etiquetas de tiempo**: Abajo (0s-10s)

### El menú debe responder:
1. **Click en elementos**: Abrir submenús
2. **Click en opciones**: Ejecutar acciones
3. **Indicadores ✓**: Mostrar estado activo
4. **Mensajes en consola**: Confirmar acciones

## 🐛 Solución de Problemas

### Si no ves la grilla:
1. Presiona **G** para activarla
2. Presiona **L** para activar etiquetas
3. Cambia tema con teclas **1-4**
4. Verifica en menú Display > Show Grid

### Si el tema no cambia:
1. Usa teclas **1-4** directamente
2. Usa menú Display > [Theme]
3. Observa el fondo de la ventana
4. Observa colores en estadísticas

### Si el menú no responde:
1. Presiona **TAB** para activar menú
2. Asegúrate de hacer click exacto en texto
3. Observa mensajes en terminal/consola
4. Usa **ESC** para cerrar menús

## 🚀 Ejecutar Pruebas

```bash
# Compilar y ejecutar
cargo build --release
cargo run --release

# O usar script de prueba
./test_funcionalidades.sh

# Enviar datos OSC de prueba (opcional)
./enviar_prueba_osc.sh
```

## ✅ Lista de Verificación

- [ ] Tema cambia con teclas 1-4
- [ ] Tema cambia desde menú Display
- [ ] Grilla visible con tecla G
- [ ] Etiquetas visibles con tecla L
- [ ] Menú se abre con TAB
- [ ] Submenús responden a clicks
- [ ] Acciones generan mensajes en consola
- [ ] Estadísticas muestran información
- [ ] Timer funciona con tecla 6
- [ ] Zoom funciona con +/-

Si algún elemento no funciona, revisa la consola para mensajes de debug.
