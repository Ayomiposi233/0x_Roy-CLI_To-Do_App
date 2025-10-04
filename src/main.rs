use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;

// Define the Todo struct
#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

// Function to load todos from file
fn load_todos() -> Vec<Todo> {
    let path = Path::new("todos.json");
    if path.exists() {
        let file = File::open(path).expect("Failed to open file");
        let todos: Vec<Todo> = serde_json::from_reader(file).unwrap_or_default();
        todos
    } else {
        Vec::new()
    }
}

// Function to save todos to file
fn save_todos(todos: &Vec<Todo>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("todos.json")
        .expect("Failed to open file");
    serde_json::to_writer_pretty(file, todos).expect("Failed to write to file");
}

// Function to list todos
fn list_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No todos found.");
        return;
    }
    println!("Todos:");
    for todo in todos {
        let status = if todo.completed { "✔" } else { "❌" };
        println!("ID: {} | {} | Title: {}", todo.id, status, todo.title);
    }
}

// Function to add a todo
fn add_todo(todos: &mut Vec<Todo>, title: String) {
    let id = todos.len() as u32 + 1;
    let todo = Todo { id, title: title.clone(), completed: false };
    todos.push(todo);
    println!("Added todo: {}", title);
}

// Function to edit a todo
fn edit_todo(todos: &mut Vec<Todo>, id: u32, new_title: String) {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.title = new_title;
        println!("Edited todo ID {} to: {}", id, todo.title);
    } else {
        println!("Todo with ID {} not found.", id);
    }
}

// Function to complete a todo
fn complete_todo(todos: &mut Vec<Todo>, id: u32) {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = true;
        println!("Completed todo ID {}: {}", id, todo.title);
    } else {
        println!("Todo with ID {} not found.", id);
    }
}

// Function to delete a todo
fn delete_todo(todos: &mut Vec<Todo>, id: u32) {
    if let Some(pos) = todos.iter().position(|t| t.id == id) {
        let removed = todos.remove(pos);
        // Re-assign IDs to maintain sequential order (optional but nice)
        for (i, todo) in todos.iter_mut().enumerate() {
            todo.id = (i + 1) as u32;
        }
        println!("Deleted todo ID {}: {}", id, removed.title);
    } else {
        println!("Todo with ID {} not found.", id);
    }
}

fn main() {
    let mut todos = load_todos();

    loop {
        println!("\nTodo CLI App");
        println!("1. List todos");
        println!("2. Add todo");
        println!("3. Edit todo");
        println!("4. Complete todo");
        println!("5. Delete todo");
        println!("6. Exit");

        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            }
        };

        match choice {
            1 => list_todos(&todos),
            2 => {
                print!("Enter todo title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                add_todo(&mut todos, title.trim().to_string());
            }
            3 => {
                print!("Enter todo ID to edit: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: u32 = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };
                print!("Enter new title: ");
                io::stdout().flush().unwrap();
                let mut new_title = String::new();
                io::stdin().read_line(&mut new_title).unwrap();
                edit_todo(&mut todos, id, new_title.trim().to_string());
            }
            4 => {
                print!("Enter todo ID to complete: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: u32 = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };
                complete_todo(&mut todos, id);
            }
            5 => {
                print!("Enter todo ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: u32 = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };
                delete_todo(&mut todos, id);
            }
            6 => {
                save_todos(&todos);
                println!("Exiting. Todos saved.");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }

        // Save after every operation (for simplicity)
        save_todos(&todos);
    }
}