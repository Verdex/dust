
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

pub fn parse( input : &str ) -> Result<Module, ParseError> {
    Err(ParseError::EndOfFile(format!("blarg")))
}
