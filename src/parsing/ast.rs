
pub struct meta {
    start : usize,
    end : usize,
}

pub struct Module {
    type_defs : Vec<TypeDef>,
    fun_defs : Vec<FunDef>,
    uses : Vec<Use>,
}

pub struct TypeDef {
    
}

pub struct FunDef {}

pub struct Use {
    namespace : Vec<String>,
    imports : Vec<Import>,
}

pub enum Import {
    Everything,
    Item(String),
}
