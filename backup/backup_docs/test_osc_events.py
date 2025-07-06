#!/usr/bin/env python3
"""
Script de prueba para enviar eventos OSC al visualizador
Verifica que el panel de información se muestre correctamente
"""

import socket
import struct
import time
import random

def send_osc_message(host, port, address, *args):
    """Envía un mensaje OSC básico"""
    # Construir mensaje OSC básico
    addr_bytes = address.encode('utf-8') + b'\x00'
    while len(addr_bytes) % 4 != 0:
        addr_bytes += b'\x00'
    
    # Tipos de argumentos
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
    
    # Completar types
    types_bytes = types.encode('utf-8') + b'\x00'
    while len(types_bytes) % 4 != 0:
        types_bytes += b'\x00'
    
    # Construir mensaje completo
    message = addr_bytes + types_bytes + arg_bytes
    
    # Enviar via UDP
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.sendto(message, (host, port))
    sock.close()

def test_panel_events():
    """Envía eventos de prueba para verificar el panel"""
    host = "127.0.0.1"
    port = 57124
    
    print("🎵 Enviando eventos de prueba al visualizador...")
    print(f"📡 Destino: {host}:{port}")
    print("⏱️  Enviando eventos cada segundo...")
    
    for i in range(5):
        # Eventos punto con diferentes frecuencias
        freq = 220 * (2 ** (i * 0.5))  # Escala musical
        amp = 0.5 + random.random() * 0.4
        dur = 1.0 + random.random() * 2.0
        
        send_osc_message(host, port, "/event", "point", freq, amp, dur)
        print(f"✅ Evento {i+1}: punto {freq:.1f}Hz, amp={amp:.2f}, dur={dur:.1f}s")
        
        time.sleep(1.0)
    
    print("🎯 Eventos enviados. Verifica que el panel muestre el contador actualizado.")

if __name__ == "__main__":
    test_panel_events()
