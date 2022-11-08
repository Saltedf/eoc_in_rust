use crate::ast::*;



pub trait Interp {
    fn interp_exp(env: &Env ,e: &Exp) -> std::io::Result<i64>;
    fn interp_program(p: &Program) -> std::io::Result<i64>;
}

pub struct InterpLint {}

impl InterpLint {
    pub fn new() -> Self{
        Self{
        }
    }
}

impl Interp for InterpLint {
    fn interp_exp(env:&Env, e: &Exp) -> std::io::Result<i64> {
        let res = match e {
            Exp::Int(n) => n.clone(),
            Exp::Prim {
                op: PrimType::Read, ..
            } => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf)?;
                buf.pop(); // 移除末尾的'\n'
                buf.parse::<i64>().expect("read expected an integer.")
            }
            Exp::Prim {
                op: PrimType::Neg,
                args,
            } => {
                let e = args.get(0).unwrap();
                0 - Self::interp_exp(env,e)?
            }
            Exp::Prim {
                op: PrimType::Add,
                args,
            } => {
                let e1 = Self::interp_exp(env,args.get(0).unwrap())?;
                let e2 = Self::interp_exp(env,args.get(1).unwrap())?;
                e1 + e2
            }
            Exp::Prim {
                op: PrimType::Sub,
                args,
            } => {
                let e1 = Self::interp_exp(env,args.get(0).unwrap())?;
                let e2 = Self::interp_exp(env,args.get(1).unwrap())?;
                e1 - e2
            }
            o => InterpLvar::interp_exp(env,o)?,
        };

        Ok(res)
    }

    fn interp_program(p: &Program) -> std::io::Result<i64> {
        //        let info = &p.info;
        let body = &p.body;
        let env = &p.env;
        let res = Self::interp_exp(&env,body)?; 

        Ok(res)
    }
}


pub struct InterpLvar {
 //   lint: InterpLint,
}

impl InterpLvar{
    pub fn new() -> Self{
        Self{
//            lint : InterpLint::new(),
        }
    }
}


impl Interp for InterpLvar {

     fn interp_exp(env:&Env,e: &Exp) -> std::io::Result<i64> {
         let res = match e {
             Exp::Var(name)  => {
                 env.get(name).unwrap().clone()
             },
             Exp::Let { var, binding, body } => {
                 let mut new_env = env.clone();
                 new_env.insert(var.clone(), Self::interp_exp(env,binding.as_ref())?);
                 Self::interp_exp(&mut new_env,body.as_ref())?
             },
             other => InterpLint::interp_exp(env,other)? 

         };
         Ok(res)
     }


    fn interp_program(p: &Program) -> std::io::Result<i64> {
        InterpLint::interp_program(p)
    }

    
}
