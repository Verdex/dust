
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

    pub fn parse_number(&mut self) -> Result<String, ParseError> { 
        self.clear()?;
        
        let mut d = self.data;
        let mut cs = vec![];

        match d {
            [] => return Err(ParseError::EndOfFile("parse_number".to_string())),
            [(_, x), rest @ ..] if x.is_numeric() 
                                || *x == '.' 
                                || *x == '-' => {
                d = rest;
                cs.push(x);
            },
            [(i, x), ..] => return Err(ParseError::ErrorAt(*i, format!("Encountered {} in parse_number", x))),
        }

        loop {
            match d {
                [] => return Err(ParseError::EndOfFile("parse_number".to_string())),
                [(_, x), rest @ ..] if x.is_numeric() 
                                    || *x == '.' 
                                    || *x == '-' 
                                    || *x == 'E'
                                    || *x == 'e' => {
                    d = rest;
                    cs.push(x);
                },
                [(i, x), ..] => break, 
            }
        }

        self.data = d;

        Ok(cs.into_iter().collect::<String>())
    }

    pub fn parse_string(&mut self) -> Result<String, ParseError> {
        let mut d = self.data;
        let mut cs = vec![];

        match d {
            [] => return Err(ParseError::EndOfFile("parse_string".to_string())),
            [(_, '"'), rest @ ..] => d = rest,
            [(i, x), ..] => return Err(ParseError::ErrorAt(*i, format!("Encountered {} at the beginning of parse_string", x))),
        }

        let mut escape = false;
        loop {
            match d {
                [] => return Err(ParseError::EndOfFile("parse_string".to_string())),
                [(_, '\\'), rest @ ..] if escape => {
                    escape = false;
                    d = rest;
                    cs.push('\\');
                },
                [(_, 'n'), rest @ ..] if escape => {
                    escape = false;
                    d = rest;
                    cs.push('\n');
                },
                [(_, 'r'), rest @ ..] if escape => {
                    escape = false;
                    d = rest;
                    cs.push('\r');
                },
                [(_, '0'), rest @ ..] if escape => {
                    escape = false;
                    d = rest;
                    cs.push('\0');
                },
                [(_, 't'), rest @ ..] if escape => {
                    escape = false;
                    d = rest;
                    cs.push('\t');
                },
                [(_, '"'), rest @ ..] if escape => {
                    escape = false;
                    d = rest;
                    cs.push('"');
                },
                [(i, x), rest @ ..] if escape => return Err(ParseError::At(*i, format!("Encountered unknown escape character {}", x))),
                [(_, '\\'), rest @ ..] => {
                    escape = true;
                    d = rest;
                },
                [(_, '"'), rest @ ..] => break,
                [(_, x), rest @ ..] => {
                    d = rest;
                    cs.push(x);
                },
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

    #[test]
    fn should_parse_int() -> Result<(), ParseError> {
        let mut input = Input { data: &"1234 ".char_indices().collect::<Vec<(usize, char)>>() };
        let number = input.parse_number()?;
        assert_eq!( number, "1234" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), " ".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_parse_float() -> Result<(), ParseError> {
        let mut input = Input { data: &"12.34 ".char_indices().collect::<Vec<(usize, char)>>() };
        let number = input.parse_number()?;
        assert_eq!( number, "12.34" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), " ".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_parse_float_starting_with_dot() -> Result<(), ParseError> {
        let mut input = Input { data: &".01 ".char_indices().collect::<Vec<(usize, char)>>() };
        let number = input.parse_number()?;
        assert_eq!( number, ".01" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), " ".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_parse_scientific_notation() -> Result<(), ParseError> {
        let mut input = Input { data: &"1234e42.0 ".char_indices().collect::<Vec<(usize, char)>>() };
        let number = input.parse_number()?;
        assert_eq!( number, "1234e42.0" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), " ".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_parse_negative_scientific_notation() -> Result<(), ParseError> {
        let mut input = Input { data: &"1234E-42 ".char_indices().collect::<Vec<(usize, char)>>() };
        let number = input.parse_number()?;
        assert_eq!( number, "1234E-42" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), " ".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_parse_negative_int() -> Result<(), ParseError> {
        let mut input = Input { data: &"-1234 ".char_indices().collect::<Vec<(usize, char)>>() };
        let number = input.parse_number()?;
        assert_eq!( number, "-1234" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), " ".to_string() ); 
        Ok(())
    }
}
