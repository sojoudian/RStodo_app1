use std::collections::HashMap;
use std::io::{self, Write};

struct TodoList {
    tasks: HashMap<u32, String>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) -> u32 {
        let id = self.next_id;
        self.tasks.insert(id, description);
        self.next_id += 1;
        id
    }

    fn remove_task(&mut self, id: u32) -> bool {
        self.tasks.remove(&id).is_some()
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
                let id = todo_list.add_task(description.trim().to_string());
                println!("Task added with ID: {}", id);
            }
            "2" => {
                print!("Enter task ID to remove: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                if let Ok(id) = id_str.trim().parse::<u32>() {
                    if todo_list.remove_task(id) {
                        println!("Task removed successfully!");
                    } else {
                        println!("Task not found!");
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
