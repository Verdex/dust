
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {

    pub fn parse_fun_def(&mut self) -> Result<FunDef, ParseError> {
    // TODO move this to misc so that we can parse local functions
    // TODO alternatively, we can just force let lambda for local functions
    // which should be okay if the let can pick up on the inference
        Err(ParseError::EndOfFile("TODO".to_string()))
    }

    pub fn parse_enum_def(&mut self) -> Result<(), ParseError> {
        Err(ParseError::EndOfFile("TODO".to_string()))
    }

    pub fn parse_struct_def(&mut self) -> Result<(), ParseError> {
        Err(ParseError::EndOfFile("TODO".to_string()))
    }

    pub fn parse_trait_def(&mut self) -> Result<(), ParseError> {
        Err(ParseError::EndOfFile("TODO".to_string()))
    }

    pub fn parse_impl_def(&mut self) -> Result<(), ParseError> {
        Err(ParseError::EndOfFile("TODO".to_string()))
        // TODO type def
    }

    pub fn parse_mod(&mut self) -> Result<(), ParseError> {
        Err(ParseError::EndOfFile("TODO".to_string()))
    }
        // TODO type sig (need way to indicate if it is an owned type (maybe do that in the function def?)) 

    fn parse_fun_sig(&mut self) -> Result<FunSig, ParseError> {
        Err(ParseError::EndOfFile("TODO".to_string()))
    }
}
