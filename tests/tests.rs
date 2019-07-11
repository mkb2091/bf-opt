#[cfg(test)]
mod tests {
    #[test]
    fn increment_to_ast() {
        assert_eq!(
            bf_opt::BfProgram::from("+"),
            bf_opt::BfProgram {
                data: vec![bf_opt::Ast::Increment(1)]
            }
        )
    }
    #[test]
    fn decrement_to_ast() {
        assert_eq!(
            bf_opt::BfProgram::from("-"),
            bf_opt::BfProgram {
                data: vec![bf_opt::Ast::Decrement(1)]
            }
        )
    }
    #[test]
    fn move_right_to_ast() {
        assert_eq!(
            bf_opt::BfProgram::from(">"),
            bf_opt::BfProgram {
                data: vec![bf_opt::Ast::MoveRight(1)]
            }
        )
    }
    #[test]
    fn move_left_to_ast() {
        assert_eq!(
            bf_opt::BfProgram::from("<"),
            bf_opt::BfProgram {
                data: vec![bf_opt::Ast::MoveLeft(1)]
            }
        )
    }
    #[test]
    fn basic_cat() {
        assert_eq!(
            bf_opt::BfProgram::from(",[.,]"),
            bf_opt::BfProgram {
                data: vec![
                    bf_opt::Ast::Input(None),
                    bf_opt::Ast::WhileLoop(bf_opt::BfProgram {
                        data: vec![bf_opt::Ast::Output(None), bf_opt::Ast::Input(None)]
                    })
                ]
            }
        )
    }
}
