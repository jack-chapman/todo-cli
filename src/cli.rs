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
                Ok(todo_list) => {
                    if todo_list.todos.is_empty() {
                        println!("No items in todo list");
                        println!("Use `add` command to add some!");
                    }
                    for (i, todo) in todo_list.todos.iter() {
                        let complete = if todo.complete { "x" } else { " " };
                        println!("{} [{}]: {}", i, complete, todo.description);
                    }
                }
                Err(e) => eprintln!("{e}"),
            },
            Commands::Add { text } => {
                let todo = Todo::new(text, None);
                match TodoList::from_file("todo_list.json") {
                    Ok(mut todo_list) => {
                        todo_list.add(todo);
                        match todo_list.to_file() {
                            Ok(_) => println!("added new item!"),
                            Err(e) => eprintln!("{e}"),
                        }
                    }
                    Err(e) => eprintln!("{e}"),
                }
            }
            Commands::Delete { todo_id } => match TodoList::from_file("todo_list.json") {
                Ok(mut todo_list) => match todo_list.todos.remove(&todo_id) {
                    Some(_) => {
                        if let Err(e) = todo_list.to_file() {
                            eprintln!("{e}");
                        } else {
                            println!("deleted todo with id [{todo_id}]");
                        }
                    }
                    None => eprintln!("No todo with id [{todo_id}] found"),
                },
                Err(e) => eprintln!("{e}"),
            },
            Commands::Complete { todo_id } => {
                let output = format!("Completed {todo_id} todo");
                todo!("{output}")
            }
            Commands::Uncomplete { todo_id } => {
                let output = format!("Uncompleted {todo_id} todo");
                todo!("{output}")
            }
            Commands::Clear => {
                todo!("Cleared all todos")
            }
            Commands::Clean => {
                todo!("Cleaned completed todods")
            }
        }
    }
}
