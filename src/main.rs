lalrpop_mod!(l_var);
mod ast;
mod interp;
mod x86;
mod clike;
mod passes; 

use std::fs::read_to_string;
use lalrpop_util::lalrpop_mod;

use crate::interp::{InterpLvar, Interp};




fn main() -> std::io::Result<()>{


    let mut args = std::env::args();
    args.next();
    let inputfile = args.next().unwrap();
    let input = read_to_string(inputfile)?;

    let ast = l_var::LvarParser::new().parse(&input).unwrap();
    println!("{:#?}", ast);
    println!("======================================");
    
    let res  =  InterpLvar::interp_program(&ast)?;
    println!(">> {}", res);

    Ok(())
}




mod tests{
    use super::*;


}
