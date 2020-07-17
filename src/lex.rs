use anyhow::anyhow;

use crate::Token;

pub fn lex<'input>(input: &'input str) -> impl Iterator<Item = anyhow::Result<Token>> + 'input {
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
                .map(Result::Ok)
                .unwrap(),
            | var if var.starts_with(":") => var[1..]
                .parse::<u64>()
                .map(Token::Var)
                .map(Result::Ok)
                .unwrap(),
            | "ap" => Ok(Token::App),
            | "cons" => Ok(Token::Cons),
            | "car" => Ok(Token::Car),
            | "cdr" => Ok(Token::Cdr),
            | "nil" => Ok(Token::Nil),
            | "isnil" => Ok(Token::IsNil),
            | "=" => Ok(Token::Assign),
            | "eq" => Ok(Token::Eq),
            | "lt" => Ok(Token::Lt),
            | "add" => Ok(Token::Add),
            | "mul" => Ok(Token::Mul),
            | "div" => Ok(Token::Div),
            | "neg" => Ok(Token::Neg),
            | "b" => Ok(Token::B),
            | "c" => Ok(Token::C),
            | "s" => Ok(Token::S),
            | "i" => Ok(Token::I),
            | "t" => Ok(Token::Bool(true)),
            | "f" => Ok(Token::Bool(false)),
            | "galaxy" => Ok(Token::Galaxy),
            | token => Err(anyhow!("Unrecognized token on line {}: {}", line, token)),
            }
        })
}
