
use std::str::{CharIndices};

use super::parse_error::{ParseError};
use super::ast::*;

pub struct Input<'a> {
    data : &'a [(usize, char)] 
}

impl<'a> Input<'a> {
    fn expect(&mut self,  s : &str) -> Result<(), ParseError>  {
        let mut d = self.data;
        for c in s.chars() {
            match d {
                [] => return Err(ParseError::EndOfFile(format!("Expected {} in {}", c, s))),
                [(_, x), rest @ ..] if *x == c => d = rest,
                [(i, x), ..] => return Err(ParseError::ErrorAt(*i, format!("Expected {} in {} but found {}", c, s, x))),
            }
        }
        self.data = d;
        Ok(())
    }

    fn clear_whitespace(&mut self) {
        let mut d = self.data;
        loop {
            match d {
                [] => break,
                [(_, x), rest @ ..] if x.is_whitespace() => d = rest,
                _ => break,
            }
        }
        self.data = d
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_expect_string() -> Result<(), ParseError> {
        let mut input = Input { data: &"::<>::".char_indices().collect::<Vec<(usize, char)>>() };
        input.expect("::<>::")?;
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        Ok(())
    }

}
