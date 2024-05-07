use crate::instructions::{Instruction, Program};
use crate::state::{State, StateTrait};
use std::io::{self, Read};

pub trait InterpreterTrait {
    fn run<W>(&self, state: &mut State, writer: &mut W) -> io::Result<()>
    where
        W: io::Write;
}

impl InterpreterTrait for Program {
    fn run<W>(&self, state: &mut State, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        for instruction in self {
            match instruction {
                Instruction::MoveRight => {
                    state.set_pointer(state.pointer + 1).expect("Out of bounds");
                }
                Instruction::MoveLeft => {
                    if state.pointer == 0 {
                        panic!("Out of bounds");
                    }
                    state.set_pointer(state.pointer - 1).unwrap();
                }
                Instruction::Increment => {
                    state.tape[state.pointer] += 1;
                }
                Instruction::Decrement => {
                    state.tape[state.pointer] -= 1;
                }
                Instruction::Print => {
                    write!(writer, "{}", state.tape[state.pointer] as u8 as char)?;
                }
                Instruction::Read => {
                    let mut buffer = [0; 1];
                    std::io::stdin()
                        .read_exact(&mut buffer)
                        .expect("Error reading input");
                    state.tape[state.pointer] = buffer[0] as i32;
                }
                Instruction::Loop(program) => {
                    while state.tape[state.pointer] != 0 {
                        program.run(state, writer)?;
                    }
                }
            }
        }
        Ok(())
    }
}
