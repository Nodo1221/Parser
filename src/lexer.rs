#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Num(u64),
    Plus, Minus, Star, Slash,
    Lpar, Rpar,
}

pub fn lex(input: &str) -> Vec<Token> {
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