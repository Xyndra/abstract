mod simple_tokens;
mod tokens;
mod tests;
mod util;

use crate::tokens::RootToken;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn parse(_input: &str) -> RootToken {
    let root = RootToken {
        children: vec![],
    };
    root
}