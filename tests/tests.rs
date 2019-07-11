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
}
