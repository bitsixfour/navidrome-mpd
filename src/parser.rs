#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr{
    And(Box<Expr>, Box <Expr>),
    Or(Vec<Expr>),
    Not(Box<Expr>),


}
pub enum Op {
    Eq,
    Ne, 
    Contains,
    StartsWith,
}

pub struct Parser {
    expr: Expr,
    reg: Op,
    input: &str, // might be needed

}


pub fn expr_parse(io: &str) -> Result<Expr, String> {
    let mut parser = Parser {;

impl Parser {
    

}
