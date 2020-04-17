
use super::ast::*;
use super::parse_error::ParseError;
use super::input::Input;

impl<'a> Input<'a> {
    fn parse_use(&mut self) -> Result<Use, ParseError> {
        self.expect("use")?;

        let mut namespace = vec![];
        let mut imports = vec![];

        namespace.push(self.parse_symbol()?);

        loop {
            self.expect("::")?;
            if matches!(self.expect("{"), Ok(_)) {
                break;
            }
            namespace.push(self.parse_symbol()?);
        }
        
        loop {
            if matches!(self.expect("}"), Ok(_)) {
                break;
            }
            if matches!(self.expect("*"), Ok(_)) {
                imports.push(Import::Everything);
            }
            else {
                imports.push(Import::Item(self.parse_symbol()?));
            }
            match self.expect(",") {
                Ok(_) => continue,
                _ => (),
            }
        }

        self.expect(";")?;

        Ok( Use { imports, namespace } )
    }

// a, (), (a), (a,b), (a,b,c), a -> b, a -> b -> c, a<b>, a<b,c,d>, (a -> b) -> c, a::b // module::concrete type or trait::abstract type
    pub fn parse_type(&mut self) -> Result<Type, ParseError> {
        if matches!( self.expect("("), Ok(()) ) {
            let mut types = vec![];

            if matches!( self.expect(")"), Err(_) ) {
                loop {
                    types.push( self.parse_type()? );

                    if matches!( self.expect(")"), Ok(()) ) {
                        break;
                    }

                    self.expect(",")?;
                }

                self.expect(")")?;
            }

            match types.len() {
                0 => Ok(Type::Unit),
                1 => Ok(types.remove(0)),
                _ => Ok(Type::Tuple(types)),
            }
        }
        else {
            let simple = self.parse_symbol()?;

            if matches!( self.expect("->"), Ok(()) ) {
                let out = self.parse_type()?;
                Ok(Type::Arrow{ input: Box::new(Type::Simple(simple)), output: Box::new(out) })
            }
            else if matches!( self.expect("::"), Ok(()) ) {
                let mut types = vec![];

                loop {
                    let t = self.parse_type()?;
                    match t {
                        Type::Simple(s) => types.push(Type::Simple(s)),
                        Type::Indexed(s, ts) => 
                            types.push(Type::Indexed(s, ts)),
                        Type::Namespace(_, _) => 
                            return Err(ParseError::ErrorAt(0, format!(""))),
                        Type::Unit => 
                            return Err(ParseError::ErrorAt(0, format!(""))),
                        Type::Tuple(_) => 
                            return Err(ParseError::ErrorAt(0, format!(""))),
                        Type::Arrow{ .. } => 
                            return Err(ParseError::ErrorAt(0, format!(""))),
                    }

                    if matches!( self.expect(">"), Ok(()) ) {
                        break;
                    }

                    self.expect(",")?;
                }

                let t = types.pop().unwrap();

                let names = types.into_iter().map(|x| match x {
                    Type::Simple(s) => s,
                    _ => panic!(""),
                }).collect::<Vec<String>>();
                   

                Ok(Type::Namespace( names, Box::new(t) ))
            }
            else if matches!( self.expect("<"), Ok(()) ) {
                let mut types = vec![];

                loop {
                    types.push( self.parse_type()? );

                    if matches!( self.expect(">"), Ok(()) ) {
                        break;
                    }

                    self.expect(",")?;
                }

                self.expect(">")?;

                Ok(Type::Indexed( simple, types ))
            }
            else {
                Ok(Type::Simple(simple))
            }
        }

         
        /*else if matches!( self.parse_symbol() ) {
            // solo symbol
            // follow by arrow
            // follow by <
            // follow by ::
            // follow by ,
            // follow by )
            // follow by >

        }*/

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_empty_use() -> Result<(), ParseError> {
        let i = "use symb::{};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 0 );
        assert_eq!( u.namespace.len(), 1);
        assert_eq!( u.namespace[0], "symb" );
        Ok(())
    }
    
    #[test]
    fn should_parse_use_with_everything() -> Result<(), ParseError> {
        let i = "use symb::{*};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 1 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert_eq!( u.namespace.len(), 1);
        assert_eq!( u.namespace[0], "symb" );
        Ok(())
    }

    #[test]
    fn should_parse_use_with_everythings() -> Result<(), ParseError> {
        let i = "use symb::{*, *};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert!( matches!( u.imports[1], Import::Everything ) );
        assert_eq!( u.namespace.len(), 1);
        assert_eq!( u.namespace[0], "symb" );
        Ok(())
    }

    #[test]
    fn should_parse_use_with_long_namespace() -> Result<(), ParseError> {
        let i = "use symb::other::some::{*, *};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert!( matches!( u.imports[1], Import::Everything ) );
        assert_eq!( u.namespace.len(), 3);
        assert_eq!( u.namespace[0], "symb" );
        assert_eq!( u.namespace[1], "other" );
        assert_eq!( u.namespace[2], "some" );
        Ok(())
    }

    #[test]
    fn should_parse_use_with_everything_and_item() -> Result<(), ParseError> {
        let i = "use symb::other::some::{*, item};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert!( matches!( u.imports[1], Import::Item(_) ) );
        match &u.imports[1] {
            Import::Item(item) if item == "item" => (),
            _ => assert!(false),
        }
        
        assert_eq!( u.namespace.len(), 3);
        assert_eq!( u.namespace[0], "symb" );
        assert_eq!( u.namespace[1], "other" );
        assert_eq!( u.namespace[2], "some" );
        Ok(())
    }

    #[test]
    fn should_parse_use_with_items() -> Result<(), ParseError> {
        let i = "use symb::other::some::{item1, item2};".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Item(_) ) );
        assert!( matches!( u.imports[1], Import::Item(_) ) );
        match &u.imports[0] {
            Import::Item(item) if item == "item1" => (),
            _ => assert!(false),
        }
        match &u.imports[1] {
            Import::Item(item) if item == "item2" => (),
            _ => assert!(false),
        }
        
        assert_eq!( u.namespace.len(), 3);
        assert_eq!( u.namespace[0], "symb" );
        assert_eq!( u.namespace[1], "other" );
        assert_eq!( u.namespace[2], "some" );
        Ok(())
    }
}
