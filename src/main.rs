use clap::Parser;
use rust_task_manager::commands::{TaskCmd, run};

#[derive(Parser)]
#[command(
    name = "task",
    version = "0.10",
    author = "Mohammed Abdul Aziz <mohdabdul532@gmail.com>",
    about = "A command-line task manager in Rust",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: TaskCmd,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli.command) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
