use crate::ast;
use std::collections::HashMap;

// #[derive(Debug,PartialEq, Eq)]
// pub enum Atom{
//     Int(i64),
//     Var(String),
// }

// #[derive(Debug,PartialEq, Eq)]
// pub enum OpCode {
//     Read,
//     Add,
//     Sub,
//     Neg,
// }


// #[derive(Debug,PartialEq, Eq)]
// pub enum Exp {
//     Atm(Atom),
//     Prim{op:OpCode , atms:Vec<Atom>}
// }

#[derive(Debug,PartialEq, Eq)]
pub enum Stmt {
    Assign{var:String, exp:ast::Exp },
}

#[derive(Debug,PartialEq, Eq)]
pub enum Tail {
    Return(ast::Exp),
    Seq{stmt:Stmt,tail:Box<Tail>},
}

#[derive(Debug,PartialEq, Eq)]
pub struct CProgram{
    info : HashMap<String, String>,
    body : Vec<(String,Tail)>,
}



