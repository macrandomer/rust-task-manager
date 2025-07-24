use anyhow::{Context, Result};
use chrono::Utc;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

/// JSON file where tasks are stored.
const STORAGE: &str = "tasks.json";

/// Supported CLI commands.
#[derive(Subcommand)]
pub enum TaskCmd {
    /// Add a new task
    Add { description: String },
    /// List all tasks
    List,
    /// Mark a task as done
    Done { id: usize },
    /// Delete a task
    Delete { id: usize },
}

/// Task structure serialized to/from JSON.
#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    created_at: String,
    done: bool,
}

/// Loads all tasks from storage, or returns an empty list if file does not exist.
fn load_tasks() -> Result<Vec<Task>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(STORAGE)
        .context("opening tasks storage")?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_default();
    Ok(tasks)
}

/// Saves all tasks to storage.
fn save_tasks(tasks: &[Task]) -> Result<()> {
    let file = File::create(STORAGE).context("creating tasks storage")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks).context("saving tasks")?;
    Ok(())
}

/// Executes the requested task command and persists changes.
pub fn run(cmd: TaskCmd) -> Result<()> {
    let mut tasks = load_tasks().context("loading tasks from storage")?;
    match cmd {
        TaskCmd::Add { description } => {
            let id = tasks.len() + 1;
            tasks.push(Task {
                id,
                description,
                created_at: Utc::now().to_rfc3339(),
                done: false,
            });
            save_tasks(&tasks).context("saving tasks after Add")?;
            println!("Added task {}", id);
        }
        TaskCmd::List => {
            for t in &tasks {
                println!(
                    "{}. [{}] {} (created {})",
                    t.id,
                    if t.done { "x" } else { " " },
                    t.description,
                    t.created_at
                );
            }
        }
        TaskCmd::Done { id } => {
            if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                t.done = true;
                save_tasks(&tasks).context("saving tasks after Done")?;
                println!("Marked task {} done", id);
            } else {
                println!("Task {} not found", id);
            }
        }
        TaskCmd::Delete { id } => {
            tasks.retain(|t| t.id != id);
            for (i, t) in tasks.iter_mut().enumerate() {
                t.id = i + 1;
            }
            save_tasks(&tasks).context("saving tasks after Delete")?;
            println!("Deleted task {}", id);
        }
    }
    Ok(())
}
