pub trait Token {
    fn get_original(&self) -> &str;
    fn get_original_location(&self) -> &str;
    fn get_left(&self) -> Option<Box<dyn Token>>;
    fn get_right(&self) -> &Vec<Box<dyn Token>>;
}