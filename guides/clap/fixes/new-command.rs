use clap::{Parser, Subcommand};

/// A tiny to-do list
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Print more detail (allowed before or after the subcommand)
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add a task
    Add {
        /// What to do
        text: String,
        /// Priority from 1 (high) to 3 (low)
        #[arg(short, long, default_value_t = 2)]
        priority: u8,
    },
    /// List tasks
    List,
    /// Mark a task as done
    Done {
        /// The task number
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Add { text, priority } => {
            println!("added {text:?} with priority {priority}");
            if cli.verbose {
                println!("(verbose mode is on)");
            }
        }
        Command::List => println!("nothing to list yet"),
    }
}
