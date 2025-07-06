#!/usr/bin/env python3
"""
Script de verificación para SC Score Visualizer
Envía una secuencia de eventos de prueba para verificar que todo funciona correctamente
"""

import socket
import struct
import time
import math
import random

class OSCClient:
    def __init__(self, host="127.0.0.1", port=57120):
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.addr = (host, port)
    
    def _pad_string(self, s):
        """Pad string to multiple of 4 bytes with null terminators"""
        s += '\0'
        while len(s) % 4 != 0:
            s += '\0'
        return s.encode('utf-8')
    
    def _encode_osc_message(self, address, *args):
        """Encode OSC message"""
        # Address
        msg = self._pad_string(address)
        
        # Type tag
        type_tag = ','
        arg_data = b''
        
        for arg in args:
            if isinstance(arg, str):
                type_tag += 's'
                arg_data += self._pad_string(arg)
            elif isinstance(arg, int):
                type_tag += 'i'
                arg_data += struct.pack('>i', arg)
            elif isinstance(arg, float):
                type_tag += 'f'
                arg_data += struct.pack('>f', arg)
        
        msg += self._pad_string(type_tag)
        msg += arg_data
        
        return msg
    
    def send_message(self, address, *args):
        """Send OSC message"""
        msg = self._encode_osc_message(address, *args)
        try:
            self.sock.sendto(msg, self.addr)
            return True
        except Exception as e:
            print(f"Error enviando mensaje: {e}")
            return False

def test_basic_events():
    """Prueba básica de todos los tipos de eventos"""
    client = OSCClient()
    
    print("=== Prueba Básica de Eventos ===")
    
    # Test 1: Punto simple
    print("1. Enviando punto...")
    success = client.send_message("/event", "point", 440.0, 0.5, 2.0, 0.1, 0.1, 0.5, 0.0, 0.0, 220.0)
    if success:
        print("   ✓ Punto enviado")
    else:
        print("   ✗ Error enviando punto")
    
    time.sleep(1)
    
    # Test 2: Glissando
    print("2. Enviando glissando...")
    success = client.send_message("/event", "gliss", 220.0, 880.0, 0.7, 3.0, 0.5, 0.8, 0.2, 0.3, 60.0)
    if success:
        print("   ✓ Glissando enviado")
    else:
        print("   ✗ Error enviando glissando")
    
    time.sleep(1)
    
    # Test 3: Cluster
    print("3. Enviando cluster...")
    success = client.send_message("/event", "cluster", 660.0, 150.0, 8.0, 0.6, 4.0, 0.8, 0.3, 0.5, 120.0)
    if success:
        print("   ✓ Cluster enviado")
    else:
        print("   ✗ Error enviando cluster")
    
    time.sleep(1)
    
    # Test 4: Ruido
    print("4. Enviando ruido...")
    success = client.send_message("/event", "noise", 1000.0, 500.0, 0.4, 2.5, 1.0, 0.0, 0.6, 0.8, 300.0)
    if success:
        print("   ✓ Ruido enviado")
    else:
        print("   ✗ Error enviando ruido")
    
    time.sleep(1)
    
    # Test 5: Masa sonora simple
    print("5. Enviando masa sonora...")
    success = client.send_message("/event", "mass", 4.0, 0.5, 3.0, 0.5, 0.3, 0.9, 0.5, 0.7, 180.0,
                                440.0, 1.0, 554.37, 0.8, 659.25, 0.6, 783.99, 0.4)
    if success:
        print("   ✓ Masa sonora enviada")
    else:
        print("   ✗ Error enviando masa sonora")
    
    print("\n=== Prueba Básica Completada ===")

