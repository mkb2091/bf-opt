#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod quickcheck_tests {
    quickcheck! {
        fn get_output_length_correct(data: String) -> bool {
        let program = bf_opt::BfProgram::from(data.as_ref());
        program.get_output_length() == program.to_string().len()
        }
    }
    quickcheck! {
        fn check_length_not_increased(data: String) -> bool {
        let program = bf_opt::BfProgram::from(data.as_ref());
        program.get_output_length() <= data.len()
        }
    }
}

#[cfg(test)]
mod to_ast {
    #[test]
    fn increment() {
        assert_eq!(
            bf_opt::BfProgram::from("+"),
            bf_opt::BfProgram {
                ast: vec![bf_opt::RealOps::Increment]
            }
        )
    }
    #[test]
    fn decrement() {
        assert_eq!(
            bf_opt::BfProgram::from("-"),
            bf_opt::BfProgram {
                ast: vec![bf_opt::RealOps::Decrement]
            }
        )
    }
    #[test]
    fn move_right() {
        assert_eq!(
            bf_opt::BfProgram::from(">"),
            bf_opt::BfProgram {
                ast: vec![bf_opt::RealOps::MoveRight]
            }
        )
    }
    #[test]
    fn move_left() {
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
}

#[cfg(test)]
mod double_conversion {
    #[test]
    fn increment() {
        assert_eq!(bf_opt::BfProgram::from("+").to_string(), "+")
    }
    #[test]
    fn decrement() {
        assert_eq!(bf_opt::BfProgram::from("-").to_string(), "-")
    }
    #[test]
    fn move_right() {
        assert_eq!(bf_opt::BfProgram::from(">").to_string(), ">")
    }
    #[test]
    fn move_left() {
        assert_eq!(bf_opt::BfProgram::from("<").to_string(), "<")
    }
    #[test]
    fn basic_cat() {
        assert_eq!(bf_opt::BfProgram::from(",[.,]").to_string(), ",[.,]")
    }
    #[test]
    fn double_loop() {
        assert_eq!(bf_opt::BfProgram::from("+[[]]").to_string(), "+[[]]")
    }
    #[test]
    fn invalid_characters() {
        assert_eq!(bf_opt::BfProgram::from("1234567890!@#$%^&*()=_abcdefghijkmnlopqrstuvwxyz").to_string(), "")
    }
}
