#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Field { field: String, op: String, value: String },
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    LParen,
    RParen,
    And,
    Or,
    Not,
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
                } else {
                    tokens.push(Token::Op("=".to_string()));
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
                    "NOT"      => Token::Not,
                    "contains" => Token::Op("contains".to_string()),
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
        }
    }

    tokens
}

struct Parser {
    tokens: Vec<Token>,
    size: i32,
    pos: i32,
}

impl Parser {
    fn new(needed_data: &str) -> Self {
        let vec: Vec<Token> = tokenize(needed_data);
        Self {
            tokens: vec,
            size: vec.len() as i32,
            pos: 0,
        }
    }
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos as usize)
    }


    fn consume(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos as usize).cloned();
        self.pos += 1;
        token
    }
    fn expect(&mut self, expected: &Token) -> Result<Token, String> {
        match self.consume() {
            Some(t) if std::mem::discriminant(&t) == std::mem::discriminant(expected) => Ok(t),
            Some(t) => Err(format!("Expected {:?}, got {:?}", expected, t)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.expect(&Token::LParen)?;

        if self.peek() == Some(&Token::Not) {
            self.consume();
            let inner = self.parse_expr()?;
            self.expect(&Token::RParen)?;
            return Ok(Expr::Not(Box::new(inner)));
        }

        if self.peek() == Some(&Token::LParen) {
            let left = self.parse_expr()?;

            let op = self.consume().ok_or("Expected AND or OR")?;

            let right = self.parse_expr()?;
            self.expect(&Token::RParen)?;

            return match op {
                Token::And => Ok(Expr::And(Box::new(left), Box::new(right))),
                Token::Or  => Ok(Expr::Or(Box::new(left), Box::new(right))),
                _ => Err(format!("Expected AND or OR, got {:?}", op)),
            };
        }

        let field = match self.consume() {
            Some(Token::Field(f)) => f,
            other => return Err(format!("Expected field, got {:?}", other)),
        };

        let op = match self.consume() {
            Some(Token::Op(o)) => o,
            other => return Err(format!("Expected op, got {:?}", other)),
        };

        let value = match self.consume() {
            Some(Token::Value(v)) => v,
            other => return Err(format!("Expected value, got {:?}", other)),
        };

        self.expect(&Token::RParen)?;

        Ok(Expr::Field { field, op, value })
    }
}
