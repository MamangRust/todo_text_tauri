// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod logging;

#[tauri::command]
fn fetch_tasks() -> Result<Vec<String>, String> {
    let content = match std::fs::read_to_string("tasks.txt") {
        Ok(data) => data,
        Err(_) => return Ok(Vec::new()),
    };
    let tasks: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    Ok(tasks)
}

#[tauri::command]
fn add_task(task: String) -> Result<(), String> {
    use std::fs;

    let formatted_task = format!("{}\n", task);

    let contents = match fs::read_to_string("tasks.txt") {
        Ok(contents) => contents,
        Err(error) => {
            log::error!("Error reading file: {}", error);
            return Err(format!("Failed to read file: {}", error));
        }
    };

    if contents.contains(&formatted_task) {
        return Ok(()); // Jika teks sudah ada, kembalikan Ok tanpa menambahkan
    }

    if let Err(error) = fs::write("tasks.txt", contents + &formatted_task) {
        log::error!("Error writing to file: {}", error);
        return Err(format!("Failed to add task: {}", error));
    }

    Ok(())
}
#[tauri::command]
fn update_task(index: usize, updated_task: String) -> Result<(), String> {
    let mut tasks = match std::fs::read_to_string("tasks.txt") {
        Ok(data) => data.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
        Err(_) => return Err("Failed to read tasks.".to_string()),
    };

    if let Some(task) = tasks.get_mut(index) {
        *task = updated_task;
    } else {
        return Err("Task index out of bounds.".to_string());
    }

    if let Err(_) = std::fs::write("tasks.txt", tasks.join("\n")) {
        return Err("Failed to update task.".to_string());
    }

    Ok(())
}

#[tauri::command]
fn delete_task(index: usize) -> Result<(), String> {
    let mut tasks = match std::fs::read_to_string("tasks.txt") {
        Ok(data) => data.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
        Err(_) => return Err("Failed to read tasks.".to_string()),
    };

    if index >= tasks.len() {
        return Err("Task index out of bounds.".to_string());
    }

    tasks.remove(index);

    if let Err(_) = std::fs::write("tasks.txt", tasks.join("\n")) {
        return Err("Failed to delete task.".to_string());
    }

    Ok(())
}

fn main() {
    logging::setup_logger().expect("Could not set up loggers.");
    log::info!("Launching app...");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_tasks,
            add_task,
            update_task,
            delete_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
