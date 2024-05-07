pub type Program = Vec<Instruction>;

#[derive(Debug)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Read,
    Loop(Program),
}

pub trait ParserTrait {
    fn parse(&self) -> Program;
}

pub trait ParserFromFileTrait {
    fn parse_as_program(&self) -> Program;
}

use crate::tokens::LexerTrait;

use super::tokens::Token;
impl ParserTrait for Vec<Token> {
    fn parse(&self) -> Program {
        let mut program = Vec::new();
        let mut loop_stack = Vec::new();
        let mut loop_start = 0;

        self.into_iter()
            .enumerate()
            .for_each(|(i, token)| match token {
                Token::MoveRight => program.push(Instruction::MoveRight),
                Token::MoveLeft => program.push(Instruction::MoveLeft),
                Token::Increment => program.push(Instruction::Increment),
                Token::Decrement => program.push(Instruction::Decrement),
                Token::Print => program.push(Instruction::Print),
                Token::Read => program.push(Instruction::Read),
                Token::LoopStart => {
                    loop_stack.push(i);
                    loop_start = i;
                }
                Token::LoopEnd => {
                    let start = loop_stack.pop().expect("Unmatched loop end");
                    let loop_program = self[start + 1..i].to_vec().parse();
                    program.push(Instruction::Loop(loop_program));
                }
            });
        program
    }
}

impl ParserFromFileTrait for std::fs::File {
    fn parse_as_program(&self) -> Program {
        let source = self.lex();
        source.parse()
    }
}
