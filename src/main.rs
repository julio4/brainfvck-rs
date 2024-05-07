mod instructions;
mod interpreter;
mod state;
mod tokens;

use std::env;
use std::fs::File;

use state::{State, StateTrait};

use crate::instructions::ParserFromFileTrait;
use crate::interpreter::InterpreterTrait;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Args validation
    if args.len() < 2 {
        println!("Usage: {} <filename> <size=100>", args[0]);
        return;
    }
    let size = if args.len() == 3 {
        args[2].parse().expect("Invalid size")
    } else {
        100
    };

    // Lex and parse the source code
    let file = File::open(&args[1]).expect("File not found");
    let program = file.parse_as_program();

    // Interpret the program
    let mut stdout = std::io::stdout();
    program
        .run(&mut State::new(size), &mut stdout)
        .expect("Error running program");
}

#[cfg(test)]
mod tests;
