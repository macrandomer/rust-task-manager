use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use clap::Subcommand;
use anyhow::{Result, Context};

const STORAGE: &str = "tasks.json";

#[derive(Subcommand)]
pub enum TaskCmd {
    Add { description: String },
    List,
    Done { id: usize },
    Delete { id: usize },
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    created_at: String,
    done: bool,
}

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

fn save_tasks(tasks: &[Task]) -> Result<()> {
    let file = File::create(STORAGE)
        .context("creating tasks storage")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)
        .context("saving tasks")?;
    Ok(())
}

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
