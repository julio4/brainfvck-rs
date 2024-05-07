use std::fmt;

pub struct State {
    pub tape: Vec<i32>,
    max_size: usize,
    pub pointer: usize,
}

#[derive(Debug)]
pub enum StateErrors {
    OutOfBounds,
}

pub trait StateTrait {
    fn new(size: usize) -> Self
    where
        Self: Sized;
    fn print(&self);
    fn set_pointer(&mut self, pointer: usize) -> Result<(), StateErrors>;
}

impl StateTrait for State {
    fn new(size: usize) -> Self {
        Self {
            tape: vec![0; size],
            max_size: size,
            pointer: 0,
        }
    }

    fn set_pointer(&mut self, pointer: usize) -> Result<(), StateErrors> {
        if pointer < self.max_size {
            self.pointer = pointer;
            Ok(())
        } else {
            Err(StateErrors::OutOfBounds)
        }
    }

    fn print(&self) {
        print!("{}", self);
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tape = self
            .tape
            .iter()
            .enumerate()
            .map(|(i, &cell)| {
                if i == self.pointer {
                    format!("[{}]", cell)
                } else {
                    format!("{}", cell)
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{}", tape)
    }
}
