
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {
    fn parse_use(&mut self) -> Result<Use, ParseError> {
        self.expect("use")?;

        loop {
            // check symbol
            // check ::
        }
        
        // check {
        // symbol list
        // maybe *
        // check }

        Err(ParseError::EndOfFile(format!("blarg")))
    }
}
