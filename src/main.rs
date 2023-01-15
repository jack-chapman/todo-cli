use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "Quick and easy todos for your project", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialises new todo-cli project
    #[command(alias = "i")]
    Init,
    /// List current todos
    #[command(alias = "ls")]
    List,
    /// Add a todo to your project
    #[command(alias = "a")]
    Add {
        /// The description of your todo item
        text: String,
    },
    /// Delete a todo from your project
    #[command(alias = "d")]
    Delete {
        /// the todo to delete
        todo_id: u8,
    },
    /// Mark a todo as completed
    #[command(alias = "c")]
    Complete {
        /// the todo to mark as completed
        todo_id: u8,
    },
    /// Mark a todo as uncompleted
    #[command(alias = "u")]
    Uncomplete {
        /// the todo to mark as uncompleted
        todo_id: u8,
    },
    /// Clear all todos in the project
    Clear,
    /// Clean up todo list
    Clean,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init => {
            println!("initialised todo project");
        }
        Commands::List => {
            println!("listed todos");
        }
        Commands::Add { text } => {
            println!("Added {text} todo");
        }
        Commands::Delete { todo_id } => {
            println!("Deleted {todo_id} todo");
        }
        Commands::Complete { todo_id } => {
            println!("Completed {todo_id} todo");
        }
        Commands::Uncomplete { todo_id } => {
            println!("Uncompleted {todo_id} todo");
        }
        Commands::Clear => {
            println!("Cleared all todos");
        }
        Commands::Clean => {
            println!("Cleaned up todo list");
        }
    }
}
