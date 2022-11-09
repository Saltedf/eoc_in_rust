use std::collections::HashMap;



#[derive(Debug,PartialEq, Eq)]
pub enum Reg{
    Rsp,
    Rbp,
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}



#[derive(Debug,PartialEq, Eq)]
pub enum Arg{
    Imm(i64),
    Reg(Reg),
    Deref(Reg,i64),
}

#[derive(Debug,PartialEq, Eq)]
pub enum OpCode{
    Addq,
    Subq,
    Negq,
    Movq,
    Pushq,
    Popq,
}

#[derive(Debug,PartialEq, Eq)]
pub enum Instr{

    Ins{op:OpCode,args:Vec<Arg>},
    Callq{label:String,count:u64},
    Retq,
    Jmp(String)

}



#[derive(Debug,PartialEq, Eq)]
pub struct Block{
    info : HashMap<String,String>,
    instrs: Vec<Instr>,
}



#[derive(Debug,PartialEq, Eq)]
pub struct X86Program {
    info : HashMap<String,String>,
    blocks: Vec<(String,Block)>,
}
