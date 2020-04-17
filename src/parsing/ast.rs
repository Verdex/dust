
#[derive(Debug)]
pub struct meta {
    start : usize,
    end : usize,
}

#[derive(Debug)]
pub struct Module {
    pub type_defs : Vec<TypeDef>,
    pub fun_defs : Vec<FunDef>,
    pub uses : Vec<Use>,
}

#[derive(Debug)]
pub struct TypeDef {
    name : String,
    type_params : Vec<String>,
    type_constructors : Vec<TypeConstructor>,
}

#[derive(Debug)]
pub struct TypeConstructor {
    name : String,
    types : Vec<Type>,
}

#[derive(Debug)]
pub enum Type {
    Unit,
    Simple(String),
    Indexed(String, Vec<Type>),
    Arrow { input : Box<Type>, output : Box<Type> },
    Tuple(Vec<Type>),
    Namespace(Vec<String>, Box<Type>),
}

#[derive(Debug)]
pub struct FunDef {

}

#[derive(Debug)]
pub struct Use {
    pub namespace : Vec<String>,
    pub imports : Vec<Import>,
}

#[derive(Debug)]
pub enum Import {
    Everything,
    Item(String),
}
