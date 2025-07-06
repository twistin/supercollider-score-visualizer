# SOLUCIÓN DEFINITIVA: Error "Variable 'scvTestBasicEvents' not defined"

## 🎯 SOLUCIÓN INMEDIATA

El archivo `supercollider_examples.scd` ha sido **actualizado y corregido** para resolver este error.

### Pasos exactos a seguir:

1. **Abre SuperCollider**

2. **Carga el archivo completo:**
   ```supercollider
   "supercollider_examples.scd".loadRelative;
   ```
   
   O alternativamente, **ejecuta todo el bloque manualmente:**
   - Abre el archivo `supercollider_examples.scd`
   - Selecciona **TODO** el código entre `(` y `)`
   - Ejecuta con `Cmd+Enter` (Mac) o `Ctrl+Enter` (Windows/Linux)

3. **Espera estos mensajes en el Post Window:**
   ```
   === Iniciando carga de SC Score Visualizer ===
   Configurando conexión OSC...
   Definiendo funciones de envío...
   Definiendo funciones de composición...
   Exportando funciones a variables globales...
   Verificando funciones exportadas...
   ✓ scvTestBasicEvents exportada correctamente
   ✓ scvSendPoint exportada correctamente
   ✓ scvSendGliss exportada correctamente
   ✓ scvSendCluster exportada correctamente
   ✓ scvSendNoise exportada correctamente
   ✓ scvSendSoundMass exportada correctamente
   
   ✓ ✓ ✓ TODAS LAS FUNCIONES CARGADAS EXITOSAMENTE ✓ ✓ ✓
   
   🎉 LISTO PARA USAR - Ejecuta: ~scvTestBasicEvents.() 🎉
   ```

4. **AHORA puedes usar las funciones:**
   ```supercollider
   ~scvTestBasicEvents.();
   ```

## 🔧 ¿Por qué ocurría el error?

El error se debía a que:
1. El bloque principal no se ejecutaba completamente
2. No había verificación de que las variables globales se exportaran correctamente
3. Faltaban mensajes informativos para guiar al usuario

## 🛡️ Mejoras implementadas

El archivo actualizado ahora incluye:

- ✅ **Instrucciones claras** al inicio del archivo
- ✅ **Mensajes informativos** durante la carga
- ✅ **Verificación automática** de exportación de funciones
- ✅ **Confirmación visual** de que todo está listo
- ✅ **Manejo de errores** si algo falla

## 🚨 Si aún tienes problemas

### Opción 1: Usar la versión robusta alternativa
```supercollider
"supercollider_examples_robust.scd".loadRelative;
```

### Opción 2: Carga manual mínima
```supercollider
(
var visualizer = NetAddr.new("127.0.0.1", 57120);
var testFunc = {
    "Enviando evento de prueba...".postln;
    visualizer.sendMsg("/event", "point", 440, 0.5, 1.0, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
    "Evento enviado".postln;
};
~scvTestBasicEvents = testFunc;
"✓ Función básica cargada. Ejecuta: ~scvTestBasicEvents.()".postln;
)
```

### Opción 3: Diagnóstico completo
```supercollider
"test_diagnostico.scd".loadRelative;
```

## 📋 Verificación final

Después de cargar, ejecuta esto para confirmar que todo funciona:
```supercollider
(
if (~scvTestBasicEvents.notNil, {
    "✓ scvTestBasicEvents está disponible".postln;
    "✓ PROBLEMA RESUELTO".postln;
}, {
    "✗ scvTestBasicEvents AÚN no está disponible".postln;
    "✗ Intenta la carga manual mínima (Opción 2 arriba)".postln;
});
)
```

## 🎼 Una vez que funcione

Asegúrate de que el visualizador Rust esté ejecutándose:
```bash
cd /Users/sdcarr/Documents/github/sc-score/sc_score_visualizer
cargo run
```

**NOTA IMPORTANTE**: Si el visualizador se cierra inesperadamente con el error:
```
thread 'main' panicked at 'attempt to add with overflow'
```

Esto se debe a un overflow en el cálculo de colores que **ya ha sido corregido**. Simplemente reinicia el visualizador con `cargo run`.

Y luego ejecuta:
```supercollider
~scvSendPoint.(440, 0.8, 3.0);
```

### 🚨 Si el visualizador se cierra al enviar eventos

Si experimentas cierres inesperados del visualizador:

1. **Reinicia el visualizador**: `cargo run`
2. **Verifica que reciba mensajes**: Deberías ver logs como:
   ```
   📨 Mensaje OSC recibido: /event con 10 argumentos
   ✅ Evento point creado: freq=440, amp=0.8, dur=3
   ```
3. **Si sigue fallando**: Usa eventos más simples:
   ```supercollider
   "test_simple_corregido.scd".loadRelative;
   ```

---

**Resumen: El archivo principal ha sido corregido y ahora debería funcionar. Si no funciona, usa las opciones alternativas listadas arriba.**
