
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

    fn parse_type_param_list(&mut self) -> Result<Vec<TypeParam>, ParseError> {
        fn constraint_list(input : &mut Input) -> Result<Vec<String>, ParseError> {
            let mut cs = vec![];
            loop {
                let c = input.parse_symbol()?;
                cs.push(c);
                match input.expect("+") {
                    Err(_) => break,
                    _ => (),
                }
            }
            Ok(cs)
        }

        self.expect("<")?;

        let mut params = vec![];

        loop {
            let name = self.parse_symbol()?;
            match self.expect(":") {
                Ok(_) => {
                    let constraints = constraint_list(self)?;
                    params.push( TypeParam { name, constraints } );
                },
                _ => params.push( TypeParam { name, constraints : vec![] } ),
            }

            match self.expect(",") {
                Err(_) => break,
                _ => (),
            }
        }

        self.expect(">")?;

        Ok(params)
    }

    fn parse_fun_sig(&mut self) -> Result<FunSig, ParseError> {
        Err(ParseError::EndOfFile("TODO".to_string()))
    }
}
