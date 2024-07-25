#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::simple_tokens::Token;
    use crate::util::type_of;

    #[test]
    fn empty_input() {
        let result = parse("");
        assert_eq!(result.get_original(), "RootToken!!!");
        assert!(result.get_left().is_none());
        assert_eq!(result.get_right().len(), 0);
        assert_eq!(type_of(result), "abstractor::tokens::root_token::RootToken");
    }
}
