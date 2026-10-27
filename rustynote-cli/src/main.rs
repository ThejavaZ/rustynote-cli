mod models;
mod storage;

use std::env;
use models::{Task, TaskStatus};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("❌ Error: Faltan argumentos.");
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            let task_text = match args.get(2) {
                Some(text) => text,
                None => {
                    println!("❌ Error: Debes escribir el contenido de la tarea.");
                    return;
                }
            };

            // 1. Cargamos las tareas actuales
            let mut tasks = match storage::load_tasks() {
                Ok(t) => t,
                Err(e) => {
                    println!("❌ Error al cargar las tareas: {}", e);
                    return;
                }
            };

            // 2. Calculamos el nuevo ID (si hay tareas, el último ID + 1, si no, 1)
            let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

            // 3. Creamos la nueva tarea
            let new_task = Task {
                id: new_id,
                text: task_text.to_string(),
                status: TaskStatus::Pending,
            };

            tasks.push(new_task.clone());

            // 4. Guardamos en el archivo JSON
            if let Err(e) = storage::save_tasks(&tasks) {
                println!("❌ Error al guardar la tarea: {}", e);
                return;
            }

            println!("✅ Tarea agregada con éxito: [{}] \"{}\"", new_task.id, new_task.text);
        }
        "list" => {
            let tasks = match storage::load_tasks() {
                Ok(t) => t,
                Err(e) => {
                    println!("❌ Error al leer las tareas: {}", e);
                    return;
                }
            };

            if tasks.is_empty() {
                println!("📭 No hay tareas registradas todavía.");
                return;
            }

            println!("\n📋 Tus tareas guardadas:");
            for task in tasks {
                println!("  [{}] {} - {}", task.id, task.text, task.status);
            }
            println!();
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            println!("❌ Comando desconocido: '{}'", command);
            print_help();
        }
    }
}

fn print_help() {
    println!("\nUsage:");
    println!("  rustynote-cli add <text>   - Create a new task");
    println!("  rustynote-cli list         - Show all tasks");
    println!("  rustynote-cli help         - Show this help message\n");
}