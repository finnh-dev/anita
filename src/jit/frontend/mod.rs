#[derive(Debug)]
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

impl Expr {
    #[allow(unused)]
    fn to_string(&self) -> String {
        match self {
            Expr::VariableRead { identifier } => format!("{identifier}"),
            Expr::Const { value } => format!("{value}"),
            Expr::Chain { side, ret } => format!("({}); ({})", side.to_string(), ret.to_string()),
            Expr::Call { identifier, args } => {
                format!(
                    "{identifier}({})",
                    args.iter()
                        .fold(String::new(), |acc, expr| acc + "(" + &expr.to_string() + ")" + ", ")
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
    fn debug() {
        let expr = "!a + b";
        match parser::expression(expr) {
            Ok(parsed) => {
                println!("{}", expr);
                println!("{}", parsed.to_string());
                println!("{:#?}", parsed)
            },
            Err(err) => println!("{}", err.to_string()),
        }

        assert!(false)
    }
}
