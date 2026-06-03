#[derive(Debug)]
#[allow(dead_code)]
enum ParseError {
    UnexpectedToken(Token)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Num(u64),
    Plus, Minus, Star, Slash,
    Lpar, Rpar,
}

impl TryFrom<&Token> for Op {
    type Error = ParseError;

    fn try_from(token: &Token) -> Result<Op, ParseError> {
        Ok(match token {
            Token::Plus => Op::Add,
            Token::Minus => Op::Subs,
            Token::Star => Op::Mul,
            Token::Slash => Op::Div,
            _ => return Err(ParseError::UnexpectedToken(*token)),
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
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

fn parse(tokens: &[Token], level: usize) -> (Box<Expr>, &[Token]) {
    assert!(!tokens.is_empty());

    // Highest precedence level (atomic)
    if level == LEVELS.len() {
        return match tokens[0] {
            Token::Num(n) => (Box::new(Expr::Num(n)), &tokens[1..]),
            Token::Minus => match tokens[1] {
                Token::Num(n) => (Box::new(Expr::Unary { op: Op::Neg, left: Box::new(Expr::Num(n)) }), &tokens[2..]),
                _ => unreachable!("Expected Num after Minus"),
            },

            // Handle parenthesis (match lpar, restart parsing from the lowest prededence, then look for rpar)
            Token::Lpar => {
                let (tree, tokens) = parse(&tokens[1..], 0);
                assert!(matches!(tokens[0], Token::Rpar), "Expected ')' Rpar");
                (tree, &tokens[1..])
            }
            _ => unreachable!("Expected Num or Minus, tokens: {:?}", tokens),
        };

    }

    // Defer to a higher level
    let (mut tree, mut tokens) = parse(&tokens, level + 1);

    while !tokens.is_empty() {
        if !LEVELS[level].contains(&tokens[0]) {
            return (tree, tokens);
        }

        let op = Op::try_from(&tokens[0]).unwrap();
        let rhs;
        
        (rhs, tokens) = parse(&tokens[1..], level + 1);

        tree = Box::new(Expr::Binary {
            op,
            left: tree,
            right: rhs
        });
    }

    (tree, tokens)
}

fn parse_expr(tokens: &[Token]) -> Box<Expr> {
    let (tree, _) = parse(tokens, 0);
    tree
}

fn lex(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' => { chars.next(); }
            '0'..='9' => {
                let mut n = 0u64;
                while let Some(d) = chars.next_if(|c| c.is_ascii_digit()) {
                    n = n * 10 + d.to_digit(10).unwrap() as u64;
                }
                tokens.push(Token::Num(n))
            }
            _ => {
                let token = match c {
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Star,
                    '/' => Token::Slash,
                    '(' => Token::Lpar,
                    ')' => Token::Rpar,
                    _ => panic!("Unexpected char: {c}"),
                };
                tokens.push(token);
                chars.next();
            }
        }
    }

    return tokens;
}

fn main() {
    let in1 = "(2 + -3) * 4";
    let in2 = "2 * 3 * -4";
    let in3 = "2 + 3 * 4 + 1";

    let lex1 = lex(in1);
    let lex2 = lex(in2);
    let lex3 = lex(in3);

    let tree1 = parse_expr(&lex1);
    let tree2 = parse_expr(&lex2);
    let tree3 = parse_expr(&lex3);

    println!("1: {in1:<15} {:?}", lex1);
    println!("2: {in2:<15} {:?}", lex2);
    println!("3: {in3:<15} {:?}", lex3);
    println!();
    println!("1: {:?}", tree1);
    println!("2: {:?}", tree2);
    println!("3: {:?}", tree3);
}