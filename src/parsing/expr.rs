
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        Ok(Expr::DString("blah".to_string()))
    }
    // TODO : Use
    // TODO : lambda
    // TODO : number
    // TODO : string
    // TODO : list
    // TODO : dictionary
    // TODO : loop
    // TODO : while
    // TODO : foreach
    // TODO : match
    // TODO : let
    // TODO : mut
    // TODO : if
    // TODO : assert 
    // TODO : panic
    // TODO : try
    // TODO : return
    // TODO : dot 
    // TODO : { }
    // TODO : ( )
    // TODO : yield (?)
    // TODO : slice
}


#[cfg(test)]
mod test {
    use super::*;

   /* #[test]
    fn should_parse_type_param_list() -> Result<(), ParseError> {
        let i = "<A : B, C : D + E + F, G, H>".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type_param_list()?;
        assert_eq!( u.len(), 4 );

        assert_eq!( u[0].name, "A" );
        assert_eq!( u[0].constraints.len(), 1 );
        assert_eq!( u[0].constraints[0], "B" );

        assert_eq!( u[1].name, "C" );
        assert_eq!( u[1].constraints.len(), 3 );
        assert_eq!( u[1].constraints[0], "D" );
        assert_eq!( u[1].constraints[1], "E" );
        assert_eq!( u[1].constraints[2], "F" );
        Ok(())
    }*/

}
