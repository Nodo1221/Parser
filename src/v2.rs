#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Num(u64),
    Plus, Minus, Star, Slash,
}

/// Parsed
#[derive(Debug)]
enum Expr {
    Num(u64),
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: Op,
        left: Box<Expr>,
    }
}

#[derive(Debug)]
enum Op { Add, Subs, Mul, Div, Neg }

const LEVELS: [&[Token]; 2] = [
    &[Token::Plus, Token::Minus],
    &[Token::Star, Token::Slash],
];

// fn unit_parse(mut tokens: &[Token], level: usize) -> (Box<Expr>, &[Token]) {

// }

fn parse(tokens: &[Token], level: usize) -> (Box<Expr>, &[Token]) {
    assert!(!tokens.is_empty());

    if level == LEVELS.len() {
        let Token::Num(n) = tokens[0] else { unreachable!("expected Num token") };
        return (Box::new(Expr::Num(n)), &tokens[1..]);
    }

    let (mut tree, mut tokens) = parse(&tokens, level + 1);

    while !tokens.is_empty() {
        if !LEVELS[level].contains(&tokens[0]) {
            return (tree, tokens);
        }

        let (newtree, newtokens) = parse(&tokens[1..], level + 1);

        let op = match tokens[0] {
            Token::Plus => Op::Add,
            Token::Minus => Op::Subs,
            Token::Star => Op::Mul,
            Token::Slash => Op::Div,
            _ => unreachable!("Expected Op"),
        };

        tree = Box::new(Expr::Binary {
            op,
            left: tree,
            right: newtree
        });

        tokens = newtokens;
    }

    // unreachable!();
    return (tree, tokens);
    // return (Box::new(Expr::Num(4)), &tokens[1..]);
}

fn main() {
    use Token::*;

    // let lexxed = vec![Num(2), Plus, Num(3), Plus, Num(4)];
    // let lexxed = vec![Num(2), Star, Num(3), Star, Num(4)];
    let lexxed = vec![Num(2), Plus, Num(3), Star, Num(4), Plus, Num(1)];

    let (tree, _) = parse(&lexxed, 0);

    println!("{:?}", tree);
}