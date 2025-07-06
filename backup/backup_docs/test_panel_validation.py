#!/usr/bin/env python3
"""
Script específico para validar el panel de información
Envía eventos continuos para mantener el panel activo y verificar alineación
"""

import socket
import struct
import time
import random

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

def test_panel_alignment():
    """Prueba específicamente la alineación del panel de información"""
    host = "127.0.0.1"
    port = 57124
    
    print("🔍 VALIDACIÓN DEL PANEL DE INFORMACIÓN")
    print("=" * 50)
    print("📍 Verificando que el contenido esté DENTRO del rectángulo")
    print("🎯 El panel debe mostrar:")
    print("   🎵 SC Score Visualizer")
    print("   📊 Eventos: [contador que aumenta]")
    print("   ⏱️  Tiempo: [tiempo en segundos]")
    print("   📡 OSC: 57124")
    print("=" * 50)
    
    # Enviar eventos cada 2 segundos para mantener actividad
    for i in range(10):
        # Diferentes tipos de eventos para probar
        if i % 3 == 0:
            # Evento punto
            freq = 220 + (i * 50)
            send_osc_message(host, port, "/event", "point", freq, 0.6, 1.5)
            print(f"📍 Enviado: Punto {freq}Hz (Total eventos: {i+1})")
        
        elif i % 3 == 1:
            # Evento glissando
            start_freq = 220 + (i * 30)
            end_freq = start_freq * 1.5
            send_osc_message(host, port, "/event", "gliss", start_freq, end_freq, 0.0, 0.7, 2.0)
            print(f"🎵 Enviado: Glissando {start_freq:.0f}→{end_freq:.0f}Hz (Total eventos: {i+1})")
        
        else:
            # Evento cluster
            center_freq = 440 + (i * 40)
            send_osc_message(host, port, "/event", "cluster", center_freq, 50.0, 3, 0.5, 1.8)
            print(f"🎆 Enviado: Cluster {center_freq}Hz (Total eventos: {i+1})")
        
        print(f"⏱️  Tiempo transcurrido: ~{(i+1)*2} segundos")
        print("👀 VERIFICA: ¿El texto está DENTRO del rectángulo negro?")
        print("-" * 30)
        
        time.sleep(2.0)
    
    print("✅ PRUEBA COMPLETADA")
    print("🔍 Validación visual necesaria:")
    print("   ✓ Panel en esquina superior derecha")
    print("   ✓ Rectángulo negro con borde azul")
    print("   ✓ Todo el texto DENTRO del rectángulo")
    print("   ✓ Contador de eventos actualizado")
    print("   ✓ Tiempo incrementándose")

if __name__ == "__main__":
    test_panel_alignment()
