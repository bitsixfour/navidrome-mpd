#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    And(Expr, Expr),
    Or(Expr, Expr),
    Not(Expr),
}
pub struct Field {
    pub field: String,
    pub op: String,
    pub value: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    And,
    Or,
    Not,
    Eq, 
    NotEq,
    Field(String),
    Value(String),
}

fn tokenize_and_filter(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            /* uneeded: sort vec by just not including this
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
            } */
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                   tokens.push(Token::Eq);
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::NotEq);
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
                    "NOT"      => Token::Not,
                    "contains" => Token::NotEq,
                    _          => Token::Field(word),
                };
                tokens.push(token);
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() {
                        num.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Value(num));
            }
            _ => { chars.next(); } 
            /* ex: ARTIST EQUALS VALUE OR ALBUM EQUALS WHATEVER */
        }
    }

    tokens
}

struct Parser {
    tokens: Vec<Token>,
    size: i32,
}

impl Parser {
    fn new(needed_data: &str) -> Self {
        let vec: Vec<Token> = tokenize_and_filter(needed_data);
        Self {
            tokens: vec,
            size: vec.len() as i32,
        }
    }

    fn create_eval(&self) -> Expr  {
        
        
        
        

    }


impl Expr {
    fn create_eval(&self) -> Self {
        match self {
            Expr::And(a, b) => 
            Expr::Or(a, b) =>
            Expr::Not(a) =>



        }

    }

}


