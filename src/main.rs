lalrpop_mod!(l_int);
mod ast;

use std::fs::read_to_string;
use lalrpop_util::lalrpop_mod;
use ast::*;

fn interp_exp(e : &ast::Exp) -> std::io::Result<i64> {
    
    let res =   match e {
        Exp::Int(n) => n.clone(),
        Exp::Prim { op:PrimType::Read, .. } =>{
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf)?;
            buf.pop();// 移除末尾的'\n'
            buf.parse::<i64>().expect("read expected an integer.")
        },
        Exp ::Prim { op:PrimType::Neg, args } => {
            let e = args.get(0).unwrap();
            0 - interp_exp(e)?
        },
        Exp ::Prim { op:PrimType::Add, args } => {
            let e1 = interp_exp( args.get(0).unwrap())?;
            let e2 = interp_exp( args.get(1).unwrap())?;
            e1 + e2
        },
        Exp ::Prim { op:PrimType::Sub, args } => {
            let e1 = interp_exp( args.get(0).unwrap())?;
            let e2 = interp_exp( args.get(1).unwrap())?;
            e1 - e2
        },
    };
    
    Ok(res)
}

fn interp_lint(p: &ast::Program) -> std::io::Result<()> {
    let info  = &p.info;
    let body = &p.body;
    
    let res =  interp_exp(body)?;
    println!(">> {}",res);
    
    Ok(())
}



fn main() -> std::io::Result<()>{


    let mut args = std::env::args();
    args.next();
    let inputfile = args.next().unwrap();
    let input = read_to_string(inputfile)?;


    let ast = l_int::LintParser::new().parse(&input).unwrap();
    println!("{:#?}", ast);
    
    interp_lint(&ast)?;

    Ok(())
}




mod tests{
    use super::*;


}
