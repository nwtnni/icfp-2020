use crate::Token;

pub fn lex<'input>(input: &'input str) -> impl Iterator<Item = Token> + 'input {
    input
        .trim()
        .split('\n')
        .enumerate()
        .filter(|(_, text)| !text.is_empty())
        .flat_map(|(line, text)| {
            text.trim()
                .split_whitespace()
                .map(move |word| (line, word))
        })
        .map(|(line, token)| {
            match token {
            | int if int.parse::<i64>().is_ok() => int
                .parse::<i64>()
                .map(Token::Int)
                .unwrap(),
            | var if var.starts_with(":") | var.starts_with("x") => var[1..]
                .parse::<u64>()
                .map(Token::Var)
                .expect("Expected variable to be valid u64"),
            | "ap" => Token::App,
            | "cons" => Token::Cons,
            | "car" => Token::Car,
            | "cdr" => Token::Cdr,
            | "nil" => Token::Nil,
            | "isnil" => Token::IsNil,
            | "=" => Token::Assign,
            | "eq" => Token::Eq,
            | "lt" => Token::Lt,
            | "add" => Token::Add,
            | "mul" => Token::Mul,
            | "div" => Token::Div,
            | "neg" => Token::Neg,
            | "b" => Token::B,
            | "c" => Token::C,
            | "s" => Token::S,
            | "i" => Token::I,
            | "t" => Token::Bool(true),
            | "f" => Token::Bool(false),
            | "galaxy" => Token::Galaxy,
            | token => panic!(format!("Unrecognized token on line {}: {}", line, token)),
            }
        })
}
