import React from 'react';
import { Week, Resource, SiteLink } from '../types';
import { CodeBlock } from '../components/CodeBlock';
import { OwnershipVisual } from '../components/OwnershipVisual';
import { PlayIcon } from '../components/icons/PlayIcon';
import { PuzzlePieceIcon } from '../components/icons/PuzzlePieceIcon';
import {CommandLineIcon} from '../components/icons/CommandLineIcon';
import { BookOpenIcon } from '../components/icons/BookOpenIcon';
import {GlobeAltIcon} from '../components/icons/GlobeAltIcon';
import { UsersIcon } from '../components/icons/UsersIcon';
import { CodeBracketIcon } from '../components/icons/CodeBracketIcon';
import { LinkIcon } from '../components/icons/LinkIcon';

export const studyPlan: Week[] = [
  {
    id: 1,
    title: 'Semana 1: Fundamentos',
    summary: 'Sentaremos las bases del lenguaje Rust, desde la instalación hasta la sintaxis básica y el control de flujo.',
    topics: [
      {
        title: 'Instalación de Rust',
        content: <p>Utiliza la herramienta <strong>rustup</strong> para instalar Rust en tu sistema desde la página oficial de Rust (rust-lang.org). Rustup administrará las versiones de Rust por ti.</p>,
      },
      {
        title: 'Tu primer programa – Hello, world!',
        content: (
          <>
            <p>Crea un proyecto simple y escribe el clásico programa que imprime "Hello, world!" en la pantalla. Esto te familiarizará con la estructura básica de un programa Rust (función `main`, macros como `println!`, etc.).</p>
            <CodeBlock>
{`fn main() {
    println!("Hello, world!");
}`}
            </CodeBlock>
          </>
        ),
      },
      {
        title: 'Variables y Mutabilidad',
        content: (
            <>
                <p>En Rust las variables son <strong>inmutables por defecto</strong>. Para declarar variables se usa la palabra clave `let`. Si necesitas que una variable sea mutable (cambiable), debes usar `mut`.</p>
                <CodeBlock>
{`let x = 5; // Inmutable
let mut contador = 0; // Mutable
contador += 1;`}
                </CodeBlock>
                <p>Este diseño fomenta la seguridad al evitar cambios inesperados, pero te da la opción de mutabilidad explícita cuando la necesitas.</p>
            </>
        )
      },
      {
        title: 'Tipos de Datos Básicos',
        content: <p>Rust es un lenguaje de <strong>tipado estático</strong>. Aprenderás los tipos escalares como enteros (ej. `u32`, `i64`), números de punto flotante (`f32`, `f64`), booleanos (`bool`) y caracteres Unicode (`char`). Un `char` en Rust ocupa 4 bytes, permitiendo representar emojis y otros símbolos.</p>,
      },
      {
        title: 'Control de Flujo',
        content: (
            <>
                <p>Rust proporciona `if`, el poderoso patrón `match`, y los bucles: `loop` (infinito), `while` (condicional) y `for` (iteración).</p>
                <CodeBlock>
{`// Bucle for
for numero in 1..=5 {
    println!("El número es {}", numero);
}

// Match
let x = 5;
match x {
    1 => println!("Es uno"),
    2 | 3 | 4 | 5 => println!("Está entre 2 y 5"),
    _ => println!("Es otro número"),
}`}
                </CodeBlock>
            </>
        )
      }
    ],
    reading: <p>Los capítulos 1 al 3 de <em>The Rust Programming Language</em> cubren estos conceptos. Complementa con <em>Rust By Example</em> para ver más código práctico.</p>,
    exercise: <p>Crea un programa que solicite tu nombre al usuario y luego imprima un saludo personalizado. Después, practica los bucles imprimiendo los números del 1 al 10 usando un `for`.</p>,
    exerciseSolution: (
        <>
            <p>Para leer la entrada del usuario, necesitamos importar el módulo `io` de la librería estándar. El código podría verse así:</p>
            <CodeBlock>
{`use std::io;

fn main() {
    // ---- Saludo personalizado ----
    println!("Por favor, introduce tu nombre:");

    let mut nombre = String::new();

    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la línea");

    // .trim() elimina los espacios en blanco y el salto de línea
    println!("¡Hola, {}!", nombre.trim());

    // ---- Bucle del 1 al 10 ----
    println!("\nAhora, contemos hasta 10:");
    for i in 1..=10 {
        println!("{}", i);
    }
}`}
            </CodeBlock>
        </>
    )
  },
  {
    id: 2,
    title: 'Semana 2: Funciones y Ownership',
    summary: 'Profundizaremos en conceptos clave de Rust: el sistema de propiedad (ownership), préstamos (borrowing) y estructuras de datos personalizadas.',
    topics: [
      {
        title: 'Funciones en Rust',
        content: (
            <>
                <p>Aprenderás a declarar funciones con `fn`. La última expresión de una función es su valor de retorno implícito si no lleva punto y coma.</p>
                <CodeBlock>
{`fn sumar(a: i32, b: i32) -> i32 {
    a + b  // devuelve la suma
}`}
                </CodeBlock>
            </>
        )
      },
      {
        title: 'El sistema de ownership (propiedad)',
        content: (
            <>
                <p>Este es el núcleo de la gestión de memoria en Rust. Cada valor tiene un <strong>único propietario</strong>. Cuando el propietario sale de su ámbito (scope), el valor es liberado automáticamente. Rust no tiene recolector de basura.</p>
                <ul className="list-disc list-inside space-y-2 my-4">
                    <li>Cada valor tiene un solo propietario.</li>
                    <li>Cuando el propietario sale de alcance, el valor se libera.</li>
                    <li>No puede haber dos propietarios a la vez para un mismo recurso.</li>
                </ul>
            </>
        )
      },
      {
        title: 'Borrowing (préstamo de referencias)',
        content: (
            <>
                <p>Podemos <strong>prestar</strong> una referencia a un valor usando `&` (inmutable) o `&mut` (mutable) sin transferir la propiedad. Esto permite acceder a datos de forma segura y eficiente.</p>
                <ul className="list-disc list-inside space-y-2 my-4">
                    <li>Puedes tener cualquier número de referencias <strong>inmutables</strong> (`&`).</li>
                    <li>O puedes tener <strong>una sola referencia mutable</strong> (`&mut`).</li>
                    <li>No puedes mezclar préstamos mutables e inmutables al mismo tiempo.</li>
                </ul>
                <OwnershipVisual />
            </>
        )
      },
      {
        title: 'Lifetimes (tiempos de vida)',
        content: <p>Los lifetimes son una forma de conectar la vida de las referencias con la vida de los datos a los que apuntan. El compilador los usa para garantizar que todas las referencias sean siempre válidas, evitando punteros colgantes. La mayoría de las veces, el compilador los infiere automáticamente.</p>
      },
      {
        title: 'Estructuras (struct) e Implementación (impl)',
        content: (
            <>
                <p>Define tus propios tipos de datos con `struct`. Añade comportamiento (métodos) a tus estructuras usando un bloque `impl`.</p>
                <CodeBlock>
{`struct Persona {
    nombre: String,
    edad: u8,
}

impl Persona {
    fn presentacion(&self) {
        println!("Me llamo {} y tengo {} años.", self.nombre, self.edad);
    }
}

fn main() {
    let persona1 = Persona {
        nombre: String::from("Alice"),
        edad: 30,
    };
    persona1.presentacion();
}`}
                </CodeBlock>
            </>
        )
      },
    ],
    reading: <p>Los capítulos 4 al 6 del “Libro de Rust” tratan en profundidad ownership, referencias, lifetimes y structs. Tómate tu tiempo, son conceptos densos pero fundamentales.</p>,
    exercise: (
        <>
            <p className="mb-2">1. Escribe una función `calcula_longitud` que reciba una referencia `&str` y devuelva su longitud (`usize`), sin tomar propiedad de la `String` original.</p>
            <p>2. Crea la `struct Persona` del ejemplo. En `main`, crea una instancia y llama a su método `presentacion`.</p>
        </>
    ),
    exerciseSolution: (
         <>
            <p>El código completo combinando ambos ejercicios sería:</p>
            <CodeBlock>
{`// 2. Definición de la struct y su implementación
struct Persona {
    nombre: String,
    edad: u8,
}

impl Persona {
    fn presentacion(&self) {
        println!("Me llamo {} y tengo {} años.", self.nombre, self.edad);
    }
}

// 1. Función que toma una referencia de slice de string
fn calcula_longitud(s: &str) -> usize {
    s.len()
}

fn main() {
    // Ejercicio 1:
    let mi_string = String::from("hola mundo");
    let longitud = calcula_longitud(&mi_string);
    
    println!("El string '{}' tiene {} caracteres.", mi_string, longitud);
    println!("Aún podemos usar mi_string aquí: {}", mi_string); // Demuestra que no se movió

    // Ejercicio 2:
    let persona1 = Persona {
        nombre: String::from("Carlos"),
        edad: 28,
    };
    persona1.presentacion();
}`}
            </CodeBlock>
        </>
    )
  },
  {
    id: 3,
    title: 'Semana 3: Colecciones y Errores',
    summary: 'Exploraremos las colecciones estándar de Rust, el manejo de errores robusto con `Result` y `Option`, y cómo organizar el código en módulos.',
    topics: [
      {
        title: 'Colecciones estándar',
        content: (
            <>
                <p>Rust proporciona colecciones muy útiles en la librería estándar:</p>
                <ul className="list-disc list-inside space-y-2 my-4">
                    <li><strong>Vectores (`Vec{'<T>'}`):</strong> Listas dinámicas que pueden crecer y encoger.</li>
                    <li><strong>Strings (`String` y `&str`):</strong> El tipo `String` es una cadena mutable y `&str` es una referencia a una cadena.</li>
                    <li><strong>HashMaps (`HashMap{'<K, V>'}`):</strong> Estructuras de clave-valor, similares a los diccionarios de Python.</li>
                </ul>
                <CodeBlock>
{`use std::collections::HashMap;

let mut numeros = vec![1, 2, 3];
numeros.push(4); // numeros ahora es [1, 2, 3, 4]

let mut notas = HashMap::new();
notas.insert(String::from("Alice"), 10);
notas.insert(String::from("Bob"), 9);`}
                </CodeBlock>
            </>
        )
      },
      {
        title: 'Manejo de errores con Result y Option',
        content: (
            <>
                <p>En Rust los errores se manejan mediante tipos en lugar de excepciones, haciendo el código más robusto.</p>
                 <ul className="list-disc list-inside space-y-2 my-4">
                    <li><strong>`Option{'<T>'}`:</strong> Representa un valor que puede existir (`Some(valor)`) o no (`None`).</li>
                    <li><strong>`Result{'<T, E>'}`:</strong> Representa una operación que puede tener éxito (`Ok(valor)`) o fallar (`Err(error)`).</li>
                </ul>
                <p>Para manejar estos tipos, se usa `match` o métodos como `if let`. El operador `?` es un atajo para propagar errores fácilmente.</p>
                <CodeBlock>
{`fn dividir(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

match dividir(10.0, 2.0) {
    Some(resultado) => println!("Resultado: {}", resultado),
    None => println!("No se puede dividir por cero"),
}`}
                </CodeBlock>
            </>
        )
      },
      {
        title: 'Módulos y organización del código',
        content: <p>A medida que tu programa crece, conviene separar el código en <strong>módulos</strong> (con `mod`) para mantenerlo organizado en espacios de nombres. Para usar elementos de un módulo en otro, se usa la palabra clave `use`.</p>
      },
    ],
    reading: <p>Los capítulos 7 a 9 del “Libro de Rust” cubren la organización del código en paquetes, crates y módulos, colecciones comunes y manejo de errores.</p>,
    exercise: (
        <>
            <p className="mb-2">1. Crea un `HashMap` para almacenar los nombres de estudiantes (como `String`) y sus notas (como `u8`).</p>
            <p className="mb-2">2. Escribe una función que reciba una referencia al `HashMap` y el nombre de un estudiante, y devuelva la nota del estudiante usando `Option{'<&u8>'}`. Si el estudiante no existe, la función debe devolver `None`.</p>
            <p>3. En `main`, llama a la función y usa `match` para imprimir la nota si existe, o un mensaje indicando que el estudiante no fue encontrado.</p>
        </>
    ),
    exerciseSolution: (
        <>
            <p>La clave aquí es usar `HashMap::get`, que devuelve un `Option` sobre una referencia al valor, evitando copias innecesarias.</p>
            <CodeBlock>
{`use std::collections::HashMap;

// 2. La función recibe una referencia al HashMap y un slice de string para la clave
fn obtener_nota<'a>(notas: &'a HashMap<String, u8>, nombre: &str) -> Option<&'a u8> {
    notas.get(nombre)
}

fn main() {
    // 1. Crear y poblar el HashMap
    let mut notas_clase = HashMap::new();
    notas_clase.insert(String::from("Ana"), 9);
    notas_clase.insert(String::from("Luis"), 7);
    notas_clase.insert(String::from("Eva"), 10);

    // 3. Probar la función
    let estudiantes = ["Ana", "Pedro"];
    for &estudiante in &estudiantes {
        println!("Buscando la nota de {}...", estudiante);
        match obtener_nota(&notas_clase, estudiante) {
            Some(nota) => println!("-> La nota de {} es: {}", estudiante, nota),
            None => println!("-> El estudiante {} no fue encontrado.", estudiante),
        }
    }
}`}
            </CodeBlock>
        </>
    ),
    project: (
        <>
            <p>¡Hora de combinar lo aprendido en un mini-proyecto de consola! Elige una de estas ideas:</p>
            <ul className="list-disc list-inside space-y-2 mt-4">
                <li><strong>Calculadora CLI:</strong> Un programa que lea dos números y una operación (+, -, *, /) y muestre el resultado. Practicarás parsing de strings a números (que devuelve un `Result`) y el uso de `match`.</li>
                <li><strong>Convertidor de temperaturas:</strong> Un programa que pida una temperatura (ej. "32F" o "100C") y la convierta a la otra unidad.</li>
            </ul>
             <p className="mt-4 text-sm text-slate-400">Puedes encontrar ejemplos de soluciones para estos proyectos en la sección "Anexo: Soluciones".</p>
        </>
    )
  },
   {
    id: 4,
    title: 'Semana 4: Proyecto y Pruebas',
    summary: 'Consolidaremos conocimientos construyendo un proyecto más complejo, aprendiendo a escribir pruebas automatizadas y a usar paquetes de la comunidad.',
    topics: [
      {
        title: 'Escritura de pruebas (`#[test]`)',
        content: (
            <>
                <p>Rust tiene soporte integrado para pruebas unitarias. Crea funciones de test anotadas con `#[test]` y usa macros como `assert_eq!` para verificar el comportamiento. Ejecútalas con `cargo test`.</p>
                <CodeBlock>
{`pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // Importa \`sumar\` del módulo padre

    #[test]
    fn test_suma() {
        assert_eq!(sumar(2, 2), 4);
    }
}`}
                </CodeBlock>
            </>
        )
      },
      {
        title: 'Cargo y crates.io (dependencias)',
        content: <p><strong>Cargo</strong> es el gestor de paquetes de Rust. Aprenderás a añadir dependencias de terceros desde <strong>crates.io</strong> (el registro de paquetes de Rust) editando tu archivo `Cargo.toml`. Esto te permite usar librerías para todo tipo de tareas, como desarrollo web, juegos, etc.</p>
      },
      {
        title: 'Proyecto final (mini API web)',
        content: (
            <>
                <p>Como colofón, te proponemos crear una pequeña API web usando un framework como <strong>Rocket</strong> o <strong>Axum</strong>. Esto te dará experiencia en un caso de uso real de Rust.</p>
                <p>Ejemplo de una ruta simple con Rocket:</p>
                 <CodeBlock>
{`#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
struct Mensaje { 
    mensaje: String 
}

#[get("/saludo")]
fn saludo() -> Json<Mensaje> {
    Json(Mensaje { mensaje: "¡Hola desde Rust!".into() })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![saludo])
}`}
                 </CodeBlock>
                <p>No te preocupes si no entiendes todo (como `async` o las macros). El objetivo es experimentar y ver Rust en acción.</p>
            </>
        )
      },
      {
        title: 'Repaso y mejoras',
        content: <p>Al finalizar tu proyecto, revisa si aplicaste las reglas de propiedad correctamente, manejaste errores con `Result` en lugar de `unwrap()`, y si escribiste algunas pruebas. ¡Refactorizar es parte del aprendizaje!</p>
      },
    ],
    reading: <p>Consulta la documentación oficial de los frameworks web como Rocket o Axum. El capítulo 11 del "Libro de Rust" es una excelente introducción a la escritura de pruebas.</p>,
    exercise: <p><strong>Desafío final:</strong> Escribe pruebas para los endpoints de tu API. Muchos frameworks permiten llamar a los "handlers" de las rutas directamente en los tests, sin necesidad de levantar un servidor real o hacer peticiones HTTP.</p>,
    exerciseSolution: (
        <>
            <p>Siguiendo el ejemplo de Rocket, un test para el endpoint `/saludo` no requiere un cliente HTTP. Podemos probar la función `saludo` directamente y verificar el JSON que produce.</p>
            <CodeBlock>
{`// Se añade este módulo al final del fichero src/main.rs
#[cfg(test)]
mod test {
    use super::saludo; // Importa la función handler

    #[test]
    fn test_saludo() {
        let respuesta_json = saludo();
        // El tipo Json se puede "desenvolver" para acceder a su contenido.
        let mensaje_interno = respuesta_json.into_inner(); 
        
        assert_eq!(mensaje_interno.mensaje, "¡Hola desde Rust!");
    }
}`}
            </CodeBlock>
            <p className="mt-2">Esta prueba es rápida y eficiente, ya que solo comprueba la lógica de la función, no toda la infraestructura web.</p>
        </>
    ),
    finalWords: (
      <div className="text-center">
        <h3 className="text-2xl font-bold text-slate-100 mb-2">🎉 ¡Felicidades! 🎉</h3>
        <p className="text-slate-300">Al completar este plan, habrás cubierto desde los fundamentos hasta la creación de un proyecto completo. Ahora tienes las bases para seguir explorando el ecosistema de Rust por tu cuenta.</p>
        <p className="font-semibold text-rust-orange mt-4">¡Mucho ánimo y happy coding! 🚀</p>
      </div>
    )
  }
];

