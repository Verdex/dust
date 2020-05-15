
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

    pub fn parse_enum_def(&mut self) -> Result<EnumDef, ParseError> {
        fn parse_types( input : &mut Input ) -> Result<Vec<Type>, ParseError> {
            let mut types = vec![];
            input.expect("(")?;
            loop {
                let t = input.parse_type()?;
                match input.expect(",") {
                    Ok(_) => types.push(t), 
                    Err(_) => break,
                }
            }
            input.expect(")")?;
            Ok(types)
        }

        fn parse_cases( input : &mut Input ) -> Result<Vec<EnumCase>, ParseError> {
            input.expect("{")?; 

            let mut cases = vec![];
            loop {

                let name = match input.parse_symbol() {
                    Ok(name) => name,
                    Err(_) => break,
                };
                
                match parse_types(input) {
                    Ok(types) => {
                        cases.push( EnumCase::TypeCase { name, types } );
                    },
                    Err(_) => {
                        match input.parse_struct_field_list() {
                            Ok(fields) if fields.len() > 0 => {
                                cases.push( EnumCase::StructCase { name, fields } );
                            },
                            _ => {
                                cases.push( EnumCase::EmptyCase { name } );
                            }
                        }
                    },
                }

                match input.expect(",") {
                    Err(_) => break,
                    _ => (),
                }
            }

            input.expect("}")?;

            Ok(cases)
        }

        self.expect("enum")?;
        let name = self.parse_symbol()?;

        match self.parse_type_param_list() {
            Ok(type_params) => {
                let cases = parse_cases(self)?;     
                Ok( EnumDef { name, type_params, cases } )
            },
            Err(_) => {
                let cases = parse_cases(self)?;     
                Ok( EnumDef { name, type_params: vec![], cases } )
            },
        }
    }

    pub fn parse_struct_def(&mut self) -> Result<StructDef, ParseError> {
        self.expect("struct")?;
        let name = self.parse_symbol()?;
        match self.parse_type_param_list() {
            Ok(type_params) => {
                let fields = self.parse_struct_field_list()?;
                Ok( StructDef { name, type_params, fields } ) 
            },
            Err(_) => {
                let fields = self.parse_struct_field_list()?;
                Ok( StructDef { name, type_params: vec![], fields } ) 
            },
        }
    }

    pub fn parse_trait_def(&mut self) -> Result<(), ParseError> {
        Err(ParseError::EndOfFile("TODO".to_string()))
    }
        // TODO type sig (need way to indicate if it is an owned type) 

    pub fn parse_impl_def(&mut self) -> Result<(), ParseError> {
        Err(ParseError::EndOfFile("TODO".to_string()))
        // TODO type def
    }

    pub fn parse_mod(&mut self) -> Result<String, ParseError> {
        self.expect("mod")?;
        let name = self.parse_symbol()?;
        self.expect(";")?;
        Ok(name)
    }

    fn parse_struct_field_list(&mut self) -> Result<Vec<StructField>, ParseError> {
        let mut fields = vec![];
        
        self.expect("{")?;

        loop {
            match self.parse_symbol() {
                Ok(name) => {
                    self.expect(":")?;
                    let field_type = self.parse_type()?;
                    fields.push( StructField { name, field_type } );
                    match self.expect(",") {
                        Ok(_) => (),
                        Err(_) => break,
                    }
                },
                Err(_) => break,
            }
        }

        self.expect("}")?;

        Ok(fields)
    }

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

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ParseError> {

        self.expect("(")?;

        let mut params = vec![];

        loop {
            
            match self.expect("mut") {
                Ok(_) =>  {
                    let name = self.parse_symbol()?;
                    self.expect(":")?;
                    let param_type = self.parse_type()?;
                    params.push( Param { name, param_type, mutable: true } );
                },
                Err(_) => {
                    match self.parse_symbol() {
                        Ok(name) => {
                            self.expect(":")?;
                            let param_type = self.parse_type()?;
                            params.push( Param { name, param_type, mutable: false } );
                        },
                        Err(_) => break, 
                    }
                },
            }

            match self.expect(",") {
                Err(_) => break,
                _ => (),
            }
        }

        self.expect(")")?;

        Ok(params)
    }

    fn parse_fun_sig(&mut self) -> Result<FunSig, ParseError> {

        let name = self.parse_symbol()?;

        let type_params = match self.parse_type_param_list() {
            Ok(tps) => tps,
            Err(_) => vec![],
        };

        let params = self.parse_param_list()?;

        match self.expect("->") {
            Ok(_) => {
                let return_type = self.parse_type()?;
                Ok( FunSig { name, type_params, params, return_type } )
            },
            Err(_) => {
                Ok( FunSig { name, type_params, params, return_type: Type::Unit } )
            },
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
    }

    #[test]
    fn should_parse_param_list() -> Result<(), ParseError> { 
        let i = "( mut a : A -> B, b : B<C>, mut c : (C, D) ) ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let mut u = input.parse_param_list()?;
        assert_eq!( u.len(), 3 );

        let a = u.remove(0);

        assert_eq!( a.name, "a" );
        let (input, output) = match a.param_type {
            Type::Arrow { input, output } => (*input, *output),
            x => panic!( "Expected Arrow, but found {:?}", x ),
        };

        match input {
            Type::Simple(n) => assert_eq!( n, "A" ),
            x => panic!( "Expected Simple but found {:?}", x ),
        }

        match output {
            Type::Simple(n) => assert_eq!( n, "B" ),
            x => panic!( "Expected Simple but found {:?}", x ),
        }

        assert_eq!( a.mutable, true );

        let b = u.remove(0);

        assert_eq!( b.name, "b" );
        
        let (b_type_name, mut b_type_params) = match b.param_type {
            Type::Indexed( name, params ) => (name, params),
            x => panic!( "Expected Indexed but found {:?}", x ),
        };

        assert_eq!( b_type_name, "B" );
        
        assert_eq!( b_type_params.len(), 1 );

        let b_type_param_0 = b_type_params.remove(0);

        match b_type_param_0 {
            Type::Simple(name) => assert_eq!( name, "C" ),
            x => panic!( "Expected Simple but found {:?}", x ),
        }

        assert_eq!( b.mutable, false );

        let c = u.remove(0);

        assert_eq!( c.name, "c" );

        let mut c_type_array = match c.param_type {
            Type::Tuple(types) => types,
            x => panic!( "Expected Tuple but found {:?}", x ),
        };

        assert_eq!( c_type_array.len(), 2 );

        let c_type_0 = match c_type_array.remove(0) {
            Type::Simple(n) => n,
            x => panic!( "Expected Simple but found {:?}", x ),
        };

        assert_eq!( c_type_0, "C" );

        let c_type_1 = match c_type_array.remove(0) {
            Type::Simple(n) => n,
            x => panic!( "Expected Simple but found {:?}", x ),
        };

        assert_eq!( c_type_1, "D" );

        assert_eq!( c.mutable, true );

        Ok(())
    }

    #[test]
    fn should_parse_emtpy_param_list() -> Result<(), ParseError> { 
        let i = "() ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_param_list()?;
        assert_eq!( u.len(), 0 );

        Ok(())
    }

    #[test]
    fn should_parse_fun_sig_with_return_type() -> Result<(), ParseError> { 
        let i = "function(blah : T) -> X ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_fun_sig()?;

        assert_eq!( u.name, "function" );
        assert_eq!( u.type_params.len(), 0 );

        match u.return_type {
            Type::Simple(n) => assert_eq!( n, "X" ),
            x => panic!( "Expected Simple but found {:?}", x ),
        }

        Ok(())
    }

    #[test]
    fn should_parse_fun_sig_with_type_param() -> Result<(), ParseError> { 
        let i = "function<T>(blah : T) ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_fun_sig()?;

        assert_eq!( u.name, "function" );
        assert_eq!( u.type_params.len(), 1 );

        assert_eq!( u.type_params[0].name, "T" );
        assert_eq!( u.type_params[0].constraints.len(), 0 );

        match u.return_type {
            Type::Unit => (),
            x => panic!( "Expected Unit but found {:?}", x ),
        }

        Ok(())
    }

    #[test]
    fn should_parse_struct_field_list() -> Result<(), ParseError> { 
        let i = "{ a : a_type, b : b_type } ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let mut u = input.parse_struct_field_list()?;

        assert_eq!( u.len(), 2 );

        let a = u.remove(0);
        let b = u.remove(0);

        assert_eq!( a.name, "a" );

        match a.field_type {
            Type::Simple(n) => assert_eq!( n, "a_type" ),
            x => panic!( "Expected Simple but found {:?}", x ),
        }

        assert_eq!( b.name, "b" );

        match b.field_type {
            Type::Simple(n) => assert_eq!( n, "b_type" ),
            x => panic!( "Expected Simple but found {:?}", x ),
        }

        Ok(())
    }

    #[test]
    fn should_parse_empty_struct_field_list() -> Result<(), ParseError> { 
        let i = "{ } ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_struct_field_list()?;

        assert_eq!( u.len(), 0 );

        Ok(())
    }

    #[test]
    fn should_parse_struct_field_def() -> Result<(), ParseError> { 
        let i = "struct some { a : a_type, b : b_type } ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_struct_def()?;

        assert_eq!( u.name, "some" );
        assert_eq!( u.fields.len(), 2 );
        assert_eq!( u.type_params.len(), 0 );

        Ok(())
    }

    #[test]
    fn should_parse_struct_field_def_with_type_params() -> Result<(), ParseError> { 
        let i = "struct some<Type : Constraint> { a : a_type, b : b_type } ".char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let u = input.parse_struct_def()?;

        assert_eq!( u.name, "some" );
        assert_eq!( u.fields.len(), 2 );
        assert_eq!( u.type_params.len(), 1 );

        Ok(())
    }

    #[test]
    fn should_parse_enum_field() -> Result<(), ParseError> { 
        let i = r#"
enum some {  
    One,
    Two,
    Three
} "#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let mut u = input.parse_enum_def()?;

        assert_eq!( u.name, "some" );
        assert_eq!( u.cases.len(), 3 );
        assert_eq!( u.type_params.len(), 0 );

        let one = u.cases.remove(0);
        let two = u.cases.remove(0);
        let three = u.cases.remove(0);

        match one {
            EnumCase::EmptyCase { name } => assert_eq!( name, "One" ),
            x => panic!( "expected empty case but found {:?}", x ),
        }

        match two {
            EnumCase::EmptyCase { name } => assert_eq!( name, "Two" ),
            x => panic!( "expected empty case but found {:?}", x ),
        }

        match three {
            EnumCase::EmptyCase { name } => assert_eq!( name, "Three" ),
            x => panic!( "expected empty case but found {:?}", x ),
        }

        Ok(())
    }

    #[test]
    fn should_parse_enum_field_def_with_type_params() -> Result<(), ParseError> { 
        let i = r#"
enum some<T : Constraint> {  
    One
} "#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let mut u = input.parse_enum_def()?;

        assert_eq!( u.name, "some" );
        assert_eq!( u.cases.len(), 1 );
        assert_eq!( u.type_params.len(), 1 );

        Ok(())
    }

    #[test]
    fn should_parse_enum_field_def_with_no_cases() -> Result<(), ParseError> { 
        let i = r#"
enum some {  
} "#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let mut u = input.parse_enum_def()?;

        assert_eq!( u.name, "some" );
        assert_eq!( u.cases.len(), 0 );
        assert_eq!( u.type_params.len(), 0 );

        Ok(())
    }

    #[test]
    fn should_parse_enum_field_def_with_struct_case() -> Result<(), ParseError> { 
        let i = r#"
enum some {  
    Blah { a : T, b : T },
    Blah { a : T, b : T },
} "#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let mut u = input.parse_enum_def()?;

        assert_eq!( u.name, "some" );
        assert_eq!( u.cases.len(), 2 );
        assert_eq!( u.type_params.len(), 0 );

        Ok(())
    }

    #[test]
    fn should_parse_enum_field_def_with_type_case() -> Result<(), ParseError> { 
        let i = r#"
enum some {  
    Blah (T1, T2, T3),
    Blah (T1, T2),
} "#.char_indices().collect::<Vec<(usize, char)>>();
        let mut input = Input::new(&i);
        let mut u = input.parse_enum_def()?;

        assert_eq!( u.name, "some" );
        assert_eq!( u.cases.len(), 2 );
        assert_eq!( u.type_params.len(), 0 );

        Ok(())
    }
}
