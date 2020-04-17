
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
                            // TODO Error messages
                            // TODO index
                        Type::Namespace(_, _) => 
                            return Err(ParseError::ErrorAt(0, format!(""))),
                        Type::Unit => 
                            return Err(ParseError::ErrorAt(0, format!(""))),
                        Type::Tuple(_) => 
                            return Err(ParseError::ErrorAt(0, format!(""))),
                        Type::Arrow{ .. } => 
                            return Err(ParseError::ErrorAt(0, format!(""))),
                    }

                    if !matches!( self.expect("::"), Ok(()) ) {
                        break;
                    }
                }

                let t = types.pop().expect("parse_type has impossible empty types list");

                let mut names = types.into_iter().map(|x| match x {
                    Type::Simple(s) => s,
                    _ => panic!("parse_type encountered impossible non-simple type"),
                }).collect::<Vec<String>>();
                   
                let mut all = vec![simple];
                all.append(&mut names);

                Ok(Type::Namespace( all, Box::new(t) ))
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

                Ok(Type::Indexed( simple, types ))
            }
            else {
                Ok(Type::Simple(simple))
            }
        }
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

    #[test]
    fn should_parse_simple_type() -> Result<(), ParseError> {
        let i = "simple ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;
        let name = match u {
            Type::Simple(s) => s,
            _ => panic!("should be simple type"), 
        };
        assert_eq!( name, "simple" );
        Ok(())
    }

    #[test]
    fn should_parse_indexed_type() -> Result<(), ParseError> {
        let i = "simple<alpha, beta> ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;
        let (name, types) = match u {
            Type::Indexed(s, ts) => (s, ts),
            _ => panic!("should be indexed type"),
        };
        assert_eq!( name, "simple" );
        assert_eq!( types.len(), 2 );

        let i0_name = match &types[0] {
            Type::Simple(s) => s,
            _ => panic!("index 0 should be simple type"),
        };
        
        let i1_name = match &types[1] {
            Type::Simple(s) => s,
            _ => panic!("index 1 should be simple type"),
        };

        assert_eq!( i0_name, "alpha" );
        assert_eq!( i1_name, "beta" );

        Ok(())
    }

    #[test]
    fn should_parse_namespace_type() -> Result<(), ParseError> {
        let i = "mod1::mod2::Trait::Type ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;
        let (names, t) = match u {
            Type::Namespace(ns, t) => (ns, t),
            _ => panic!("should be namespace type"),
        };

        assert_eq!( names.len(), 3 );
        assert_eq!( names[0], "mod1" );

        let st_name = match *t {
            Type::Simple(s) => s,
            _ => panic!("type should be simple type"),
        };

        assert_eq!( st_name, "Type" );

        Ok(())
    }

    #[test]
    fn should_parse_unit_type() -> Result<(), ParseError> {
        let i = "() ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        match u {
            Type::Unit => Ok(()),
            _ => panic!("should be unit type"),
        }
    }

    #[test]
    fn should_parse_tuple_type() -> Result<(), ParseError> {
        let i = "(alpha, beta, gamma) ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let types = match u {
            Type::Tuple(ts) => ts, 
            _ => panic!("should be tuple type"),
        };

        assert_eq!( types.len(), 3 );

        let t1_name = match &types[0] {
            Type::Simple(s) => s,
            _ => panic!("t1 should be simple type"),
        };

        assert_eq!( t1_name, "alpha" );
        Ok(())
    }
}
