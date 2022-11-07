use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Program {

    pub    info : HashMap<String,String>,
    pub    body : Exp, 

}

#[derive(Debug,Clone,PartialEq, Eq)]
pub enum PrimType{

    Read,
    Add,
    Sub,
    Neg,
}


#[derive(Debug,Clone,PartialEq, Eq)]
pub enum Exp {
    Int(i64),
    Prim{ op: PrimType, args: Vec<Exp> },
}
