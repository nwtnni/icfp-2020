use std::collections::HashMap;
use std::iter::Peekable;
use std::rc::Rc;

use crate::ast;
use crate::Token;

pub fn interaction_protocol<I: IntoIterator<Item = Token>>(
    tokens: I,
) -> ast::Protocol {
    let mut tokens = tokens.into_iter();
    let mut assignments = HashMap::new();
    while let Some((var, exp)) = assign(&mut tokens) {
        assignments.insert(var, Rc::new(exp));
    }
    ast::Protocol {
        assignments: Rc::new(assignments),
        galaxy: galaxy(&mut tokens),
    }
}

pub fn test_suite<I: IntoIterator<Item = Token>>(
    tokens: I,
) -> ast::TestSuite {
    let mut tokens = tokens.into_iter().peekable();
    let mut equals = Vec::new();
    while let Some(t) = test(&mut tokens) {
        equals.push(t);
    }
    ast::TestSuite {
        equals,
    }
}

fn test<I: Iterator<Item = Token>>(
    tokens: &mut Peekable<I>
) -> Option<ast::Test> {
    let mut assignments = HashMap::new();
    while let Some(Token::Var(_)) = dbg!(tokens.peek()) {
        let (var, exp) = assign(tokens).expect("Failed to parse expression for assignment");
        assignments.insert(var, Rc::new(exp));
    }
    Some(dbg!(ast::Test{assignments: Rc::new(assignments), equal: equal(tokens)?}))
}

fn assign<I: Iterator<Item = Token>>(
    tokens: &mut I,
) -> Option<(u64, ast::Exp)> {
    let var = match tokens.next() {
    | Some(Token::Var(var)) => var,
    | Some(Token::Galaxy) => return None,
    | _ => panic!("Invalid assignment: expected var or 'galaxy' token"),
    };

    match dbg!(tokens.next()) {
    | Some(Token::Assign) => (),
    | _ => panic!("Invalid assignment: expected '=' token"),
    }

    Some((var, exp(tokens)?))
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

fn equal<I: Iterator<Item = Token>>(
    tokens: &mut I,
) -> Option<ast::Equal> {
    let lhs = exp(tokens)?;

    match tokens.next() {
    | Some(Token::Assign) => (),
    | _ => panic!("Invalid equality: expected '=' token"),
    }

    Some(ast::Equal {
        lhs,
        rhs: exp(tokens)?,
    })
}

fn exp<I: Iterator<Item = Token>>(
    tokens: &mut I,
) -> Option<ast::Exp> {
    use Token::*;
    let exp = match tokens.next()? {
    | Var(var) => ast::Exp::Var(var),
    | Int(int) => ast::Exp::Int(int),
    | Bool(bool) => ast::Exp::Bool(bool),
    | Neg => ast::Exp::Neg,
    | Inc => ast::Exp::Inc,
    | Dec => ast::Exp::Dec,
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
    | App => ast::Exp::App(exp(tokens).map(Rc::new)?, exp(tokens).map(Rc::new)?),
    | _ => panic!("Invalid expression"),
    };
    Some(exp)
}
