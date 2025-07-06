#!/usr/bin/env python3
"""
Script para verificar la sintaxis básica del archivo SuperCollider
"""

def check_supercollider_syntax(filename):
    """Verificación básica de sintaxis SuperCollider"""
    with open(filename, 'r') as f:
        content = f.read()
    
    # Contadores de paréntesis y llaves
    parens = 0
    braces = 0
    
    # Verificaciones básicas
    issues = []
    
    for line_num, line in enumerate(content.split('\n'), 1):
        # Contar paréntesis y llaves
        parens += line.count('(') - line.count(')')
        braces += line.count('{') - line.count('}')
        
        # Verificar líneas que terminan con punto y coma suelto
        stripped = line.strip()
        if stripped.endswith(';') and not stripped.startswith('//'):
            # Verificar si es una cadena suelta (problemática)
            if stripped.startswith('"') and stripped.count('"') >= 2:
                issues.append(f"Línea {line_num}: Posible cadena suelta con semicolon: {stripped}")
    
    # Verificar balance de paréntesis y llaves
    if parens != 0:
        issues.append(f"Paréntesis desbalanceados: {parens}")
    if braces != 0:
        issues.append(f"Llaves desbalanceadas: {braces}")
    
    return issues

if __name__ == "__main__":
    import sys
    filename = sys.argv[1] if len(sys.argv) > 1 else "supercollider_examples.scd"
    print(f"Verificando sintaxis de {filename}...")
    
    try:
        issues = check_supercollider_syntax(filename)
        
        if not issues:
            print("✅ No se encontraron problemas de sintaxis obvios")
            print("✅ Paréntesis y llaves están balanceados")
            print("✅ No hay cadenas sueltas con semicolon")
        else:
            print("⚠️  Se encontraron posibles problemas:")
            for issue in issues:
                print(f"   {issue}")
                
    except Exception as e:
        print(f"❌ Error al verificar archivo: {e}")
