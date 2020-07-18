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
        exp: exp(arena, tokens)?,
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
    let lhs = exp(arena, tokens)?;

    match tokens.next() {
    | Some(Token::Assign) => (),
    | _ => panic!("Invalid equality: expected '=' token"),
    }

    Some(ast::Equal {
        lhs,
        rhs: exp(arena, tokens)?,
    })
}

fn exp<'arena, I: Iterator<Item = Token>>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    tokens: &mut I,
) -> Option<ast::Exp<'arena>> {
    use Token::*;
    let exp = match tokens.next()? {
    | Var(var) => ast::Exp::Var(var),
    | Int(int) => ast::Exp::Int(int),
    | Bool(bool) => ast::Exp::Bool(bool),
    | Neg => ast::Exp::Neg,
    | Add => ast::Exp::Add,
    | Mul => ast::Exp::Mul,
    | Div => ast::Exp::Div,
    | Eq => ast::Exp::Eq,
    | Lt => ast::Exp::Lt,
    | S => ast::Exp::S,
    | C => ast::Exp::C,
    | B => ast::Exp::B,
    | I => ast::Exp::I,
    | Cons => ast::Exp::Cons,
    | Car => ast::Exp::Car,
    | Cdr => ast::Exp::Cdr,
    | Nil => ast::Exp::Nil,
    | IsNil => ast::Exp::IsNil,
    | Galaxy => ast::Exp::Galaxy,
    | App => ast::Exp::App(
        arena.alloc(exp(arena, tokens)?),
        arena.alloc(exp(arena, tokens)?),
    ),
    | _ => panic!("Invalid expression"),
    };
    Some(exp)
}
