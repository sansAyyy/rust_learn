use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "A tiny JSON-backed todo CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add { text: String },
    List,
    Done { id: u64 },
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    id: u64,
    text: String,
    done: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = data_file();
    let mut items = load_items(&path)?;

    match cli.command {
        Commands::Add { text } => {
            let next_id = items.iter().map(|item| item.id).max().unwrap_or(0) + 1;
            items.push(TodoItem {
                id: next_id,
                text,
                done: false,
            });
            save_items(&path, &items)?;
            println!("added todo #{next_id}");
        }
        Commands::List => {
            if items.is_empty() {
                println!("no todos yet");
            }
            for item in items {
                let mark = if item.done { "x" } else { " " };
                println!("[{mark}] #{} {}", item.id, item.text);
            }
        }
        Commands::Done { id } => {
            let Some(item) = items.iter_mut().find(|item| item.id == id) else {
                anyhow::bail!("todo #{id} does not exist");
            };
            item.done = true;
            save_items(&path, &items)?;
            println!("marked todo #{id} as done");
        }
    }

    Ok(())
}

fn data_file() -> PathBuf {
    std::env::temp_dir().join("rust_learn_todos.json")
}

fn load_items(path: &PathBuf) -> Result<Vec<TodoItem>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content =
        fs::read_to_string(path).with_context(|| format!("read todo file {}", path.display()))?;
    let items = serde_json::from_str(&content)
        .with_context(|| format!("parse todo file {}", path.display()))?;
    Ok(items)
}

fn save_items(path: &PathBuf, items: &[TodoItem]) -> Result<()> {
    let content = serde_json::to_string_pretty(items)?;
    fs::write(path, content).with_context(|| format!("write todo file {}", path.display()))
}
