use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,
    content: String,
    completed: bool,
    created_at: String,
}

impl Todo {
    fn new(id: u32, content: String) -> Self {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        Todo {
            id,
            content,
            completed: false,
            created_at: now,
        }
    }
}

#[derive(Parser)]
#[command(name = "tasky", version, about = "A tiny todo manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add {
        /// The task description
        #[arg(required = true)]
        content: String,
    },
    /// List todos(pending by default)
    List {
        /// show all todos including completed
        #[arg(short, long)]
        all: bool,
    },
    /// Mark a todo as done
    Done {
        /// the Id of the todo to complete
        id: u32,
    },
    /// Remove a todo
    Remove {
        /// The ID of the todo to remove
        id: u32,
    },
}

fn data_file() -> PathBuf {
    let dir = dirs::config_dir()
        .expect("Cannot determine config directory")
        .join("tasky");
    fs::create_dir_all(&dir).expect("Cannot create config directory");
    dir.join("todos.json")
}
fn load_todos() -> Vec<Todo> {
    let path = data_file();
    if !path.exists() {
        return Vec::new();
    }
    let content = fs::read_to_string(&path).expect("Failed to read todos file");
    serde_json::from_str(&content).unwrap_or_default()
}

fn save_todos(todos: &[Todo]) {
    let path = data_file();
    let json = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");
    fs::write(&path, json).expect("Failed to write todos file");
}

fn cmd_add(todos: &mut Vec<Todo>, content: String) {
    let new_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let todo = Todo::new(new_id, content);
    println!("\x1b[32m✓\x1b[0m Added #{}: {}", todo.id, todo.content);
    todos.push(todo);
}

fn cmd_list(todos: &[Todo], all: bool) {
    let items: Vec<&Todo> = if all {
        todos.iter().collect()
    } else {
        todos.iter().filter(|t| !t.completed).collect()
    };

    if items.is_empty() {
        println!("Nothing here. Use `tasky add <text>` to create one.");
        return;
    }

    let label = if all { "All" } else { "Pending" };
    println!("  {} ({}):", label, items.len());
    for t in &items {
        if t.completed {
            println!(
                "   \x1b[9m[{}] {}\x1b[0m  \x1b[32m✓ done\x1b[0m  ({})",
                t.id, t.content, t.created_at
            );
        } else {
            println!("  [{}] {} ({})", t.id, t.content, t.created_at);
        }
    }
}

fn cmd_done(todos: &mut Vec<Todo>, id: u32) {
    match todos.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            todo.completed = true;
            println!("\x1b[32m✓\x1b[0m Done #{}: {}", id, todo.content);
        }
        None => {
            eprint!("Todo #{} not found.", id);
            std::process::exit(1);
        }
    }
}

fn cmd_remove(todos: &mut Vec<Todo>, id: u32) {
    let len_before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < len_before {
        println!("Removed #{}", id);
    } else {
        eprintln!("Todo #{} not found.", id);
        std::process::exit(1);
    }
}

fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    match cli.command {
        Commands::Add { content } => {
            cmd_add(&mut todos, content);
            save_todos(&todos);
        }
        Commands::List { all } => {
            cmd_list(&todos, all);
        }
        Commands::Done { id } => {
            cmd_done(&mut todos, id);
            save_todos(&todos);
        }
        Commands::Remove { id } => {
            cmd_remove(&mut todos, id);
            save_todos(&todos);
        }
    }
}
