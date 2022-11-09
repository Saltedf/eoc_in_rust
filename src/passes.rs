use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
//pub type Env = HashMap<String,i64>;


pub struct Env<V> {
    last_insert: VecDeque<String>,
    table : HashMap<String, BTreeMap<usize, V>>,
    symgen:  SymbolGen,
}

impl<V> Env<V> {
    pub fn new() -> Self {
        Self {
            last_insert : VecDeque::new(),
            table : HashMap::new(),
            symgen: SymbolGen::new(),
        }
        
    }

    pub fn insert<S: AsRef<str>>(&mut self,var:S, binding:V)-> String  {
        self.last_insert.push_front(var.as_ref().to_string());
        
        let (base,post ) =
            if let Some((name, p)) = var.as_ref().split_once(".") {
                (name,Some(p.parse::<usize>().unwrap()))
            }else {
                (var.as_ref(), None)
            };

        let tmp = match self.table.get_mut(base) {
            Some(m) => m,
            None => {
                self.table.insert(base.to_string(), BTreeMap::new());
                self.table.get_mut(base).unwrap()
            },
        };
        
        if let Some(p) = post {
            tmp.insert(p, binding);
            return Symbol::new(base,p).fullname()
        } else {
            let sym = self.symgen.gensym(var);
            self.table.get_mut(&sym.base).unwrap().insert(sym.postfix,binding);
            return sym.fullname()
        }
        
    }

    pub fn remove_last(&mut self) {
        let last = self.last_insert.pop_front().unwrap();
        self.remove(last);
    }

    
    fn remove<S: AsRef<str>>(&mut self, var : S)  -> Option<V> {

        if let Some((b, i)) = var.as_ref().split_once(".") {
            let tmp = self.table
                .get_mut(b)
                .unwrap();
            let post: usize = i.parse().unwrap();
            tmp.remove(&post)

        }else {
            let tmp = self.table
                .get_mut(var.as_ref())
                .unwrap();
            let last =tmp.keys().last().map(|e|e.clone()).unwrap();  
            tmp.remove(&last)
        }


    }

    pub fn get<S: AsRef<str>>(&self, var: S) -> Option<(String,&V)> {
       if let Some((b, i)) = var.as_ref().split_once(".") {
            
           let post: usize = i.parse().unwrap();
           self.table.get(b).unwrap().get(&post).map(|e|
                                               (var.as_ref().to_string(),
                                                            e)
           )
           
        } else {
            
           self.table
                .get(var.as_ref())
                .unwrap()
                .iter()
                .last()
               .map(|e|
                    (format!("{}.{}", var.as_ref(),e.0),e.1 )
               )
       }

        
        
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub base: String,
    postfix: usize,
}

impl Symbol {
    pub fn new<S: AsRef<str>>(var: S,postfix:usize) -> Self {
        Self {
            base: var.as_ref().to_string(),
            postfix: postfix,
        }
    }

    pub fn fullname(&self) -> String {
        format!("{}.{}", self.base, self.postfix)
    }
}

pub struct SymbolGen {
    index: usize,
}

impl SymbolGen {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn gensym<S: AsRef<str>>(&mut self, id: S) -> Symbol {
        self.index += 1;
        Symbol {
            base: id.as_ref().to_string(),
            postfix: self.index - 1,
        }
    }
}

mod tests {
    use std::collections::BTreeMap;
    use super::*;
    
    #[test]
    fn btree() {
        let mut t = BTreeMap::new();
        t.insert(4, "aa");
        t.insert(2, "vv");
        t.insert(9, "ss");

        eprintln!("{:?}", t.keys().collect::<Vec<_>>());
    }


    #[test]
    fn symenv(){
        let mut  env  = Env::new();
        env.insert("x",123);
        env.insert("y",234);
        env.insert("x",455);

        eprintln!("{:?}", env.get("x"));
//        env.remove_last();
        eprintln!("{:?}", env.get("x"));

        eprintln!("y={:?}", env.get("y"));
        
    }
    
}
