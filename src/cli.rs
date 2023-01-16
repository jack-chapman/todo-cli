use clap::Parser;
use clap::Subcommand;

use crate::commands::init;
use crate::commands::test_command;

#[derive(Debug, Subcommand)]
pub enum Commands {
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

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "Quick and easy todos for your project", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn run() {
        let args = Cli::parse();

        match args.command {
            Commands::Init => {
                let res = init();
                match res {
                    Err(e) => {
                        eprintln!("Unable to initialise todo-cli project");
                        eprintln!("{}", e);
                        std::process::exit(exitcode::CANTCREAT);
                    }
                    Ok(_) => {
                        println!("todo-cli project created!");
                    }
                }
            }
            Commands::List => {
                test_command("listed todos");
            }
            Commands::Add { text } => {
                let output = format!("Added {text} todo");
                test_command(&output);
            }
            Commands::Delete { todo_id } => {
                let output = format!("Deleted {todo_id} todo");
                test_command(&output);
            }
            Commands::Complete { todo_id } => {
                let output = format!("Completed {todo_id} todo");
                test_command(&output);
            }
            Commands::Uncomplete { todo_id } => {
                let output = format!("Uncompleted {todo_id} todo");
                test_command(&output);
            }
            Commands::Clear => {
                test_command("Cleared all todos");
            }
            Commands::Clean => {
                test_command("Cleaned up todo list");
            }
        }
    }
}