def test_xenakis_style():
    """Prueba estilo Xenakis con eventos más complejos"""
    client = OSCClient()
    
    print("\n=== Prueba Estilo Xenakis ===")
    
    # Secuencia de puntos dispersos
    print("Enviando secuencia de puntos dispersos...")
    for i in range(20):
        freq = 200 + (2000 * i / 20) + random.uniform(-100, 100)
        amp = 0.3 + random.uniform(0, 0.4)
        density = i / 20
        texture = (i / 20) ** 2
        hue = random.uniform(0, 360)
        
        client.send_message("/event", "point", freq, amp, 0.5 + random.uniform(0, 1.0), 
                          0.05, 0.1, density, texture, random.uniform(0, 0.2), hue)
        time.sleep(0.2)
    
    print("✓ Secuencia de puntos completada")
    
    # Glissandi convergentes
    print("Enviando glissandi convergentes...")
    center_freq = 440
    for i in range(6):
        start_freq = center_freq * (0.5 + random.uniform(0, 1.5))
        end_freq = center_freq * (0.5 + random.uniform(0, 1.5))
        curvature = random.uniform(-1.0, 1.0)
        duration = 2.0 + random.uniform(0, 3.0)
        hue = 30 + random.uniform(0, 60)
        
        client.send_message("/event", "gliss", start_freq, end_freq, 
                          0.4 + random.uniform(0, 0.3), duration, curvature, 
                          0.8, random.uniform(0, 0.2), random.uniform(0, 0.3), hue)
        time.sleep(0.3)
    
    print("✓ Glissandi completados")
    
    # Nube de clusters
    print("Enviando nube de clusters...")
    for i in range(4):
        center_freq = 100 + random.uniform(0, 3000)
        spread = 50 + random.uniform(0, 200)
        num_voices = 4 + random.randint(0, 12)
        duration = 2.0 + random.uniform(0, 3.0)
        hue = random.uniform(0, 360)
        
        client.send_message("/event", "cluster", center_freq, spread, num_voices,
                          0.5 + random.uniform(0, 0.4), duration, 
                          0.7 + random.uniform(0, 0.3), random.uniform(0, 0.5), 
                          random.uniform(0, 0.6), hue)
        time.sleep(1.0)
    
    print("✓ Nube de clusters completada")
    
    print("\n=== Prueba Estilo Xenakis Completada ===")

def test_connection():
    """Prueba de conexión básica"""
    client = OSCClient()
    
    print("=== Prueba de Conexión ===")
    print("Intentando conectar a 127.0.0.1:57120...")
    
    # Intentar enviar un mensaje simple
    success = client.send_message("/test", "hello")
    
    if success:
        print("✓ Conexión exitosa - socket puede enviar datos")
        print("  Si el visualizador está funcionando, debería mostrar un mensaje en la consola")
    else:
        print("✗ Error de conexión")
        print("  Verifica que:")
        print("  1. El visualizador Rust esté ejecutándose")
        print("  2. Esté escuchando en el puerto 57120")
        print("  3. No haya firewall bloqueando la conexión")
    
    return success

def main():
    """Función principal de verificación"""
    print("SC Score Visualizer - Script de Verificación")
    print("=" * 50)
    
    # Prueba de conexión
    if not test_connection():
        print("\nNo se pudo establecer conexión. Saliendo...")
        return
    
    print("\nEsperando 2 segundos antes de continuar...")
    time.sleep(2)
    
    # Menú de pruebas
    while True:
        print("\nOpciones disponibles:")
        print("1. Prueba básica de eventos")
        print("2. Prueba estilo Xenakis")
        print("3. Prueba de conexión solamente")
        print("4. Salir")
        
        try:
            choice = input("\nSelecciona una opción (1-4): ").strip()
            
            if choice == "1":
                test_basic_events()
            elif choice == "2":
                test_xenakis_style()
            elif choice == "3":
                test_connection()
            elif choice == "4":
                print("Saliendo...")
                break
            else:
                print("Opción inválida")
                
        except KeyboardInterrupt:
            print("\nInterrumpido por el usuario. Saliendo...")
            break
        except Exception as e:
            print(f"Error: {e}")

if __name__ == "__main__":
    main()
