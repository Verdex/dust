
use std::str::{CharIndices};

use super::parse_error::{ParseError};
use super::ast::*;

pub struct Input<'a> {
    pub data : &'a [(usize, char)] 
}

impl<'a> Input<'a> {
    pub fn expect(&mut self,  s : &str) -> Result<(), ParseError>  {
        self.clear();

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

    fn clear(&mut self) { // TODO needs to clear comments as well
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

    pub fn parse_symbol(&mut self) -> Result<String, ParseError> {
        self.clear();

        let mut d = self.data;
        let mut cs = vec![];

        match d {
            [] => return Err(ParseError::EndOfFile("parse_symbol".to_string())),
            [(_, x), rest @ ..] if x.is_alphabetic() || *x == '_' => {
                d = rest;
                cs.push(x);
            },
            [(i, x), ..] => return Err(ParseError::ErrorAt(*i, format!("Encountered {} in parse_symbol", x))),
        }

        loop {
            match d {
                [] => return Err(ParseError::EndOfFile("parse_symbol".to_string())),
                [(_, x), rest @ ..] if x.is_alphanumeric() || *x == '_' => {
                    d = rest;
                    cs.push(x);
                },
                [(_, x), ..] => break,
            }
        }

        self.data = d;

        Ok(cs.into_iter().collect::<String>())
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

    #[test]
    fn should_parse_symbol() -> Result<(), ParseError> {
        let mut input = Input { data: &"_Symbol_123 ".char_indices().collect::<Vec<(usize, char)>>() };
        let symbol = input.parse_symbol()?;
        assert_eq!( symbol, "_Symbol_123" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), " ".to_string() ); 
        Ok(())
    }
}
