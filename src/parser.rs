#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    And(Box<Expr>, Box<Expr>),
    Or(Vec<Expr>),
    Not(Box<Expr>),
    Field { field: String, op: Op, value: String },
}


#[derive(Debug, Clone, PartialEq)]
enum Token {
    LParen,
    RParen,
    And,
    Or,
    Field(String),
    Op(String),
    Value(String),
}
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' => { chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '"' | '\'' => {
                let quote = c;
                chars.next();
                let mut value = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == quote { break; }
                    value.push(ch);
                    chars.next();
                }
                chars.next(); // closing quote
                tokens.push(Token::Value(value));
            }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::Op("==".to_string()));
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::Op("!=".to_string()));
                }
            }
            'A'..='Z' | 'a'..='z' => {
                let mut word = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphabetic() {
                        word.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let token = match word.as_str() {
                    "AND"      => Token::And,
                    "OR"       => Token::Or,
                    "contains" => Token::Op("contains".to_string()),
                    _          => Token::Field(word),
                };
                tokens.push(token);
            }
            _ => { chars.next(); }
        }
    }

    tokens
}
