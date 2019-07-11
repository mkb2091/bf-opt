#[cfg(test)]
mod tests {
    #[test]
    fn increment_to_ast() {
        assert_eq!(
            bf_opt::BfProgram::from("+"),
            bf_opt::BfProgram {
                ast: vec![bf_opt::RealOps::Increment]
            }
        )
    }
    #[test]
    fn decrement_to_ast() {
        assert_eq!(
            bf_opt::BfProgram::from("-"),
            bf_opt::BfProgram {
                ast: vec![bf_opt::RealOps::Decrement]
            }
        )
    }
    #[test]
    fn move_right_to_ast() {
        assert_eq!(
            bf_opt::BfProgram::from(">"),
            bf_opt::BfProgram {
                ast: vec![bf_opt::RealOps::MoveRight]
            }
        )
    }
    #[test]
    fn move_left_to_ast() {
        assert_eq!(
            bf_opt::BfProgram::from("<"),
            bf_opt::BfProgram {
                ast: vec![bf_opt::RealOps::MoveLeft]
            }
        )
    }
    #[test]
    fn basic_cat() {
        assert_eq!(
            bf_opt::BfProgram::from(",[.,]"),
            bf_opt::BfProgram {
                ast: vec![
                    bf_opt::RealOps::Input,
                    bf_opt::RealOps::WhileLoop(bf_opt::BfProgram {
                        ast: vec![bf_opt::RealOps::Output, bf_opt::RealOps::Input]
                    })
                ]
            }
        )
    }
    #[test]
    fn double_loop() {
        assert_eq!(
            bf_opt::BfProgram::from("+[[]]"),
            bf_opt::BfProgram {
                ast: vec![
                    bf_opt::RealOps::Increment,
                    bf_opt::RealOps::WhileLoop(bf_opt::BfProgram {
                        ast: vec![bf_opt::RealOps::WhileLoop(bf_opt::BfProgram {
                            ast: vec![]
                        })]
                    })
                ]
            }
        )
    }
    #[test]
    fn double_conversion() {
        assert_eq!(bf_opt::BfProgram::from("+").to_string(), "+")
    }
}
