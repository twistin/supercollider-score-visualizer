#!/usr/bin/env python3
"""
Script para hacer visible la ventana del visualizador
"""

import subprocess
import time
import os

def find_and_focus_visualizer():
    """Buscar y enfocar la ventana del visualizador"""
    print("🔍 Buscando proceso del visualizador...")
    
    # Verificar que el proceso esté ejecutándose
    try:
        result = subprocess.run(['pgrep', '-f', 'sc_score_visualizer'], 
                              capture_output=True, text=True)
        if result.returncode == 0:
            pid = result.stdout.strip()
            print(f"✅ Visualizador encontrado con PID: {pid}")
        else:
            print("❌ Visualizador no está ejecutándose")
            return False
    except Exception as e:
        print(f"❌ Error buscando proceso: {e}")
        return False
    
    # Intentar hacer visible la ventana usando osascript
    apple_script = '''
    tell application "System Events"
        set foundWindow to false
        repeat with theProcess in (get processes)
            try
                if name of theProcess contains "sc_score_visualizer" then
                    set frontmost of theProcess to true
                    set foundWindow to true
                    exit repeat
                end if
            end try
        end repeat
        return foundWindow
    end tell
    '''
    
    try:
        result = subprocess.run(['osascript', '-e', apple_script], 
                              capture_output=True, text=True)
        if result.returncode == 0:
            print("✅ Comando de enfoque ejecutado")
        else:
            print(f"⚠️ Resultado del script: {result.stdout}")
    except Exception as e:
        print(f"⚠️ Error con osascript: {e}")
    
    return True

def test_window_creation():
    """Test para verificar la creación de ventana"""
    print("🖥️  Probando creación de ventana...")
    
    # Usar comando de macOS para listar ventanas
    try:
        # Buscar ventanas que contengan "SC Score" o "Visualizer"
        result = subprocess.run([
            'osascript', '-e', 
            'tell application "System Events" to get name of every window of every process'
        ], capture_output=True, text=True)
        
        if "SC Score" in result.stdout or "Visualizer" in result.stdout:
            print("✅ Ventana del visualizador encontrada en el sistema")
            return True
        else:
            print("❌ Ventana del visualizador NO encontrada")
            print(f"Ventanas disponibles: {result.stdout[:200]}...")
            return False
            
    except Exception as e:
        print(f"❌ Error verificando ventanas: {e}")
        return False

if __name__ == "__main__":
    print("🎯 === TEST DE VENTANA DEL VISUALIZADOR ===")
    
    # Test 1: Verificar proceso
    if find_and_focus_visualizer():
        print("\n🎯 Test 1: ✅ Proceso encontrado")
    else:
        print("\n🎯 Test 1: ❌ Proceso no encontrado")
        exit(1)
    
    # Test 2: Verificar ventana
    time.sleep(2)
    if test_window_creation():
        print("\n🎯 Test 2: ✅ Ventana encontrada")
    else:
        print("\n🎯 Test 2: ❌ Ventana no encontrada")
    
    print("\n✅ Tests completados")
    print("Si no ves la ventana, puede estar fuera de pantalla o minimizada")
    print("Intenta usar Cmd+Tab para cambiar entre aplicaciones")
