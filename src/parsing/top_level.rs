
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {
    // TODO struct def
    // TODO enum def
    // TODO fun def
    // TODO impl def
        // TODO type def
    // TODO mod
    // TODO trait def
        // TODO func sig
        // TODO type sig (need way to indicate if it is an owned type (maybe do that in the function def?))
    pub fn parse_type_def(&self) -> Result<TypeDef, ParseError> {
        Err(ParseError::EndOfFile(format!("blarg")))
    }
}
