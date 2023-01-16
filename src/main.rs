use cli::Cli;

mod cli;
mod commands;
mod todo;

fn main() {
    Cli::run();
}
