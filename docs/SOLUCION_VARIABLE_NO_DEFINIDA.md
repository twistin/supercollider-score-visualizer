# Solución al Error: "Variable 'scvTestBasicEvents' not defined"

## Problema
```
ERROR: Variable 'scvTestBasicEvents' not defined.
  in interpreted text
  line 1 char 18:
  scvTestBasicEvents.() 
```

## Causa
Este error ocurre cuando:
1. El bloque principal del archivo `supercollider_examples.scd` no se ha ejecutado completamente
2. Hubo un error durante la ejecución que impidió la exportación de las variables globales
3. Las variables se ejecutaron en un contexto diferente al esperado

## Soluciones

### Solución 1: Usar la versión robusta (RECOMENDADO)

1. Usa el archivo `supercollider_examples_robust.scd` que está optimizado para evitar estos errores
2. En SuperCollider, ejecuta:
   ```supercollider
   // Cargar el archivo completo
   "supercollider_examples_robust.scd".loadRelative;
   ```
3. O copia y pega todo el contenido del archivo en SuperCollider y ejecuta el bloque completo

### Solución 2: Diagnóstico paso a paso

1. Usa el archivo `test_diagnostico.scd` para verificar el estado:
   ```supercollider
   "test_diagnostico.scd".loadRelative;
   ```

2. Ejecuta cada sección paso a paso para identificar dónde falla

### Solución 3: Verificación manual

1. Ejecuta este código para verificar qué variables globales están disponibles:
   ```supercollider
   (
   "Variables globales disponibles con 'scv':".postln;
   currentEnvironment.keys.do { |key|
       if (key.asString.beginsWith("scv"), {
           ("  ~" ++ key).postln;
       });
   };
   )
   ```

2. Si no aparece ninguna variable, ejecuta TODO el bloque principal del archivo:
   ```supercollider
   // Selecciona TODO el contenido entre ( ... ) del archivo y ejecuta
   // IMPORTANTE: Debe ser el bloque completo, no líneas individuales
   ```

### Solución 4: Carga manual de función básica

Si necesitas una solución rápida, ejecuta esto:
```supercollider
(
var visualizer = NetAddr.new("127.0.0.1", 57120);
var testFunc = {
    "Enviando evento de prueba...".postln;
    visualizer.sendMsg("/event", "point", 440, 0.5, 1.0, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
};
~scvTestBasicEvents = testFunc;
"Función básica cargada. Ejecuta: ~scvTestBasicEvents.()".postln;
)
```

## Verificación de que funcionó

Después de cualquier solución, ejecuta:
```supercollider
(
if (~scvTestBasicEvents.notNil, {
    "✓ scvTestBasicEvents está disponible".postln;
}, {
    "✗ scvTestBasicEvents AÚN no está disponible".postln;
});
)
```

## Notas importantes

1. **Ejecutar bloques completos**: SuperCollider requiere que ejecutes el bloque COMPLETO entre paréntesis `( ... )`, no líneas individuales
2. **Orden de ejecución**: Las variables globales solo estarán disponibles DESPUÉS de ejecutar el bloque que las define
3. **Contexto de variables**: Las variables `~` son globales y persisten entre ejecuciones
4. **Verificar el visualizador**: Asegúrate de que el programa Rust esté ejecutándose en puerto 57120

## Archivos relevantes

- `supercollider_examples_robust.scd` - Versión mejorada y más robusta
- `test_diagnostico.scd` - Herramientas de diagnóstico
- `supercollider_examples.scd` - Archivo original (puede tener problemas de carga)

## Si nada funciona

1. Reinicia SuperCollider completamente
2. Usa la versión robusta del archivo
3. Ejecuta el diagnóstico paso a paso
4. Verifica que el visualizador Rust esté ejecutándose
