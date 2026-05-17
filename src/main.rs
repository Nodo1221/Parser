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

fn parse_add_level(mut tokens: &[Token]) -> (Box<Expr>, &[Token]) {
    assert!(!tokens.is_empty());
         
    let mut i = 0;


    // let Token::Num(first) = tokens[0] else { panic!() };

    let (mut tree, mut tokens) = parse_mul_level(tokens);

    // tokens = &tokens[1..];
        
    loop {
        println!("add loop: {:?}", tokens);

        if tokens.is_empty() {
            return (tree, tokens);
        }

        let Token::Plus = tokens[0] else { panic!("not a plus mate {:?}", tokens[0]) };
        tokens = &tokens[1..];

        let Token::Num(number) = tokens[0] else { panic!("not a number but: {:?}", tokens[0]) };

        // look ahead:
        match tokens[1] {
            Token::Plus | Token::Minus => {
                //continue normally
            }
            Token::Star | Token::Slash => {
                // let newtree: Vec<&[Token]> = &tokens[0..1].collect();
                // let newtree = vec![tokens[0].clone()];


                (tree, tokens) = parse_mul_level(&tokens[2..]);

                tree = Box::new(Expr::Binary { op: Op::Add, left: Box::new(Expr::Num(number)), right: tree });

                // newtree.push(tokens);

                println!("after lookahead mul parsing: {:?}", tokens);
            }
            _ => unreachable!("{:?}", tokens[2]),

        }

        match tokens[0] {
            Token::Plus | Token::Minus => {
                let Token::Num(another_num) = tokens[1] else {return (tree, tokens);};
                tree = Box::new(Expr::Binary { op: Op::Add, left: tree, right: Box::new(Expr::Num(another_num)) });
                tokens = &tokens[2..];
            }
            // Token::Star | Token::Slash => {
            //     println!("shift");
            //     let Token::Num(another_num) = tokens[1] else {return tree;};
            //     // remove boxes
            //     tree = Box::new(Expr::Binary { op: Op::Mul, left: tree, right: Box::new(Expr::Num(another_num)) });
            // tokens = &tokens[2..];
            // }
            _ => {return (tree, tokens);}
        }

        i += 1;
    }
}

fn main() {
    use Token::*;

    // let lexxed = vec![Num(2), Star, Num(3), Star, Num(4)];
    let lexxed = vec![Num(2), Plus, Num(3), Star, Num(4), Plus, Num(1)];
    // let lexxed = vec![Num(2)];

    let (tree, _) = parse_add_level(&lexxed);

    println!("{:?}", tree);

}
