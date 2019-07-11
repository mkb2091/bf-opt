#[derive(PartialEq, Debug)]
pub enum Ast {
    MoveRight(usize),
    MoveLeft(usize),
    Increment(u8),
    Decrement(u8),
    Output(Option<u8>),
    Input(Option<isize>),
    WhileLoop(BfProgram),
}

#[derive(PartialEq, Debug)]
pub struct BfProgram {
    pub data: Vec<Ast>,
}

impl BfProgram {
    pub fn new() -> BfProgram {
        BfProgram { data: Vec::new() }
    }
}

impl From<&str> for BfProgram {
    fn from(data: &str) -> BfProgram {
        let mut program = BfProgram::new();
        for x in data.chars() {
            match x {
                '+' => program.data.push(Ast::Increment(1)),
                '-' => program.data.push(Ast::Decrement(1)),
                _ => {}
            }
        }
        program
    }
}
