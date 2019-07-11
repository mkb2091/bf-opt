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
    quickcheck! {
        fn basic_cat(input: Vec<u8>) -> bool {
            let program = bf_opt::BfProgram::from(",[.,]");
            let mut instance = bf_opt::interpreter::Instance::new();
            instance.run(&program, &input, 1 + 4 * input.len());
            let mut equal = true;
            for i in 0..(if input.len() > instance.output.len() { input.len() } else { instance.output.len() }) {
                if input[i] == 0 {
                    return equal;
                }
                equal &= input[i] == instance.output[i];
            }
            equal
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
        assert_eq!(
            bf_opt::BfProgram::from("1234567890!@#$%^&*()=_abcdefghijkmnlopqrstuvwxyz").to_string(),
            ""
        )
    }
    #[test]
    fn wikipedia_hello_world() {
        assert_eq!(
        bf_opt::BfProgram::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.").to_string(), 
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    }
}

#[cfg(test)]
mod interpreter_test {
    #[test]
    fn wikipedia_hello_world() {
        let program = bf_opt::BfProgram::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
        let mut instance = bf_opt::interpreter::Instance::new();
        println!("Ran: {:}", instance.run(&program, &[0], 100000));
        assert_eq!(
            instance.output,
            vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, 10]
        );
    }
}
