use std::env;

fn main() {
    // Capturamos los argumentos de la línea de comandos en un Vector de Strings
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("❌ Error: Faltan argumentos.");
        print_help();
        return;
    }

    // El primer argumento (índice 1) es el comando
    let command = &args[1];

    // Usamos pattern matching (match), el equivalente ultra potente al switch
    match command.as_str() {
        "add" => {
            // Verificamos si pasaron el texto de la nota/tarea
            let task_text = match args.get(2) {
                Some(text) => text,
                None => {
                    println!("❌ Error: Debes escribir el contenido de la tarea.");
                    return;
                }
            };
            println!("📝 Agregando tarea: \"{}\"", task_text);
            // Aquí agregaremos la lógica de guardado más adelante
        }
        "list" => {
            println!("📋 Listando todas tus tareas...");
            // Aquí agregaremos la lógica de lectura del archivo más adelante
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