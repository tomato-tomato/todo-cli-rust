use anyhow::Result;
use chrono::Local;
use clap::Parser;
use colored::Colorize;

mod cli;
mod storage;
mod todo;

use cli::{Cli, Commands};
use storage::{load_todos, save_todos};
use todo::{Todo, print_todos};

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

fn cmd_list(todos: &[Todo], all: bool, priority: Option<u8>) {
    // let items: Vec<&Todo> = if all {
    //     todos.iter().collect()
    // } else {
    //     todos.iter().filter(|t| !t.completed).collect()
    // };
    let items: Vec<&Todo> = todos
        .iter()
        .filter(|t| all || !t.completed)
        .filter(|t| match priority {
            Some(p) => t.priority > p,
            None => true,
        })
        .collect();

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
            todo.completed_at = if undo { None } else { Some(Local::now()) };
            println!("{}{} #{}: {}", "✔".green(), sign, id, todo.content);
        }
        None => {
            eprint!("Todo #{} not found.", id);
            std::process::exit(1);
        }
    }
}

fn cmd_edit(todos: &mut Vec<Todo>, id: u32, content: &str) {
    match todos.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            todo.content = content.to_string();
            println!("{} #{}: {}", "✔ Edit".yellow(), id, content);
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
        Commands::List { all, priority } => {
            cmd_list(&todos, all, priority);
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
        Commands::Edit { id, content } => {
            cmd_edit(&mut todos, id, &content);
            save_todos(&todos)?;
        }
    }

    Ok(())
}
