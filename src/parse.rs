use typed_arena::Arena;

use crate::ast;
use crate::Token;

pub fn interaction_protocol<'arena, I: IntoIterator<Item = Token>>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    tokens: I,
) -> ast::Protocol<'arena> {
    let mut tokens = tokens.into_iter();
    let mut assignments = Vec::new();
    while let Some(stm) = assign(arena, &mut tokens) {
        assignments.push(stm);
    }
    ast::Protocol {
        assignments,
        galaxy: galaxy(&mut tokens),
    }
}

pub fn test_suite<'arena, I: IntoIterator<Item = Token>>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    tokens: I,
) -> ast::TestSuite<'arena> {
    let mut tokens = tokens.into_iter();
    let mut equals = Vec::new();
    while let Some(equal) = equal(arena, &mut tokens) {
        equals.push(equal);
    }
    ast::TestSuite {
        equals,
    }
}

fn assign<'arena, I: Iterator<Item = Token>>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    tokens: &mut I,
) -> Option<ast::Assign<'arena>> {
    let var = match tokens.next() {
    | Some(Token::Var(var)) => var,
    | Some(Token::Galaxy) => return None,
    | _ => panic!("Invalid assignment: expected var or 'galaxy' token"),
    };

    match dbg!(tokens.next()) {
    | Some(Token::Assign) => (),
    | _ => panic!("Invalid assignment: expected '=' token"),
    }

    Some(ast::Assign {
        var,
        exp: exp(arena, tokens),
    })
}

fn galaxy<'arena, I: Iterator<Item = Token>>(tokens: &mut I) -> u64 {
    match tokens.next() {
    | Some(Token::Assign) => (),
    | _ => panic!("Invalid galaxy: expected '=' token"),
    }

    match tokens.next() {
    | Some(Token::Var(var)) => var,
    | _ => panic!("Expected galaxy var token"),
    }
}

fn equal<'arena, I: Iterator<Item = Token>>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    tokens: &mut I,
) -> Option<ast::Equal<'arena>> {
    let lhs = exp(arena, tokens);

    match tokens.next() {
    | Some(Token::Assign) => (),
    | _ => panic!("Invalid equality: expected '=' token"),
    }

    Some(ast::Equal {
        lhs,
        rhs: exp(arena, tokens),
    })
}

fn exp<'arena, I: Iterator<Item = Token>>(
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
        arena.alloc(exp(arena, tokens)),
        arena.alloc(exp(arena, tokens)),
    ),
    | _ => panic!("Invalid expression"),
    }
}
