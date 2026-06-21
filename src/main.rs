use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,                     // id值
    content: String,             // 待办内容
    completed: bool,             // 是否完成
    created_at: DateTime<Local>, // 创建日期
    #[serde(default)]
    priority: u8, // 待办的重要程度 0-普通，默认颜色；1-高，黄色；2-紧急，红色
}

impl Todo {
    fn new(id: u32, content: String, priority: u8) -> Self {
        // let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        Todo {
            id,
            content,
            completed: false,
            created_at: Local::now(),
            priority,
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
        // The priority of the task (0: 普通, 1:高, 2:紧急)
        #[arg(short, long, default_value_t = 0)]
        priority: u8,
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
        /// undo the task
        #[arg(long)]
        undo: bool,
    },
    /// Remove a todo
    Remove {
        /// The ID of the todo to remove
        id: u32,
    },
    /// Search the task include content
    Search {
        /// The content for search
        keyword: String,
    },
}

fn data_file() -> PathBuf {
    let dir = dirs::config_dir()
        .expect("Cannot determine config directory")
        .join("tasky");
    fs::create_dir_all(&dir).expect("Cannot create config directory");
    dir.join("todos.json")
}
fn load_todos() -> Result<Vec<Todo>> {
    let path = data_file();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path).context("Failed to read todos file")?;
    let todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();
    Ok(todos)
}

fn save_todos(todos: &[Todo]) -> Result<()> {
    let path = data_file();
    let json = serde_json::to_string_pretty(todos).context("Failed to serialize todos")?;
    fs::write(&path, json).context("Failed to write todos file")?;
    Ok(())
}

fn print_todos(label: &str, items: &[&Todo]) {
    if items.is_empty() {
        println!("Nothing here.");
        return;
    }
    println!("  {} ({})", label, items.len());
    for t in items {
        let priority_tag = match t.priority {
            2 => format!("  {}", "!!urgent".red().bold()),
            1 => format!("  {}", "! high".yellow()),
            _ => String::new(),
        };
        if t.completed {
            println!(
                "   {} {}  {} ({}){}",
                format!("[{}]", t.id).dimmed(),
                t.content.strikethrough(),
                "✔ done".green(),
                t.created_at.format("%Y-%m-%d %H:%M"),
                priority_tag,
            );
        } else {
            println!(
                "   [{}] {} ({}){}",
                t.id,
                t.content,
                t.created_at.format("%Y-%m-%d %H:%M"),
                priority_tag,
            );
        }
    }
}

fn cmd_add(todos: &mut Vec<Todo>, content: String, priority: u8) {
    let new_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let todo = Todo::new(new_id, content, priority);
    let priority_info = if priority > 0 {
        format!(
            "   [{}]",
            match priority {
                2 => "urgent".red().bold().to_string(),
                1 => "high".yellow().to_string(),
                _ => "normal".to_string(),
            }
        )
    } else {
        String::new()
    };
    println!(
        "{} Added #{}: {}{}",
        "✔".green(),
        todo.id,
        todo.content,
        priority_info
    );
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
    print_todos(label, &items);
}

fn cmd_done(todos: &mut Vec<Todo>, id: u32, undo: bool) {
    match todos.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            let sign = if undo { "Undo" } else { "Done" };
            todo.completed = !undo;
            println!("{}{} #{}: {}", "✔".green(), sign, id, todo.content);
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

fn cmd_search(todos: &[Todo], keyword: &str) {
    // let get_items: Vec<Todo> = todos
    //     .iter()
    //     .filter(|t| t.content.contains(&keyword))
    //     .cloned()
    //     .collect();
    // cmd_list(&get_items, true);
    let matches: Vec<&Todo> = todos
        .iter()
        .filter(|t| t.content.contains(&keyword))
        .collect();
    let label = format!("Search: \"{}\"", keyword);
    print_todos(&label, &matches);
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut todos = load_todos()?;

    match cli.command {
        Commands::Add { content, priority } => {
            cmd_add(&mut todos, content, priority);
            save_todos(&todos)?;
        }
        Commands::List { all } => {
            cmd_list(&todos, all);
        }
        Commands::Done { id, undo } => {
            cmd_done(&mut todos, id, undo);
            save_todos(&todos)?;
        }
        Commands::Remove { id } => {
            cmd_remove(&mut todos, id);
            save_todos(&todos)?;
        }
        Commands::Search { keyword } => {
            cmd_search(&todos, &keyword);
        }
    }

    Ok(())
}
