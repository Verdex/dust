
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_empty_use() -> Result<(), ParseError> {
        let mut input = Input { data: &"use symb::{};".char_indices().collect::<Vec<(usize, char)>>() };
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 0 );
        assert_eq!( u.namespace.len(), 1);
        assert_eq!( u.namespace[0], "symb" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        Ok(())
    }
    
    #[test]
    fn should_parse_use_with_everything() -> Result<(), ParseError> {
        let mut input = Input { data: &"use symb::{*};".char_indices().collect::<Vec<(usize, char)>>() };
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 1 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert_eq!( u.namespace.len(), 1);
        assert_eq!( u.namespace[0], "symb" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_parse_use_with_everythings() -> Result<(), ParseError> {
        let mut input = Input { data: &"use symb::{*, *};".char_indices().collect::<Vec<(usize, char)>>() };
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert!( matches!( u.imports[1], Import::Everything ) );
        assert_eq!( u.namespace.len(), 1);
        assert_eq!( u.namespace[0], "symb" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_parse_use_with_long_namespace() -> Result<(), ParseError> {
        let mut input = Input { data: &"use symb::other::some::{*, *};".char_indices().collect::<Vec<(usize, char)>>() };
        let u = input.parse_use()?;
        assert_eq!( u.imports.len(), 2 );
        assert!( matches!( u.imports[0], Import::Everything ) );
        assert!( matches!( u.imports[1], Import::Everything ) );
        assert_eq!( u.namespace.len(), 3);
        assert_eq!( u.namespace[0], "symb" );
        assert_eq!( u.namespace[1], "other" );
        assert_eq!( u.namespace[2], "some" );
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_parse_use_with_everything_and_item() -> Result<(), ParseError> {
        let mut input = Input { data: &"use symb::other::some::{*, item};".char_indices().collect::<Vec<(usize, char)>>() };
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
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        Ok(())
    }

    #[test]
    fn should_parse_use_with_items() -> Result<(), ParseError> {
        let mut input = Input { data: &"use symb::other::some::{item1, item2};".char_indices().collect::<Vec<(usize, char)>>() };
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
        assert_eq!( input.data.into_iter().map(|(_,x)| x).collect::<String>(), "".to_string() ); 
        Ok(())
    }
}
