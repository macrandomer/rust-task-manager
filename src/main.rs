use clap::{Parser, Subcommand};
use rust_task_manager::commands::{run, TaskCmd};

#[derive(Parser)]
#[command(name="task")]
#[command(about="Manage your tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: TaskCmd,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli.command) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
