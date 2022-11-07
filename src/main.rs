lalrpop_mod!(l_int);
mod ast;

use std::{fs::read_to_string, io::Stdin};
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



fn pe_neg(r : &ast::Exp) ->ast::Exp {
    match r {
        Exp::Int(n) => Exp::Int(0-n),
        sub => Exp::Prim { op: PrimType::Neg, args: vec![sub.clone()] },
    }
}

fn pe_add(r1:&ast::Exp,r2 : &ast::Exp) -> ast::Exp {
    match (r1,r2) {
        (Exp::Int(n1),Exp::Int(n2)) => Exp::Int(n1+n2),
        (_,_) => Exp::Prim { op: PrimType::Add, args: vec![r1.clone(),r2.clone()] },
    }
}

fn pe_sub(r1:&ast::Exp,r2 : &ast::Exp) -> ast::Exp {
    match (r1,r2) {
        (Exp::Int(n1),Exp::Int(n2)) => Exp::Int(n1-n2),
        (_,_) => Exp::Prim { op: PrimType::Sub, args: vec![r1.clone(),r2.clone()] },
    }
}

fn pe_exp(e: &ast::Exp) -> ast::Exp {
    match e {
        i@ Exp::Int(_) => i.clone(),
        rd @ Exp::Prim { op: PrimType::Read,.. } =>  rd.clone(),
        Exp::Prim { op: PrimType::Neg, args} =>  pe_neg(&pe_exp(&args[0])),
        Exp::Prim { op: PrimType::Add, args} =>  pe_add(&pe_exp(&args[0]),&pe_exp(&args[1])),
        Exp::Prim { op: PrimType::Sub, args} =>  pe_sub(&pe_exp(&args[0]),&pe_exp(&args[1])), 
    }
}

fn pe_lint(p: &ast::Program) -> ast::Program {
    
    ast::Program{
        info : p.info.clone() ,
        body : pe_exp(&p.body),
    }
}

mod tests{
    use super::*;

    fn test_pe(p : &ast::Program ){
        assert_eq!(interp_lint(p).unwrap(), interp_lint(&pe_lint(p)).unwrap())
    }
    
    #[test]
    fn test_pe_lint(){
        let p1 = l_int::LintParser::new().parse("(+ 10 (- (+ 5 3)))").unwrap();
        let p2 = l_int::LintParser::new().parse("(+ 1 (+ 3 1))").unwrap();
        let p3 = l_int::LintParser::new().parse("(- (+ 3 (- 5)))").unwrap();

        test_pe(&p1);
        test_pe(&p2);
        test_pe(&p3);
    }

}
