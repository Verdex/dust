
#[derive(Debug)]
pub struct Meta {
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
    Infer,
}

#[derive(Debug)]
pub struct FunSig {
    pub name : String,
    pub type_params : Vec<TypeParam>,
    pub params : Vec<Param>,
    pub return_type : Type,
}

#[derive(Debug)]
pub struct FunDef {
    pub sig : FunSig,
    // TODO defition
}

#[derive(Debug)]
pub struct Param {
    pub name : String,
    pub param_type : Type,
    pub mutable : bool,
}

#[derive(Debug)]
pub struct TypeParam {
    pub name : String,
    pub constraints : Vec<String>,
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

#[derive(Debug)]
pub struct StructField {
    pub name : String,
    pub field_type : Type,
}
