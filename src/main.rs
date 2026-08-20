use std::io::{self, stdout, Write}; 
use crossterm::{execute, terminal::{Clear, ClearType}, style::Stylize};

#[tokio::main]
async fn main() {
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All)).expect("Could not clear terminal");

    println!("This is Dream Studio V1. You are currently in the main menu.

    Here are the commaands you can run:
    - `{}` shows all the worlds you have saved
    - `{}` starts a new world
    - `{}` loads an existing world
    - `{}` loads the latest world
    - `{}` closes Dream Studio
    ",
    "list worlds".blue(),
    "start new".blue(),
    "load <world>".blue(),
    "load last".blue(),
    "exit".blue()
    );

    loop {
        print!("{}", "> ".yellow().bold());

        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get user input");

        let terimmed_input = input.trim();

        println!("Received input {}", terimmed_input.yellow());
    }
}
