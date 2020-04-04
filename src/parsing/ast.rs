
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
    type_structure : TypeStruct,
}

#[derive(Debug)]
pub enum TypeStruct {
    Simple(String),
    Indexed(String, Vec<TypeStruct>),
    Arrow { input : Box<TypeStruct>, output : Box<TypeStruct> },
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
