use std::fs::File;
use std::io::{BufReader, Read};

pub type SourceCode = Vec<Token>;

#[derive(Debug, Clone)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Read,
    LoopStart,
    LoopEnd,
}

pub trait LexerTrait {
    fn lex(&self) -> SourceCode;
}

impl LexerTrait for File {
    fn lex(&self) -> SourceCode {
        let mut tokens = Vec::new();

        let reader = BufReader::new(self);
        for result in reader.bytes() {
            let c = result.expect("Error reading file") as char;
            match c {
                '>' => tokens.push(Token::MoveRight),
                '<' => tokens.push(Token::MoveLeft),
                '+' => tokens.push(Token::Increment),
                '-' => tokens.push(Token::Decrement),
                '.' => tokens.push(Token::Print),
                ',' => tokens.push(Token::Read),
                '[' => tokens.push(Token::LoopStart),
                ']' => tokens.push(Token::LoopEnd),
                _ => (),
            }
        }
        tokens
    }
}
