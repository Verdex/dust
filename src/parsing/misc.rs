
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

    pub parse_type(&mut self) -> Result<Type, ParseError> {

    }

    fn parse_tuple_type(&mut self) -> Result<Type, ParseError> {
        self.expect("(")?;
        let mut types = vec![];

        println!("paren begin");

        if matches!( self.expect(")"), Err(_) ) {
            loop {
                let t = self.parse_type()?;
                println!("paren type: {:?}", t);
                types.push(t);


                if matches!( self.expect(")"), Ok(()) ) {
                    println!("paren end");
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

// a, (), (a), (a,b), (a,b,c), a -> b, a -> b -> c, a<b>, a<b,c,d>, (a -> b) -> c, a::b // module::concrete type or trait::abstract type
    fn parse_tail_type(&mut self) -> Result<Type, ParseError> {
        let tuple = self.parse_tuple_type();
        match tuple {
            Ok(t) => 
        }
        /*if matches!( self.expect("("), Ok(()) ) {
            let mut types = vec![];

            println!("paren begin");

            if matches!( self.expect(")"), Err(_) ) {
                loop {
                    let t = self.parse_type()?;
                    println!("paren type: {:?}", t);
                    types.push(t);


                    if matches!( self.expect(")"), Ok(()) ) {
                        println!("paren end");
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
        else {*/
            let simple = self.parse_symbol()?;
            println!("else simple symbol: {:?}", simple);

            if matches!( self.expect("->"), Ok(()) ) {
                println!("check arrow output");
                let out = self.parse_type()?;
                println!("arrow output: {:?}", out);
                Ok(Type::Arrow{ input: Box::new(Type::Simple(simple)), output: Box::new(out) })
            }
            else if matches!( self.expect("::"), Ok(()) ) {
                let mut names = vec![];

                loop {
                    let restore_point = self.create_restore();
                    let name = self.parse_symbol()?;

                    if !matches!( self.expect("::"), Ok(()) ) {
                        self.restore(restore_point);
                        break;
                    }

                    names.push(name);
                }

                let t = self.parse_type()?;

                match t {
                    Type::Simple(_) => (),
                    Type::Indexed(_, _) => (),
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

                names.insert( 0, simple );
                
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

                Ok(Type::Indexed( simple, types ))
            }
            else {
                Ok(Type::Simple(simple))
            }
        //}
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
        assert_eq!( names[1], "mod2" );
        assert_eq!( names[2], "Trait" );

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

    #[test]
    fn should_parse_arrow_type() -> Result<(), ParseError> {
        let i = "alpha -> beta ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let (input, output) = match u {
            Type::Arrow { input, output } => (input, output), 
            _ => panic!("should be arrow type"),
        };

        let i_name = match *input {
            Type::Simple(s) => s,
            _ => panic!("input type should be simple"),
        };

        assert_eq!( i_name, "alpha" );

        let o_name = match *output {
            Type::Simple(s) => s,
            _ => panic!("input type should be simple"),
        };

        assert_eq!( o_name, "beta" );
        Ok(())
    }

    #[test]
    fn should_parse_paren_type() -> Result<(), ParseError> {
        let i = "(((alpha))) ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let name = match u {
            Type::Simple(s) => s, 
            _ => panic!("should be simple type"),
        };

        assert_eq!( name, "alpha" );
        Ok(())
    }

    #[test]
    fn should_parse_arrow_past_arrow_parameter() -> Result<(), ParseError> {
        let i = "a -> (b -> c) -> d".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;


        let (input_a, output_bc_etc) = match u {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("should be arrow type, but found: {:?}", x),
        };

        let name = match input_a {
            Type::Simple(n) => n,
            x => panic!("first input should be simple type, but found: {:?}", x),
        };

        assert_eq!( name, "a" );

        let (input_bc, output_d) = match output_bc_etc {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("first output should be arrow type, but found: {:?}", x),
        };

        let (input_b, output_c) = match input_bc {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("second input should be arrow type, but found {:?}", x),
        };

        Ok(())
    }

    #[test]
    fn should_parse_paren_arrows() -> Result<(), ParseError> {
        let i = "a -> b -> (c -> d) -> ((e -> f) -> g) -> i ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        println!("u = {:?}", u);

        let (input_a, output_b_etc) = match u {
            Type::Arrow{ input, output } => (*input, *output), 
            _ => panic!("should be arrow type"),
        };

        let name = match input_a {
            Type::Simple(n) => n,
            x => panic!("first input should be simple type, but found: {:?}", x),
        };

        assert_eq!(name, "a");

        let (input_b, output_cd_etc) = match output_b_etc {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!("first output should be arrow type, but found: {:?}", x),
        };

        let name = match input_b {
            Type::Simple(n) => n,
            x => panic!("second input should be simple type, but found: {:?}", x),
        };
        
        assert_eq!(name, "b");

        let (input_cd, output_efg_etc) = match output_cd_etc {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!("second output should be arrow type, but found: {:?}", x),
        };

        let (input_c, output_d) = match input_cd {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!("third input should be arrow type, but found: {:?}", x),
        };

        let name = match input_c {
            Type::Simple(n) => n,
            x => panic!("third input's input should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "c" );
        
        Ok(())
    }

    #[test]
    fn should_parse_complex_tuple() -> Result<(), ParseError> {
        let i = "(a -> b, c::d::e, (f -> g) -> h, (), i<j,k,l>, (m, n)) ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let types = match u {
            Type::Tuple(types) => types, 
            _ => panic!("should be tuple type"),
        };

        assert_eq!( types.len(), 6 );
        Ok(())
    }
}
