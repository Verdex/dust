
use std::str::{CharIndices};

use super::parse_error::{ParseError};
use super::ast::*;

pub struct Input<'a> {
    data : &'a [(usize, char)] 
}

impl<'a> Input<'a> {
    pub fn new(input : &'a [(usize, char)] ) -> Input<'a> { 
        Input { data: input }
    }

    pub fn expect(&mut self,  s : &str) -> Result<(), ParseError>  {
        self.clear()?;

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

    fn clear(&mut self) -> Result<(), ParseError> { 
        let mut d = self.data;
        let mut comment = 0;
        loop {
            match d {
                [] if comment > 0 => return Err(ParseError::EndOfFile("Expected end of comment but found end of file".to_string())),
                [] => break,
                [(_, '/'), (_, '*'), rest @ ..] => {
                    comment += 1;
                    d = rest; 
                },
                [(_, '*'), (_, '/'), rest @ ..] if comment > 0 => {
                    comment -= 1;
                    d = rest; 
                }, 
                [(_, x), rest @ ..] if comment > 0 => d = rest,
                [(_, x), rest @ ..] if x.is_whitespace() => d = rest,
                _ => break,
            }
        }
        self.data = d;
        Ok(())
    }

    pub fn parse_symbol(&mut self) -> Result<String, ParseError> {
        self.clear()?;

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

    #[test]
    fn should_clear_whitespace() -> Result<(), ParseError> {
        let mut input = Input { data: &"   x".char_indices().collect::<Vec<(usize, char)>>() };
        input.clear()?;
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "x".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_clear_block_comment() -> Result<(), ParseError> {
        let mut input = Input { data: &r#"  
        
        /* comments %^& 124

        */
        
        x"#.char_indices().collect::<Vec<(usize, char)>>() };
        input.clear()?;
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "x".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_clear_nested_block_comment() -> Result<(), ParseError> {
        let mut input = Input { data: &r#"  
        
        /* comments %^& 124

            /* nested */
            /* other nest */
            /* /* nest nest */ */

        */
        
        x"#.char_indices().collect::<Vec<(usize, char)>>() };
        input.clear()?;
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "x".to_string() ); 
        Ok(())
    }
}
