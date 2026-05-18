use std::unreachable;

#[derive(Debug, Clone, Copy)]
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

fn parse_mul_level(mut tokens: &[Token]) -> (Box<Expr>, &[Token]) {
    assert!(!tokens.is_empty());
         
    let mut i = 0;

    println!("parsing mul level: {:?}", tokens);
    let Token::Num(first) = tokens[0] else { panic!() };

    let mut tree = Box::new(Expr::Num(first));

    tokens = &tokens[1..];
        
    loop {
        println!("mul loop: {:?}", tokens);

        if tokens.is_empty() {
            return (tree, tokens);
        }

        match tokens[0] {
            Token::Plus | Token::Minus => {return (tree, tokens);}
            Token::Star | Token::Slash => {
                println!("shift");
                let Token::Num(another_num) = tokens[1] else {return (tree, tokens);};
                // remove boxes
                tree = Box::new(Expr::Binary { op: Op::Mul, left: tree, right: Box::new(Expr::Num(another_num)) });
            tokens = &tokens[2..];
            }
            _ => {return (tree, tokens);}
        }

        i += 1;
    }
}

fn parse_add_level2(tokens: &[Token]) -> (Box<Expr>, &[Token]) {
    assert!(!tokens.is_empty());

    // NOTE: every level can just stop if it encounters a symbol not from its level.

    // Parse level higher
    let (mut tree, mut tokens) = parse_mul_level(tokens);

    while !tokens.is_empty() {
        println!("add loop: {:?}", tokens);

        // Parse RHS level higher
        println!("sending to parse mul: {:?}", &tokens[1..]);
        let (newtree, newtokens) = parse_mul_level(&tokens[1..]);
        
        println!("Tokens should be a + i guess here? but {:?}", tokens);

        match tokens[0] {
            Token::Minus => { tree = Box::new(Expr::Binary { op: Op::Subs, left: tree, right: newtree }); }
            Token::Plus => { tree = Box::new(Expr::Binary { op: Op::Add, left: tree, right: newtree }); }
            _ => {unreachable!("All higher levels should've been parsed. Encountered: {:?}", tokens[0])}
        }

        tokens = newtokens;

    }

    (tree, tokens)
}

fn parse_add_level(mut tokens: &[Token]) -> (Box<Expr>, &[Token]) {
    assert!(!tokens.is_empty());

    // Parse level higher
    let (mut tree, mut tokens) = parse_mul_level(tokens);
        
    while !tokens.is_empty() {        
        println!("add loop: {:?}", tokens);
        
        assert!(matches!(tokens[0], Token::Plus | Token::Minus), "All higher level tokens should've been parsed. Encountered: {:?}", tokens[0]);

        tokens = &tokens[1..];

        let Token::Num(number) = tokens[0] else { panic!("not a number but: {:?}", tokens[0]) };

        // Look ahead
        match tokens[1] {
            // Defer to level higher
            Token::Star | Token::Slash => {
                (tree, tokens) = parse_mul_level(&tokens[2..]);

                tree = Box::new(Expr::Binary { op: Op::Add, left: Box::new(Expr::Num(number)), right: tree });

                println!("after lookahead mul parsing: {:?}", tokens);
            }

            // Continue normally
            _ => {}

        }

        println!("before match: {:?}", tokens);

        match tokens[0] {
            Token::Plus | Token::Minus => {
                let Token::Num(another_num) = tokens[1] else {return (tree, tokens);};
                tree = Box::new(Expr::Binary { op: Op::Add, left: tree, right: Box::new(Expr::Num(another_num)) });
                tokens = &tokens[2..];
            }
            _ => {println!("returning from match because {:?}", tokens[0]); return (tree, tokens);}
        }
    }
    (tree, tokens)
}

// 2 + 3 * 2
// 2 + 3 + 4

fn main() {
    use Token::*;

    // let lexxed = vec![Num(2), Star, Num(3), Star, Num(4)];


    // 2 + 3 * 4 + 1
    // let lexxed = vec![Num(2), Plus, Num(3), Star, Num(4), Plus, Num(1)];

    let lexxed = vec![Num(2), Plus, Num(3), Plus, Num(4)];
    // let lexxed = vec![Num(2)];

    let (tree, _) = parse_add_level2(&lexxed);

    println!("{:?}", tree);

}
