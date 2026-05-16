#[derive(Debug)]
enum Token {
    Num(u64),
    Plus, Minus, Star, Slash,
}

/// Parsed
#[derive(Debug)]
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

#[derive(Debug)]
enum Op { Add, Subs, Mul, Div, Neg }

fn parse_mul_level(mut tokens: &[Token]) -> Box<Expr> {
    assert!(!tokens.is_empty());
         
    let mut i = 0;

    let Token::Num(first) = tokens[0] else { panic!() };

    let mut tree = Box::new(Expr::Num(first));

    tokens = &tokens[1..];
        
    loop {
        println!("loop: {:?}", tokens);

        if tokens.is_empty() {
            return tree;
        }

        match tokens[0] {
            Token::Plus | Token::Minus => {return tree;}
            Token::Star | Token::Slash => {
                println!("shift");
                let Token::Num(another_num) = tokens[1] else {return tree;};
                // remove boxes
                tree = Box::new(Expr::Binary { op: Box::new(Op::Mul), left: tree, right: Box::new(Expr::Num(another_num)) });
            tokens = &tokens[2..];
            }
            _ => {return tree;}
        }

        i += 1;
    }
}

fn main() {
    use Token::*;

    let lexxed = vec![Num(2), Star, Num(3), Star, Num(4)];

    let tree = parse_mul_level(&lexxed);

    println!("{:?}", *tree);

}
