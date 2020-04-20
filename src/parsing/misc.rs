
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

    pub fn parse_type(&mut self) -> Result<Type, ParseError> {
        let tuple = self.parse_tuple_type();
        match tuple {
            Ok(t) => return self.check_arrow_type(t),
            _ => (),
        }

        let simple = self.parse_symbol()?; 

        if matches!( self.expect("::"), Ok(()) ) {
            let t = self.parse_namespace_type(simple)?;
            return self.check_arrow_type(t);
        }

        if matches!( self.expect("<"), Ok(()) ) {
            let t = self.parse_index_type(simple)?;
            return self.check_arrow_type(t);
        }

        self.check_arrow_type(Type::Simple(simple))
    }

    fn parse_namespace_type(&mut self, simple : String) -> Result<Type, ParseError> {
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

        let ns_type = self.parse_symbol()?; 

        names.insert( 0, simple );

        if matches!( self.expect("<"), Ok(()) ) {
            let t = self.parse_index_type(ns_type)?;
            self.check_arrow_type(Type::Namespace( names, Box::new(t) ))
        }
        else {
            self.check_arrow_type(Type::Namespace( names, Box::new(Type::Simple(ns_type)) ))
        }
    }

    fn parse_index_type(&mut self, simple : String) -> Result<Type, ParseError> {
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

    fn parse_tuple_type(&mut self) -> Result<Type, ParseError> {
        self.expect("(")?;
        let mut types = vec![];

        if matches!( self.expect(")"), Err(_) ) {
            loop {
                let t = self.parse_type()?;

                types.push(t);

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

    fn check_arrow_type(&mut self, t : Type) -> Result<Type, ParseError> {
        if matches!( self.expect("->"), Ok(()) ) {
            let out = self.parse_type()?;
            Ok(Type::Arrow{ input: Box::new(t), output: Box::new(out) })
        }
        else {
            Ok(t)
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
        let i = "a -> (b -> c) -> d ".char_indices().collect::<Vec<(usize, char)>>();
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

        let name = match input_b {
            Type::Simple(n) => n,
            x => panic!("second input input should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "b" );

        let name = match output_c {
            Type::Simple(n) => n,
            x => panic!("second input output should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "c" );

        let name = match output_d {
            Type::Simple(n) => n,
            x => panic!("final output should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "d" );

        Ok(())
    }

    #[test]
    fn should_parse_paren_arrows() -> Result<(), ParseError> {
        let i = "a -> b -> (c -> d) -> ((e -> f) -> g) -> i ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let (input_a, output_b_etc) = match u {
            Type::Arrow{ input, output } => (*input, *output), 
            x => panic!("should be arrow type, but found {:?}", x),
        };

        let name = match input_a {
            Type::Simple(n) => n,
            x => panic!("input_a should be simple type, but found: {:?}", x),
        };

        assert_eq!(name, "a");

        let (input_b, output_cd_etc) = match output_b_etc {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!("input_b_etc should be arrow type, but found: {:?}", x),
        };

        let name = match input_b {
            Type::Simple(n) => n,
            x => panic!("input_b should be simple type, but found: {:?}", x),
        };
        
        assert_eq!(name, "b");

        let (input_cd, output_efg_etc) = match output_cd_etc {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!("output_cd_etc should be arrow type, but found: {:?}", x),
        };

        let (input_c, output_d) = match input_cd {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!("input_cd should be arrow type, but found: {:?}", x),
        };

        let name = match input_c {
            Type::Simple(n) => n,
            x => panic!("input_c should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "c" );

        let name = match output_d {
            Type::Simple(n) => n,
            x => panic!("output_d should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "d" );
        
        let (input_efg, output_i) = match output_efg_etc {
            Type::Arrow{input, output} => (*input, *output),
            x => panic!("input_efg_etc should be arrow type, but found {:?}", x),
        };

        let (input_ef, output_g) = match input_efg {
            Type::Arrow{input, output} => (*input, *output),
            x => panic!("input_efg should be arrow type, but found {:?}", x),
        };

        let (input_e, output_f) = match input_ef {
            Type::Arrow{input, output} => (*input, *output),
            x => panic!("input_ef should be arrow type, but found {:?}", x),
        };

        let name = match input_e {
            Type::Simple(n) => n,
            x => panic!("input_e should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "e" );

        let name = match output_f {
            Type::Simple(n) => n,
            x => panic!("output_f should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "f" );

        let name = match output_g {
            Type::Simple(n) => n,
            x => panic!("output_g should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "g" );

        let name = match output_i {
            Type::Simple(n) => n,
            x => panic!("output_i should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "i" );

        Ok(())
    }

    #[test]
    fn should_parse_complex_tuple() -> Result<(), ParseError> {
        let i = "(a -> b, c::d::e, (), i<j,k,l>, (m, n)) ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let mut types = match u {
            Type::Tuple(types) => types, 
            _ => panic!("should be tuple type"),
        };

        assert_eq!( types.len(), 5 );

        let one = types.remove(0);

        let (one_input, one_output) = match one {
            Type::Arrow{input, output} => (*input, *output),   
            x => panic!("one should be arrow type, but found {:?}", x),
        };

        let name = match one_input {
            Type::Simple(n) => n,
            x => panic!("one_input should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "a" );

        let name = match one_output {
            Type::Simple(n) => n,
            x => panic!("one_output should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "b" );

        let two = types.remove(0);
        
        let (names, t) = match two {
            Type::Namespace(ns, t) => (ns, *t),
            x => panic!("two should be namespace type, but found {:?}", x),
        };
        
        assert_eq!( names.len(), 2 );
        assert_eq!( names[0], "c" );
        assert_eq!( names[1], "d" );

        let name = match t {
            Type::Simple(n) => n,
            x => panic!("t should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "e" );

        let three = types.remove(0);

        assert_eq!( matches!( three, Type::Unit ), true );

        let four = types.remove(0);

        let (name, mut ts) = match four {
            Type::Indexed(n, ts) => (n, ts),
            x => panic!("four should be indexed type, but found {:?}", x),
        };

        assert_eq!( name, "i" );

        assert_eq!( ts.len(), 3 );

        let index_one = ts.remove(0);

        let name = match index_one {
            Type::Simple(n) => n,
            x => panic!( "index_one should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "j" );

        let index_two = ts.remove(0);

        let name = match index_two {
            Type::Simple(n) => n,
            x => panic!( "index_two should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "k" );

        let index_three = ts.remove(0);

        let name = match index_three {
            Type::Simple(n) => n,
            x => panic!( "index_three should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "l" );

        let five = types.remove(0);

        let mut ts = match five {
            Type::Tuple(ts) => ts,
            x => panic!( "five should be tuple type, but found {:?}", x),
        };

        assert_eq!( ts.len(), 2 );
        
        let tuple_one = ts.remove(0);

        let name = match tuple_one {
            Type::Simple(n) => n,
            x => panic!( "tuple_one should be simple type but found {:?}", x),
        };

        assert_eq!( name, "m" );

        let tuple_two = ts.remove(0);

        let name = match tuple_two {
            Type::Simple(n) => n,
            x => panic!( "tuple_two should be simple type but found {:?}", x),
        };

        assert_eq!( name, "n" );
        Ok(())
    }

    #[test]
    fn should_parse_index_namespace() -> Result<(), ParseError> {
        let i = "a::e<f> ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;
        
        let (names, t) = match u {
            Type::Namespace(names, t) => (names, *t),
            x => panic!("should be namespace type, but found {:?}", x),
        };

        assert_eq!( names.len(), 1 );
        assert_eq!( names[0], "a" );

        let (name, mut ts) = match t {
            Type::Indexed(n, ts) => (n, ts),
            x => panic!("t should be indexed type, but found {:?}", x),
        };

        assert_eq!( name, "e" );

        assert_eq!( ts.len(), 1 );
        
        let index_one = ts.remove(0);

        let name = match index_one {
            Type::Simple(n) => n,
            x => panic!("index_one should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "f" );

        Ok(())
    }

    #[test]
    fn should_parse_indexed_arrow_param() -> Result<(), ParseError> {
        let i = "a<b> -> c<d>".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let (input, output) = match u {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("should be arrow type, but found {:?}", x),
        };

        let (name, mut ts) = match input {
            Type::Indexed(name, ts) => (name, ts),
            x => panic!("input should be index type, but found {:?}", x),
        };

        assert_eq!( name, "a" );

        let index_one = ts.remove(0);

        let name = match index_one {
            Type::Simple(n) => n,
            x => panic!("index_one should be index type, but found {:?}", x),
        };

        assert_eq!( name, "b" );

        let (name, mut ts) = match output {
            Type::Indexed(name, ts) => (name, ts),
            x => panic!("output should be index type, but found {:?}", x),
        };

        assert_eq!( name, "c" );

        let index_one = ts.remove(0);

        let name = match index_one {
            Type::Simple(n) => n,
            x => panic!("index_one should be index type, but found {:?}", x),
        };

        assert_eq!( name, "d" );

        Ok(())
    }

    #[test]
    fn should_parse_namespace_arrow_param() -> Result<(), ParseError> {
        let i = "a::b -> c::d ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_type()?;

        let (input_ab, output_cd) = match u {
            Type::Arrow {input, output} => (*input, *output),
            x => panic!("should be arrow type, but found {:?}", x),
        };

        let (names, t) = match input_ab {
            Type::Namespace(ns, t) => (ns, *t),
            x => panic!("input_ab should be indexed type, but found {:?}", x),
        };

        assert_eq!( names.len(), 1 );
        assert_eq!( names[0], "a" );

        let name = match t {
            Type::Simple(n) => n,
            x => panic!("t should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "b" );

        let (names, t) = match output_cd {
            Type::Namespace(ns, t) => (ns, *t),
            x => panic!("output_cd should be indexed type, but found {:?}", x),
        };

        assert_eq!( names.len(), 1 );
        assert_eq!( names[0], "c" );

        let name = match t {
            Type::Simple(n) => n,
            x => panic!("t should be simple type, but found {:?}", x),
        };

        assert_eq!( name, "d" );

        Ok(())
    }
}
