use clap::Parser;
use clap::Subcommand;

use crate::todo::Todo;
use crate::todo::TodoList;

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
        todo_id: usize,
    },
    /// Mark a todo as completed
    #[command(alias = "c")]
    Complete {
        /// the todo to mark as completed
        todo_id: usize,
    },
    /// Mark a todo as uncompleted
    #[command(alias = "u")]
    Uncomplete {
        /// the todo to mark as uncompleted
        todo_id: usize,
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
                let todo_list = TodoList::new();
                let res = todo_list.to_new_file();
                match res {
                    Err(e) => {
                        eprintln!("Unable to initialise todo-cli project");
                        eprintln!("{e}");
                        std::process::exit(exitcode::CANTCREAT);
                    }
                    Ok(_) => {
                        println!("todo-cli project created!");
                    }
                }
            }
            Commands::List => match TodoList::from_file("todo_list.json") {
                Ok(todo_list) => todo_list.list(),
                Err(e) => eprintln!("{e}"),
            },
            Commands::Add { text } => match TodoList::from_file("todo_list.json") {
                Ok(mut todo_list) => match todo_list.add_todo(text) {
                    Ok(_) => println!("added todo"),
                    Err(e) => eprintln!("{e}"),
                },
                Err(e) => eprintln!("{e}"),
            },
            Commands::Delete { todo_id } => match TodoList::from_file("todo_list.json") {
                Ok(mut todo_list) => match todo_list.remove_todo(&todo_id) {
                    Ok(_) => println!("todo removed!"),
                    Err(e) => eprintln!("{e}"),
                },
                Err(e) => eprintln!("{e}"),
            },
            Commands::Complete { todo_id } => match TodoList::from_file("todo_list.json") {
                Ok(mut todo_list) => match todo_list.change_todo_status(&todo_id, true) {
                    Ok(_) => println!("todo completed!"),
                    Err(e) => eprintln!("{e}"),
                },
                Err(e) => eprintln!("{e}"),
            },
            Commands::Uncomplete { todo_id } => match TodoList::from_file("todo_list.json") {
                Ok(mut todo_list) => match todo_list.change_todo_status(&todo_id, false) {
                    Ok(_) => println!("todo uncompleted!"),
                    Err(e) => eprintln!("{e}"),
                },
                Err(e) => eprintln!("{e}"),
            },
            Commands::Clear => {
                todo!("Cleared all todos")
            }
            Commands::Clean => {
                todo!("Cleaned completed todods")
            }
        }
    }
}
