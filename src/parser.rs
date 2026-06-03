use crate::lexer::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Expr {
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
pub enum Op { Add, Subs, Mul, Div, Neg }

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseError {
    UnexpectedToken(Token)
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

pub fn parse_expr(tokens: &[Token]) -> Box<Expr> {
    let (tree, _) = parse(tokens, 0);
    tree
}