export const resources: Resource[] = [
    {
        title: 'Playground de Rust',
        description: <p>Un entorno online en <a href="https://play.rust-lang.org" target="_blank" rel="noopener noreferrer" className="text-rust-orange hover:underline">play.rust-lang.org</a> para ejecutar código Rust al instante en tu navegador. Ideal para pruebas rápidas.</p>,
        icon: <PlayIcon className="w-8 h-8 text-green-400" />
    },
    {
        title: 'Rustlings',
        description: <p>Pequeños ejercicios para corregir que te guían a través de la sintaxis y conceptos del lenguaje. Se ejecuta en tu terminal.</p>,
        icon: <PuzzlePieceIcon className="w-8 h-8 text-yellow-400" />
    },
    {
        title: 'Rust by Example',
        description: <p>Recurso oficial que enseña Rust mostrando ejemplos de código para cada concepto, con explicaciones mínimas.</p>,
        icon: <CommandLineIcon className="w-8 h-8 text-blue-400" />
    },
    {
        title: 'The Rust Book',
        description: <p>La referencia principal y más completa para aprender Rust. Cubre todo desde lo básico hasta lo más avanzado en profundidad.</p>,
        icon: <BookOpenIcon className="w-8 h-8 text-purple-400" />
    },
    {
        title: 'Curso interactivo LearnRust',
        description: <p>Disponible en <a href="https://www.learnrust.io" target="_blank" rel="noopener noreferrer" className="text-rust-orange hover:underline">learnrust.io</a> (en inglés), ofrece lecciones con ejercicios interactivos en el navegador.</p>,
        icon: <GlobeAltIcon className="w-8 h-8 text-teal-400" />
    },
    {
        title: 'Comunidades y Documentación',
        description: <p>El foro oficial y los grupos de Discord/Telegram son excelentes lugares para hacer preguntas. La documentación siempre es tu mejor amiga.</p>,
        icon: <UsersIcon className="w-8 h-8 text-pink-400" />
    },
    {
        title: 'Plugin rust-analyzer',
        description: <p>Si usas VS Code, esta extensión es imprescindible. Proporciona autocompletado, errores en tiempo real y mucho más.</p>,
        icon: <CodeBracketIcon className="w-8 h-8 text-sky-400" />
    }
];

