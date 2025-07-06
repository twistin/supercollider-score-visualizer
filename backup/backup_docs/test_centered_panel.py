#!/usr/bin/env python3
"""
Script para validar el centrado perfecto del panel de información
Verifica que el contenido esté perfectamente centrado en el rectángulo azul
"""

import socket
import struct
import time

def send_osc_message(host, port, address, *args):
    """Envía un mensaje OSC básico"""
    addr_bytes = address.encode('utf-8') + b'\x00'
    while len(addr_bytes) % 4 != 0:
        addr_bytes += b'\x00'
    
    types = ','
    arg_bytes = b''
    
    for arg in args:
        if isinstance(arg, str):
            types += 's'
            encoded = arg.encode('utf-8') + b'\x00'
            while len(encoded) % 4 != 0:
                encoded += b'\x00'
            arg_bytes += encoded
        elif isinstance(arg, float):
            types += 'f'
            arg_bytes += struct.pack('>f', arg)
        elif isinstance(arg, int):
            types += 'i'
            arg_bytes += struct.pack('>i', arg)
    
    types_bytes = types.encode('utf-8') + b'\x00'
    while len(types_bytes) % 4 != 0:
        types_bytes += b'\x00'
    
    message = addr_bytes + types_bytes + arg_bytes
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.sendto(message, (host, port))
    sock.close()

def test_centered_panel():
    """Prueba específicamente el centrado perfecto del panel"""
    host = "127.0.0.1"
    port = 57124
    
    print("🎯 VALIDACIÓN DEL PANEL PERFECTAMENTE CENTRADO")
    print("=" * 55)
    print("📍 VERIFICACIONES VISUALES REQUERIDAS:")
    print("   ✅ Panel en esquina superior derecha")
    print("   ✅ Rectángulo negro con borde azul grueso")
    print("   ✅ Contenido CENTRADO en el rectángulo:")
    print("      🎵 SC Score Visualizer")
    print("      📊 Eventos: [contador]")
    print("      ⏱️  Tiempo: [segundos]")
    print("      📡 OSC: 57124")
    print("   ✅ Texto color AZUL homogéneo con el borde")
    print("   ✅ Espaciado uniforme y simétrico")
    print("=" * 55)
    
    # Enviar eventos graduales para verificar centrado
    for i in range(8):
        freq = 220 * (1.5 ** (i % 4))  # Progresión musical
        amp = 0.5 + (i * 0.05)
        dur = 1.5 + (i % 3) * 0.5
        
        send_osc_message(host, port, "/event", "point", freq, amp, dur)
        
        print(f"📍 Evento {i+1}: {freq:.0f}Hz")
        print(f"   👁️  VERIFICA: ¿Texto centrado horizontal y verticalmente?")
        print(f"   🎨 VERIFICA: ¿Color azul homogéneo con el borde?")
        print(f"   📊 Contador de eventos: {i+1}")
        print(f"   ⏱️  Tiempo: ~{(i+1)*2} segundos")
        print("-" * 40)
        
        time.sleep(2.0)
    
    print("🎯 PRUEBA DE CENTRADO COMPLETADA")
    print()
    print("✅ CHECKLIST FINAL DE VALIDACIÓN:")
    print("   □ El panel está en la esquina superior derecha")
    print("   □ El rectángulo tiene borde azul grueso y bien definido")
    print("   □ TODO el texto está DENTRO del rectángulo")
    print("   □ El texto está CENTRADO horizontal y verticalmente")
    print("   □ El color del texto es azul, homogéneo con el borde")
    print("   □ El espaciado entre líneas es uniforme")
    print("   □ El contador de eventos se actualiza correctamente")
    print("   □ El tiempo incrementa en tiempo real")
    print()
    print("🎨 Si todas las verificaciones son ✅, el panel está")
    print("   PERFECTAMENTE CENTRADO Y CON HOMOGENEIDAD VISUAL")

if __name__ == "__main__":
    test_centered_panel()
