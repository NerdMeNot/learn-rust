use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use todo::{List, load, save};

/// A to-do list kept in a JSON file
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Where the list is stored
    #[arg(long, global = true, default_value = "todo.json")]
    file: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add a task
    Add {
        /// What to do
        text: String,
        /// 1 (high) to 3 (low)
        #[arg(short, long, default_value_t = 2)]
        priority: u8,
    },
    /// Show the tasks
    List {
        /// Hide finished tasks
        #[arg(long)]
        open: bool,
    },
    /// Mark a task as done
    Done { id: u32 },
    /// Delete a task
    Remove { id: u32 },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut list: List = load(&cli.file)?;
    match cli.command {
        Command::Add { text, priority } => {
            println!("added {}", list.add(&text, priority)?);
            save(&list, &cli.file)?;
        }
        Command::List { open } => {
            let shown: Vec<_> = list.tasks().iter().filter(|t| !open || !t.done).collect();
            if shown.is_empty() {
                println!("no tasks");
            }
            for task in shown {
                println!("{task}");
            }
        }
        Command::Done { id } => {
            println!("finished {}", list.complete(id)?);
            save(&list, &cli.file)?;
        }
        Command::Remove { id } => {
            println!("removed {}", list.remove(id)?);
            save(&list, &cli.file)?;
        }
    }
    Ok(())
}
