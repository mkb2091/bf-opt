pub mod interpreter;

#[derive(PartialEq, Debug)]
pub enum RealOps {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Input,
    Output,
    WhileLoop(BfProgram),
}

impl RealOps {
    fn to_string(&self) -> String {
        match self {
            RealOps::Increment => "+".to_string(),
            RealOps::Decrement => "-".to_string(),
            RealOps::MoveRight => ">".to_string(),
            RealOps::MoveLeft => "<".to_string(),
            RealOps::Input => ",".to_string(),
            RealOps::Output => ".".to_string(),
            RealOps::WhileLoop(x) => format!("[{:}]", x.to_string()),
        }
    }
    fn get_output_length(&self) -> usize {
        match self {
            RealOps::WhileLoop(x) => x.get_output_length() + 2,
            _ => 1,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct BfProgram {
    pub ast: Vec<RealOps>,
}

impl BfProgram {
    pub fn new() -> BfProgram {
        BfProgram { ast: Vec::new() }
    }
    pub fn to_string(&self) -> String {
        let mut result: String = String::new();
        for op in self.ast.iter() {
            result.push_str(&op.to_string());
        }
        result
    }
    pub fn get_output_length(&self) -> usize {
        let mut length = 0;
        for op in self.ast.iter() {
            length += op.get_output_length();
        }
        length
    }
    pub fn optimise(&mut self) {
        opt::remove_redundant(self);
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
                        program.ast.push(RealOps::WhileLoop(BfProgram::from(
                            loop_data_string.as_ref(),
                        )));
                        loop_data = Vec::new();
                        in_loop = false;
                    } else {
                        loop_data.push(x);
                    }
                } else {
                    if x == '[' {
                        unclosed += 1;
                    }
                    loop_data.push(x);
                }
            } else {
                match x {
                    '+' => program.ast.push(RealOps::Increment),
                    '-' => program.ast.push(RealOps::Decrement),
                    '>' => program.ast.push(RealOps::MoveRight),
                    '<' => program.ast.push(RealOps::MoveLeft),
                    '.' => program.ast.push(RealOps::Output),
                    ',' => program.ast.push(RealOps::Input),
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
