use typed_arena::Arena;

use crate::ast;
use crate::Token;

pub fn parse<'arena, I: IntoIterator<Item = Token>>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    tokens: I,
) -> ast::Program<'arena> {
    let mut tokens = tokens.into_iter();
    let mut stms = Vec::new();
    while let Some(stm) = parse_stm(arena, &mut tokens) {
        stms.push(stm);
    }
    ast::Program { stms }
}

fn parse_stm<'arena, I: Iterator<Item = Token>>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    tokens: &mut I,
) -> Option<ast::Stm<'arena>> {
    let var = match tokens.next() {
    | Some(Token::Var(var)) => var,
    | _ => return None,
    };

    match tokens.next() {
    | Some(Token::Assign) => (),
    | _ => panic!("Invalid statement: expected '='"),
    }

    Some(ast::Stm {
        var,
        exp: parse_exp(arena, tokens),
    })
}

fn parse_exp<'arena, I: Iterator<Item = Token>>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    tokens: &mut I,
) -> ast::Exp<'arena> {
    use Token::*;
    match tokens.next() {
    | Some(Var(var)) => ast::Exp::Var(var),
    | Some(Int(int)) => ast::Exp::Int(int),
    | Some(Bool(bool)) => ast::Exp::Bool(bool),
    | Some(Neg) => ast::Exp::Neg,
    | Some(Add) => ast::Exp::Add,
    | Some(Mul) => ast::Exp::Mul,
    | Some(Div) => ast::Exp::Div,
    | Some(Eq) => ast::Exp::Eq,
    | Some(Lt) => ast::Exp::Lt,
    | Some(S) => ast::Exp::S,
    | Some(C) => ast::Exp::C,
    | Some(B) => ast::Exp::B,
    | Some(I) => ast::Exp::I,
    | Some(Cons) => ast::Exp::Cons,
    | Some(Car) => ast::Exp::Car,
    | Some(Cdr) => ast::Exp::Cdr,
    | Some(Nil) => ast::Exp::Nil,
    | Some(IsNil) => ast::Exp::IsNil,
    | Some(Galaxy) => ast::Exp::Galaxy,
    | Some(App) => ast::Exp::App(
        arena.alloc(parse_exp(arena, tokens)),
        arena.alloc(parse_exp(arena, tokens)),
    ),
    | _ => panic!("Invalid expression"),
    }
}
