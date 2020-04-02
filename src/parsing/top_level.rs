
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {
    fn parse_type_def(&self) -> Result<TypeDef, ParseError> {
        Err(ParseError::EndOfFile(format!("blarg")))
    }
}