export const interestingSites: SiteLink[] = [
    {
        title: 'Documentación oficial de Rust',
        url: 'https://www.rust-lang.org/learn',
        category: 'Oficial',
        description: <p>El punto de partida central para toda la documentación, incluyendo el libro, Rust by Example y más.</p>,
        icon: <BookOpenIcon className="w-8 h-8 text-purple-400" />
    },
    {
        title: 'Crates.io',
        url: 'https://crates.io/',
        category: 'Oficial',
        description: <p>El registro de paquetes de la comunidad de Rust. Explora las librerías disponibles para tus proyectos.</p>,
        icon: <CodeBracketIcon className="w-8 h-8 text-sky-400" />
    },
    {
        title: 'Exercism - Rust Track',
        url: 'https://exercism.org/tracks/rust',
        category: 'Práctica',
        description: <p>Plataforma con cientos de ejercicios de programación en Rust, con mentoría de la comunidad.</p>,
        icon: <PuzzlePieceIcon className="w-8 h-8 text-yellow-400" />
    },
     {
        title: 'This Week in Rust',
        url: 'https://this-week-in-rust.org/',
        category: 'Noticias',
        description: <p>Un boletín semanal que resume las últimas noticias, artículos y desarrollos en el ecosistema de Rust.</p>,
        icon: <GlobeAltIcon className="w-8 h-8 text-teal-400" />
    },
    {
        title: 'Rust en Español (Discord)',
        url: 'https://discord.gg/kbA2a5gVfB',
        category: 'Comunidad',
        description: <p>La comunidad más grande de Rust en español. Un lugar excelente para hacer preguntas y compartir conocimientos.</p>,
        icon: <UsersIcon className="w-8 h-8 text-pink-400" />
    },
    {
        title: 'Are we web yet?',
        url: 'https://www.arewewebyet.org/',
        category: 'Noticias',
        description: <p>Un resumen del estado del arte del desarrollo web en Rust, incluyendo frameworks, librerías y herramientas.</p>,
        icon: <LinkIcon className="w-8 h-8 text-blue-400" />
    }
];

export const finalThoughts: React.ReactNode = (
    <div className="text-center">
        <h3 className="text-xl font-bold text-slate-100 mb-2">Un último consejo</h3>
        <p className="text-slate-300">Recuerda que <strong>la mejor forma de aprender Rust es con práctica constante y paciencia</strong>. No te frustres si algunos conceptos, especialmente ownership y lifetimes, tardan en "hacer clic". Es normal. Sigue construyendo cosas pequeñas, leyendo código de otros y, sobre todo, ¡disfruta del proceso de aprender un lenguaje tan poderoso y seguro!</p>
    </div>
);