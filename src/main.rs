enum Token {
    Num(u64),
    Plus, Minus, Star, Slash,
}

/// Parsed
enum Expr {
    Num(u64),
    Binary {
        op: Box<Op>,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: Box<Op>,
        left: Box<Expr>,
    }
}

enum Op { Add, Subs, Mul, Div, Neg }

fn parse_mul_level(mut tokens: &[Token]) -> Box<Expr> {
    let mut i = 0;

    let Token::Num(first) = tokens[0] else { return; };

    let mut tree = Box::new(Expr::Num(first));
        
    loop {
        if tokens.is_empty() {
            return tree;
        }

        match tokens[1] {
            Token::Plus | Token::Minus => {return;}
            Token::Star | Token::Slash => {
                let Token::Num(another_num) = tokens[2] else {return;};
                // remove boxes
                tree = Box::new(Expr::Binary { op: Box::new(Op::Mul), left: tree, right: Box::new(Expr::Num(another_num)) });
            tokens = &tokens[2..];
            }
            _ => {return;}
        }

        i += 1;
    }
}

fn main() {
    use Token::*;

    let lexxed = vec![Num(2), Plus, Num(3), Star, Num(2)];

}
