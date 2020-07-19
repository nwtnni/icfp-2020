use std::collections::HashMap;
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
        assignments,
        galaxy: galaxy(&mut tokens),
    }
}

fn assign<I: Iterator<Item = Token>>(
    tokens: &mut I,
) -> Option<(u64, ast::Exp)> {
    let var = match tokens.next() {
    | Some(Token::Var(var)) => var,
    | Some(Token::Galaxy) => return None,
    | _ => panic!("Invalid assignment: expected var or 'galaxy' token"),
    };

    match tokens.next() {
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

pub fn exp<I: Iterator<Item = Token>>(
    tokens: &mut I,
) -> Option<ast::Exp> {
    use Token::*;
    let exp = match tokens.next()? {
    | Var(var) => ast::Exp::Atom(ast::Atom::Var(var)),
    | Int(int) => ast::Exp::Atom(ast::Atom::Int(int)),
    | Bool(bool) => ast::Exp::Atom(ast::Atom::Bool(bool)),
    | Neg => ast::Exp::Atom(ast::Atom::Neg),
    | Inc => ast::Exp::Atom(ast::Atom::Inc),
    | Dec => ast::Exp::Atom(ast::Atom::Dec),
    | Add => ast::Exp::Atom(ast::Atom::Add),
    | Mul => ast::Exp::Atom(ast::Atom::Mul),
    | Div => ast::Exp::Atom(ast::Atom::Div),
    | Eq => ast::Exp::Atom(ast::Atom::Eq),
    | Lt => ast::Exp::Atom(ast::Atom::Lt),
    | S => ast::Exp::Atom(ast::Atom::S),
    | C => ast::Exp::Atom(ast::Atom::C),
    | B => ast::Exp::Atom(ast::Atom::B),
    | I => ast::Exp::Atom(ast::Atom::I),
    | Cons => ast::Exp::Atom(ast::Atom::Cons),
    | Car => ast::Exp::Atom(ast::Atom::Car),
    | Cdr => ast::Exp::Atom(ast::Atom::Cdr),
    | Nil => ast::Exp::Atom(ast::Atom::Nil),
    | IsNil => ast::Exp::Atom(ast::Atom::IsNil),
    | Galaxy => ast::Exp::Atom(ast::Atom::Galaxy),
    | App => ast::Exp::App(exp(tokens).map(Rc::new)?, exp(tokens).map(Rc::new)?, Default::default()),
    | _ => panic!("Invalid expression"),
    };
    Some(exp)
}
