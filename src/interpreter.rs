use crate::{BfProgram, RealOps};
use std::convert::TryInto;

#[derive(PartialEq)]
pub struct EndlessVec<T: std::marker::Copy> {
    positive_data: Vec<T>,
    negative_data: Vec<T>,
    default_value: T,
}

impl<T: std::marker::Copy> EndlessVec<T> {
    pub fn new(default_value: T) -> EndlessVec<T> {
        EndlessVec {
            positive_data: Vec::new(),
            negative_data: Vec::new(),
            default_value,
        }
    }
    pub fn get(&self, index: isize) -> T {
        if index >= 0 {
            if self.positive_data.len() <= (index as usize) {
                self.default_value
            } else {
                self.positive_data[(index as usize)]
            }
        } else {
            let pos: usize = (1_isize - index).try_into().unwrap();
            if self.negative_data.len() <= pos {
                self.default_value
            } else {
                self.negative_data[pos]
            }
        }
    }
    pub fn set(&mut self, index: isize, value: T) {
        if index >= 0 {
            if self.positive_data.len() <= (index as usize) {
                for _ in self.positive_data.len()..(index as usize) {
                    self.positive_data.push(self.default_value);
                }
                self.positive_data.push(value);
            } else {
                self.positive_data[(index as usize)] = value;
            }
        } else {
            let pos: usize = (1_isize - index).try_into().unwrap();
            if self.negative_data.len() <= pos {
                for _ in self.negative_data.len()..pos {
                    self.negative_data.push(self.default_value);
                }
                self.negative_data.push(value);
            } else {
                self.negative_data[pos] = value;
            }
        }
    }
}

#[derive(PartialEq)]
pub struct Instance {
    data: EndlessVec<u8>,
    pub output: Vec<u8>,
    pos: isize,
    input_pos: usize,
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            data: EndlessVec::new(0),
            output: Vec::new(),
            pos: 0,
            input_pos: 0,
        }
    }
    pub fn run(&mut self, program: &crate::BfProgram, input: &[u8], ops_to_run: usize) -> usize {
        let mut ops_run = 0;
        for op in program.ast.iter() {
            ops_run += 1;
            if ops_run < ops_to_run {
                match op {
                    RealOps::Increment => {
                        self.data.set(self.pos, self.data.get(self.pos) + 1);
                    }
                    RealOps::Decrement => {
                        self.data.set(self.pos, self.data.get(self.pos) - 1);
                    }
                    RealOps::MoveRight => self.pos += 1,
                    RealOps::MoveLeft => self.pos -= 1,
                    RealOps::Output => {
                        self.output.push(self.data.get(self.pos));
                    }
                    RealOps::Input => {
                        let new_data = if input.len() > self.input_pos {
                            input[self.input_pos]
                        } else {
                            0
                        };
                        self.input_pos += 1;

                        self.data.set(self.pos, new_data);
                    }
                    RealOps::WhileLoop(body) => {
                        while self.data.get(self.pos) > 0 {
                            ops_run += self.run(body, input, ops_to_run - ops_run);
                        }
                    }
                }
            } else {
                break;
            }
        }
        ops_run
    }
}
