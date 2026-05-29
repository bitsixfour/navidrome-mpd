#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    And(Field, Field),
    Or(Field, Field),
    Field(Field),
    Empty,
}
#[derive(Debug, Clone, PartialEq, Eq)]
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


mod parser {
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
            }
        }

        tokens
    }


    fn parse(&self) -> Vec<Token> {
        let mut fields: Vec<Field> = Vec::new();
        let mut ops: Vec<Token> = Vec::new();
        let mut i = 0;

        while i + 2 < self.tokens.len() {
            match &self.tokens[i] {
                Token::Field(f) => {
                    let op = match &self.tokens[i + 1] {
                        Token::Eq => "==",
                        Token::NotEq => "!=",
                        _ => return Expr::Empty,
                    };
                    if let Token::Value(v) = &self.tokens[i + 2] {
                        fields.push(Field { field: f.clone(), op: op.into(), value: v.clone() });
                    }
                    i += 3;
                }
                Token::And | Token::Or => {
                    ops.push(self.tokens[i].clone());
                    i += 1;
                }
                _ => return Expr::Empty,
            }
        }

        match fields.len() {
            0 => Expr::Empty,
            1 => Expr::Field(fields.remove(0)),
            _ => {
                let b = fields.remove(1);
                let a = fields.remove(0);
                match ops.remove(0) {
                    Token::And => Expr::And(a, b),
                    _ => Expr::Or(a, b),
                }
            }
        }
    }
}


