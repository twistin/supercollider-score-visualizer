#!/bin/bash

echo "🔍 Verificando ventana del visualizador..."

# Buscar ventanas del visualizador
APP_NAME="SC Score Visualizer"
WINDOW_INFO=$(osascript -e "tell application \"System Events\" to get the name of every window of every process")

echo "Buscando ventana: $APP_NAME"

# Intentar hacer foco en la ventana
osascript <<EOF
tell application "System Events"
    set windowFound to false
    repeat with theProcess in (get processes whose name contains "sc_score_visualizer")
        repeat with theWindow in (get windows of theProcess)
            if name of theWindow contains "SC Score Visualizer" then
                set windowFound to true
                set frontmost of theProcess to true
                perform action "AXRaise" of theWindow
                exit repeat
            end if
        end repeat
    end repeat
    
    if not windowFound then
        display dialog "No se encontró la ventana del visualizador. ¿Está ejecutándose?" buttons {"OK"} default button "OK"
    else
        display dialog "Ventana del visualizador encontrada y enfocada" buttons {"OK"} default button "OK"
    end if
end tell
EOF

echo "✅ Script de verificación completado"
