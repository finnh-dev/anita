use std::{collections::HashSet, vec};

#[derive(Debug, Clone)]
pub enum Expr {
    VariableRead {
        identifier: String,
    },
    Const {
        value: f32,
    },
    Chain {
        side: Box<Expr>,
        ret: Box<Expr>,
    },
    Call {
        identifier: String,
        args: Vec<Expr>,
    },
    Add {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Sub {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Mul {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Div {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Mod {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Exp {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Neg {
        value: Box<Expr>,
    },
    Assign {
        identifier: String,
        value: Box<Expr>,
    },
    Eq {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Neq {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Gt {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Lt {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Geq {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Leq {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    And {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Or {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Not {
        value: Box<Expr>,
    },
}

pub struct Variables {
    pub read: Vec<String>,
    pub write: Vec<String>,
    pub uninitialized: Vec<String>,
}

pub struct NonEmptyUninitializedVariables(Box<[String]>);

impl Variables {
    pub fn set_defined(&mut self, identifier: &String) {
        self.uninitialized = self
            .uninitialized
            .clone()
            .into_iter()
            .filter(|e| e != identifier)
            .collect();
    }

    pub fn initialized_identifiers(mut self) -> Result<Box<[String]>, Box<[String]>> {
        if !self.uninitialized.is_empty() {
            return Err(self.uninitialized.into());
        }
        let mut identifiers = Vec::new();
        identifiers.append(&mut self.read);
        identifiers.append(&mut self.write);
        let unique_identifiers = identifiers.into_iter().collect::<HashSet<String>>().into_iter().collect();
        Ok(unique_identifiers)
    }
}



impl Expr {
    

    pub fn variables(&self) -> Variables {
        let read = self.variables_read();
        let write = self.variables_write();
        let undefined: Vec<String> = self
            .variables_all()
            .iter()
            .filter_map(|e| {
                if !write.contains(e) {
                    Some(e.to_string())
                } else {
                    None
                }
            })
            .collect();
        Variables {
            read,
            write,
            uninitialized: undefined,
        }
    }

    fn variables_write(&self) -> Vec<String> {
        match self {
            Expr::VariableRead { identifier: _ } => Vec::new(),
            Expr::Const { value: _ } => Vec::new(),
            Expr::Chain { side, ret } => {
                let mut v = side.variables_write();
                v.extend(ret.variables_write());
                v
            }
            Expr::Call {
                identifier: _,
                args,
            } => {
                let mut v = Vec::new();
                for arg in args {
                    v.extend(arg.variables_write());
                }
                v
            }
            Expr::Add { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Sub { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Mul { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Div { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Mod { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Exp { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Neg { value } => value.variables_write(),
            Expr::Assign { identifier, value } => {
                let mut v = value.variables_write();
                v.insert(0, identifier.to_string());
                v
            }
            Expr::Eq { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Neq { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Gt { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Lt { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Geq { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Leq { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::And { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Or { lhs, rhs } => {
                let mut v = lhs.variables_write();
                v.extend(rhs.variables_write());
                v
            }
            Expr::Not { value } => value.variables_write(),
        }
    }

    fn variables_read(&self) -> Vec<String> {
        match self {
            Expr::VariableRead { identifier } => vec![identifier.to_string()],
            Expr::Const { value: _ } => Vec::new(),
            Expr::Chain { side, ret } => {
                let mut v = side.variables_read();
                v.extend(ret.variables_read());
                v
            }
            Expr::Call {
                identifier: _,
                args,
            } => {
                let mut v = Vec::new();
                for arg in args {
                    v.extend(arg.variables_read());
                }
                v
            }
            Expr::Add { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Sub { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Mul { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Div { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Mod { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Exp { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Neg { value } => value.variables_read(),
            Expr::Assign {
                identifier: _,
                value,
            } => value.variables_read(),
            Expr::Eq { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Neq { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Gt { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Lt { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Geq { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Leq { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::And { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Or { lhs, rhs } => {
                let mut v = lhs.variables_read();
                v.extend(rhs.variables_read());
                v
            }
            Expr::Not { value } => value.variables_read(),
        }
    }

    fn variables_all(&self) -> Vec<String> {
        match self {
            Expr::VariableRead { identifier } => vec![identifier.to_string()],
            Expr::Const { value: _ } => Vec::new(),
            Expr::Chain { side, ret } => {
                let mut v = side.variables_all();
                v.extend(ret.variables_all());
                v
            }
            Expr::Call {
                identifier: _,
                args,
            } => {
                let mut v = Vec::new();
                for arg in args {
                    v.extend(arg.variables_all());
                }
                v
            }
            Expr::Add { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Sub { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Mul { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Div { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Mod { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Exp { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Neg { value } => value.variables_all(),
            Expr::Assign { identifier, value } => {
                let mut v = value.variables_all();
                v.insert(0, identifier.to_string());
                v
            }
            Expr::Eq { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Neq { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Gt { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Lt { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Geq { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Leq { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::And { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Or { lhs, rhs } => {
                let mut v = lhs.variables_all();
                v.extend(rhs.variables_all());
                v
            }
            Expr::Not { value } => value.variables_all(),
        }
    }

    #[allow(unused)]
    fn to_string(&self) -> String {
        match self {
            Expr::VariableRead { identifier } => format!("{identifier}"),
            Expr::Const { value } => format!("{value}"),
            Expr::Chain { side, ret } => format!("({}); ({})", side.to_string(), ret.to_string()),
            Expr::Call { identifier, args } => {
                format!(
                    "{identifier}({})",
                    args.iter().fold(String::new(), |acc, expr| acc
                        + "("
                        + &expr.to_string()
                        + ")"
                        + ", ")
                )
            }
            Expr::Add { lhs, rhs } => format!("({}) + ({})", lhs.to_string(), rhs.to_string()),
            Expr::Sub { lhs, rhs } => format!("({}) - ({})", lhs.to_string(), rhs.to_string()),
            Expr::Mul { lhs, rhs } => format!("({}) * ({})", lhs.to_string(), rhs.to_string()),
            Expr::Div { lhs, rhs } => format!("({}) / ({})", lhs.to_string(), rhs.to_string()),
            Expr::Mod { lhs, rhs } => format!("({}) % ({})", lhs.to_string(), rhs.to_string()),
            Expr::Exp { lhs, rhs } => format!("({}) ^ ({})", lhs.to_string(), rhs.to_string()),
            Expr::Neg { value } => format!("-{}", value.to_string()),
            Expr::Assign { identifier, value } => format!("{identifier} = ({})", value.to_string()),
            Expr::Eq { lhs, rhs } => format!("({}) == ({})", lhs.to_string(), rhs.to_string()),
            Expr::Neq { lhs, rhs } => format!("({}) != ({})", lhs.to_string(), rhs.to_string()),
            Expr::Gt { lhs, rhs } => format!("({}) > ({})", lhs.to_string(), rhs.to_string()),
            Expr::Lt { lhs, rhs } => format!("({}) < ({})", lhs.to_string(), rhs.to_string()),
            Expr::Geq { lhs, rhs } => format!("({}) >= ({})", lhs.to_string(), rhs.to_string()),
            Expr::Leq { lhs, rhs } => format!("({}) <= ({})", lhs.to_string(), rhs.to_string()),
            Expr::And { lhs, rhs } => format!("({}) && ({})", lhs.to_string(), rhs.to_string()),
            Expr::Or { lhs, rhs } => format!("({}) || ({})", lhs.to_string(), rhs.to_string()),
            Expr::Not { value } => format!("!({})", value.to_string()),
        }
    }
}

peg::parser!(pub grammar parser() for str {
    pub rule expression() -> Expr
    = precedence!{
        s:@ _ ";" _ r:(@) { Expr::Chain { side: Box::new(s), ret: Box::new(r) } }
        --
        l:literal() { Expr::Const { value: l } }
        i:identifier() _ "=" _ e:(@) { Expr::Assign { identifier: i, value: Box::new(e) }}
        --
        a:(@) _ "&&" _ b:@ { Expr::And{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ "||" _ b:@ { Expr::Or{ lhs: Box::new(a), rhs: Box::new(b) } }
        --
        a:(@) _ "==" _ b:@ { Expr::Eq{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ "!=" _ b:@ { Expr::Neq{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ ">"  _ b:@ { Expr::Gt{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ ">=" _ b:@ { Expr::Geq{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ "<"  _ b:@ { Expr::Lt{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ "<=" _ b:@ { Expr::Leq{ lhs: Box::new(a), rhs: Box::new(b) } }
        --
        a:(@) _ "+" _ b:@ { Expr::Add{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ "-" _ b:@ { Expr::Sub{ lhs: Box::new(a), rhs: Box::new(b) } }
        --
        a:(@) _ "*" _ b:@ { Expr::Mul{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ "/" _ b:@ { Expr::Div{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ "%" _ b:@ { Expr::Mod{ lhs: Box::new(a), rhs: Box::new(b) } }
        a:(@) _ "^" _ b:@ { Expr::Exp{ lhs: Box::new(a), rhs: Box::new(b) } }
        --
        "!" a:@ { Expr::Not{ value: Box::new(a) } }
        "-" a:@  { Expr::Neg { value: Box::new(a) } }
        "(" _ e:expression() _ ")" { e }
        --
        i:identifier() _ "(" args:((_ e:expression() _ {e}) ** ",") ")" { Expr::Call { identifier: i, args } }
        i:identifier() { Expr::VariableRead { identifier: i }}
    }

    rule identifier() -> String
    = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
    / expected!("identifier")

    rule literal() -> f32
    = n:$("-"?['0'..='9']+("."['0'..='9']*)?) {? n.parse().or(Err("f32"))}

    rule _() =  quiet!{[' ' | '\t']*}
});

#[cfg(test)]
mod tests {
    use super::parser;

    #[test]
    #[ignore = "just for debugguing"]
    fn debug() {
        let expr = "!a + b";
        match parser::expression(expr) {
            Ok(parsed) => {
                println!("{}", expr);
                println!("{}", parsed.to_string());
                println!("{:#?}", parsed)
            }
            Err(err) => println!("{}", err.to_string()),
        }

        assert!(false)
    }
}
