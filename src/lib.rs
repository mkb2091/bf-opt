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
        let mut loop_data: Vec<char> = Vec::new();
        let mut in_loop: bool = false;
        let mut unclosed = 0;
        for x in data.chars() {
            if in_loop {
                if x == ']' {
                    unclosed -= 1;
                    if unclosed == 0 {
                        let loop_data_string: String = loop_data.into_iter().collect();
                        program
                            .data
                            .push(Ast::WhileLoop(BfProgram::from(loop_data_string.as_ref())));
                        loop_data = Vec::new();
                        in_loop = false;
                    }
                } else {
                    loop_data.push(x);
                }
            } else {
                match x {
                    '+' => program.data.push(Ast::Increment(1)),
                    '-' => program.data.push(Ast::Decrement(1)),
                    '>' => program.data.push(Ast::MoveRight(1)),
                    '<' => program.data.push(Ast::MoveLeft(1)),
                    '.' => program.data.push(Ast::Output(None)),
                    ',' => program.data.push(Ast::Input(None)),
                    '[' => {
                        in_loop = true;
                        unclosed = 1;
                    }
                    _ => {}
                }
            }
        }
        program
    }
}
