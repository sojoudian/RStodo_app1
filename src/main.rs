use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const STORAGE_FILE: &str = "todo_list.json";

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: HashMap<u32, String>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        if Path::new(STORAGE_FILE).exists() {
            match TodoList::load_from_file() {
                Ok(todo_list) => return todo_list,
                Err(e) => {
                    eprintln!("Error loading from file: {}. Starting with empty list.", e);
                }
            }
        }
        
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn load_from_file() -> Result<TodoList, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(STORAGE_FILE)?;
        let todo_list: TodoList = serde_json::from_str(&file_content)?;
        Ok(todo_list)
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(STORAGE_FILE, json)?;
        Ok(())
    }

    fn add_task(&mut self, description: String) -> Result<u32, Box<dyn std::error::Error>> {
        let id = self.next_id;
        self.tasks.insert(id, description);
        self.next_id += 1;
        self.save_to_file()?;
        Ok(id)
    }

    fn remove_task(&mut self, id: u32) -> Result<bool, Box<dyn std::error::Error>> {
        let task_removed = self.tasks.remove(&id).is_some();
        if task_removed {
            self.save_to_file()?;
        }
        Ok(task_removed)
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        for (id, task) in &self.tasks {
            println!("{}: {}", id, task);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    
    loop {
        println!("\n=== Todo List Manager ===");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. List tasks");
        println!("4. Exit");
        print!("Choose an option (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                match todo_list.add_task(description.trim().to_string()) {
                    Ok(id) => println!("Task added with ID: {}", id),
                    Err(e) => println!("Error adding task: {}", e),
                }
            }
            "2" => {
                print!("Enter task ID to remove: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                if let Ok(id) = id_str.trim().parse::<u32>() {
                    match todo_list.remove_task(id) {
                        Ok(true) => println!("Task removed successfully!"),
                        Ok(false) => println!("Task not found!"),
                        Err(e) => println!("Error removing task: {}", e),
                    }
                } else {
                    println!("Invalid ID!");
                }
            }
            "3" => {
                todo_list.list_tasks();
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please try again."),
        }
    }
}
