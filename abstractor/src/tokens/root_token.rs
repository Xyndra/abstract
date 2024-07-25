use crate::simple_tokens::Token;

pub struct RootToken {
    pub children: Vec<Box<dyn Token>>,
}

impl Token for RootToken {
    fn get_original(&self) -> &str {
        "RootToken!!!"
    }

    fn get_original_location(&self) -> &str {
        "0"
    }

    fn get_left(&self) -> Option<Box<dyn Token>> {
        None
    }

    fn get_right(&self) -> &Vec<Box<dyn Token>> {
        &self.children
    }
}