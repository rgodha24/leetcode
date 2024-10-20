use crate::Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let (expr, _) = Expr::parse(&expression);
        dbg!(&expr);
        expr.collapse()
    }
}

#[derive(Debug, Clone)]
enum Expr {
    And(Vec<Expr>),
    Or(Vec<Expr>),
    Not(Box<Expr>),
    Bool(bool),
}

impl Expr {
    fn collapse(self) -> bool {
        match self {
            Self::And(exp) => exp.into_iter().all(|e| e.collapse()),
            Self::Or(exp) => exp.into_iter().any(|e| e.collapse()),
            Self::Not(exp) => !exp.collapse(),
            Self::Bool(b) => b,
        }
    }
    fn parse<'a>(mut s: &'a str) -> (Self, &'a str) {
        match s.as_bytes()[0] {
            b',' => panic!("comma in parse. s = {s}"),
            b')' => panic!("close paren in parse. s = {s}"),
            b'&' => {
                s = &s[2..];
                let mut v = Vec::new();
                loop {
                    let (parsed, new_s) = Expr::parse(s);
                    s = new_s;
                    v.push(parsed);
                    match s.as_bytes()[0] {
                        b')' => return (Expr::And(v), &s[1..]),
                        b',' => {
                            s = &s[1..];
                        }
                        b => panic!("invalid char in expr {}", b as char),
                    }
                }
            }
            b'|' => {
                s = &s[2..];
                let mut v = Vec::new();
                loop {
                    let (parsed, new_s) = Expr::parse(s);
                    s = new_s;
                    v.push(parsed);
                    match s.as_bytes()[0] {
                        b')' => return (Expr::Or(v), &s[1..]),
                        b',' => {
                            s = &s[1..];
                        }
                        b => panic!("invalid char in expr {}", b as char),
                    }
                }
            }
            b'!' => {
                let (exp, s) = Expr::parse(&s[2..]);
                (Expr::Not(Box::new(exp)), &s[1..])
            }
            b'f' => (Expr::Bool(false), &s[1..]),
            b't' => (Expr::Bool(true), &s[1..]),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test1() {
        let cases = vec![("|(f,f,f,t)", true), ("&(|(f))", false)];
        for (expr, b) in cases {
            assert_eq!(b, Solution::parse_bool_expr(expr.to_owned()))
        }
    }
}
