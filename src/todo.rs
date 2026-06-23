use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,                     // id值
    pub content: String,             // 待办内容
    pub completed: bool,             // 是否完成
    pub created_at: DateTime<Local>, // 创建日期
    #[serde(default)]
    pub priority: u8, // 待办的重要程度 0-普通，默认颜色；1-高，黄色；2-紧急，红色
    #[serde(default)]
    pub completed_at: Option<DateTime<Local>>,
}
impl Todo {
    pub fn new(id: u32, content: String, priority: u8) -> Self {
        // let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        Todo {
            id,
            content,
            completed: false,
            created_at: Local::now(),
            priority,
            completed_at: None, // 新建时没有完成时间
        }
    }
}

pub fn print_todos(label: &str, items: &[&Todo]) {
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
            let completed_time = match &t.completed_at {
                Some(time) => format!(" ({})", time.format("%Y-%m-%d %H:%M").to_string()),
                None => String::new(),
            };
            println!(
                "   {} {}  {}{} ({}){}",
                format!("[{}]", t.id).dimmed(),
                t.content.strikethrough(),
                "✔ done".green(),
                completed_time.red(),
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
