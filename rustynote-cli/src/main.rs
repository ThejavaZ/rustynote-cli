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

            let mut tasks = match storage::load_tasks() {
                Ok(t) => t,
                Err(e) => {
                    println!("❌ Error al cargar las tareas: {}", e);
                    return;
                }
            };

            let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

            let new_task = Task {
                id: new_id,
                text: task_text.to_string(),
                status: TaskStatus::Pending,
            };

            tasks.push(new_task.clone());

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
        "done" => {
            let id_str = match args.get(2) {
                Some(id) => id,
                None => {
                    println!("❌ Error: Debes especificar el ID de la tarea a completar.");
                    return;
                }
            };

            // Parseamos el argumento de texto a u32 de forma segura con match
            let id: u32 = match id_str.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("❌ Error: El ID debe ser un número válido.");
                    return;
                }
            };

            let mut tasks = match storage::load_tasks() {
                Ok(t) => t,
                Err(e) => {
                    println!("❌ Error al cargar las tareas: {}", e);
                    return;
                }
            };

            let mut found = false;
            for task in &mut tasks {
                if task.id == id {
                    task.status = TaskStatus::Completed;
                    found = true;
                    break;
                }
            }

            if !found {
                println!("❌ Error: No se encontró ninguna tarea con el ID {}", id);
                return;
            }

            if let Err(e) = storage::save_tasks(&tasks) {
                println!("❌ Error al guardar los cambios: {}", e);
                return;
            }

            println!("✨ Tarea [{}] marcada como completada.", id);
        }
        "delete" => {
            let id_str = match args.get(2) {
                Some(id) => id,
                None => {
                    println!("❌ Error: Debes especificar el ID de la tarea a eliminar.");
                    return;
                }
            };

            let id: u32 = match id_str.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("❌ Error: El ID debe ser un número válido.");
                    return;
                }
            };

            let mut tasks = match storage::load_tasks() {
                Ok(t) => t,
                Err(e) => {
                    println!("❌ Error al cargar las tareas: {}", e);
                    return;
                }
            };

            let initial_len = tasks.len();
            
            // retained: nos quedamos solo con las tareas que NO coincidan con el ID a eliminar
            tasks.retain(|t| t.id != id);

            if tasks.len() == initial_len {
                println!("❌ Error: No se encontró ninguna tarea con el ID {}", id);
                return;
            }

            if let Err(e) = storage::save_tasks(&tasks) {
                println!("❌ Error al guardar los cambios: {}", e);
                return;
            }

            println!("🗑️ Tarea [{}] eliminada con éxito.", id);
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
    println!("  rustynote-cli add <text>    - Create a new task");
    println!("  rustynote-cli list          - Show all tasks");
    println!("  rustynote-cli done <id>     - Mark a task as completed");
    println!("  rustynote-cli delete <id>   - Delete a task by ID");
    println!("  rustynote-cli help          - Show this help message\n");
}