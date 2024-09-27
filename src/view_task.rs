// src/view_task.rs

use crate::task::Task;

pub fn ejecutar_programa() {
    let tarea = Task::new(
        1,
        "Gestion en Rust".to_string(),
        "Aprender estructuras y módulos en Rust".to_string(),
        "2024-12-31".to_string(),
    );

    println!("{}", tarea.to_string());
}

/*
Ejercicio: Sistema de Gestión de Tareas Avanzado
Construir un sistema de gestión de tareas que permita crear, editar, eliminar y buscar tareas.

Requisitos:
1. Estructura de datos avanzada:

Usa una estructura como HashMap o BTreeMap para almacenar las tareas, cada tarea estará identificada por un ID único.
Las tareas deberán tener atributos como: Título, Descripción, Fecha de vencimiento (puedes usar cadenas), y Estado (pendiente o completada).

2. Funciones principales:

Añadir una tarea: permitir al usuario añadir una nueva tarea con título, descripción y fecha de vencimiento.
Editar una tarea: Posibilidad de modificar una tarea existente (cambiar título, descripción, etc.).
Eliminar una tarea: Eliminar una tarea existente por su ID.
Buscar tarea: Permitir buscar tareas por título o fecha de vencimiento.
Listar tareas: Mostrar todas las tareas, diferenciando entre las completadas y las pendientes.
Marcar una tarea como completada: Cambiar el estado de la tarea de "pendiente" a "completada".

3. Manejo de errores:

Implementa un buen manejo de errores cuando se intente realizar acciones sobre tareas que no existen, IDs inválidos, o búsquedas sin resultados.
4. Interacción con el usuario:

Implementa un menú donde se puede elegir opciones como:
Crear tarea
Editar tarea
Eliminar tarea
Buscar tarea
Listar todas las tareas
Marcar tarea como completada
Salir del sistema
5. Persistencia de datos (opcional pero desafiante):

Hacer que las tareas se guarden en un archivo de texto (o JSON) y se carguen automáticamente al iniciar el programa, simulando una base de datos simple.



Ejemplo de estructura esperada:
struct Tarea {
    id: u32,
    titulo: String,
    descripcion: String,
    fecha_vencimiento: String,
    estado: Estado,
}

enum Estado {
    Pendiente,
    Completada,
}
Desafíos adicionales:
Iteradores avanzados: Usa iteradores para filtrar tareas por estado o fecha.
Generics y Traits: Si te sientes cómodo, trata de usar generics o traits en alguna parte del código, como para el manejo de persistencia o el sistema de búsqueda.
Concurrency (opcional): Si quieres un reto avanzado, implementa alguna funcionalidad concurrente usando hilos (por ejemplo, para guardar el estado del sistema mientras sigues trabajando).

*/