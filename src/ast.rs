use std::collections::HashMap;

pub type Env = HashMap<String,i64>;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub info: HashMap<String, String>,
    pub env: Env, 
    pub body: Exp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimType {
    Read,
    Add,
    Sub,
    Neg,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp {
    Int(i64),
    Prim { op: PrimType, args: Vec<Exp> },
    Var(String),
    Let {var:String,binding: Box<Exp>,body:Box<Exp>},
}



