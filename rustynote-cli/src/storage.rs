use std::fs::{self, File};
use std::io::{self, Write};
use crate::models::Task;

const DB_FILE: &str = "notes.json";

/// Carga las tareas desde notes.json. Si el archivo no existe, devuelve un vector vacío.
pub fn load_tasks() -> io::Result<Vec<Task>> {
    // Si el archivo no existe aún, retornamos una lista vacía sin error
    if !std::path::Path::new(DB_FILE).exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(DB_FILE)?;
    
    // Deserializamos el JSON a un Vector de Tasks. Si hay error de formato, devolvemos vacío.
    let tasks: Vec<Task> = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
    
    Ok(tasks)
}

/// Guarda el listado completo de tareas en notes.json con formato legible.
pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let data = serde_json::to_string_pretty(tasks)?;
    let mut file = File::create(DB_FILE)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}