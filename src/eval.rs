use typed_arena::Arena;
use std::fmt;

use crate::ast;

pub enum Value<'a> {
    Int(i64),
    Bool(bool),
    Cons(&'a Value<'a>, &'a Value<'a>),
    Var(u64),
    Nil,
    Closure(Box<dyn Fn(&'a ast::Exp<'a>) -> Value<'a> + 'a>)
}

impl<'a> Clone for Value<'a> {
    fn clone(&self) -> Self {
        use Value::*;
        match self {
        | Closure(_) => panic!("Cannot clone closure"),
        | Int(n) => Int(*n),
        | Bool(n) => Bool(*n),
        | Cons(a, b) => Cons(*a, *b),
        | Var(n) => Var(*n),
        | Nil => Nil,
        }
    }
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
        | Closure(_) => panic!("Cannot clone closure"),
        | Int(n) => write!(f, "Int: {}", n),
        | Bool(n) => write!(f, "Bool: {}", n),
        | Cons(n1, n2) => write!(f, "Cons: ( {:?}, {:?} )", n1, n2),
        | Var(n) => write!(f, "Var: {}", n),
        | Nil => write!(f, "Nil"),
        }
    }
}

impl<'a> Eq for Value<'a> {}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
        | (Closure(_), _) => panic!("Cannot clone closure"),
        | (_, Closure(_)) => panic!("Cannot clone closure"),
        | (Int(n1), Int(n2)) => n1 == n2,
        | (Bool(n1), Bool(n2)) => n1 == n2,
        | (Cons(n1, m1), Cons(n2, m2)) => n1 == n2 && m1 == m2,
        | (Var(n1), Var(n2)) => n1 == n2,
        | (Nil, Nil) => true,
        | _ => false,
        }
    }
}

pub fn eval_wack<'arena>(
    arena_expr: &'arena Arena<ast::Exp<'arena>>,
    arena_value: &'arena Arena<Value<'arena>>,
    expr: &ast::Exp<'arena>,
) -> Value<'arena> {
    use ast::Exp::*;
    match *expr {
    | Nil => Value::Nil,
    | Int(n) => Value::Int(n),
    | Var(name) => Value::Var(name),
    | Bool(b) => Value::Bool(b),
    | App(f, v) =>
        match eval_wack(arena_expr, arena_value, f) {
        | Value::Closure(func) => func(v),
        | Value::Bool(true) => todo!("hmm"),
        | _ => panic!("Can't apply values that are not closures or bool")
        },
    | Neg => Value::Closure(Box::new(move |e| {
        match eval_wack(arena_expr, arena_value, e) {
        | Value::Int(n) => Value::Int(-n),
        | _ => panic!("hmm"),
        }
    }
    ).leak()),
    | _ => todo!()
    }
}

pub fn eval<'arena>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    expr: &ast::Exp<'arena>,
) -> ast::Exp<'arena> {
    use ast::Exp::*;
    match *expr {
    | Nil => Nil,
    | Int(n) => Int(n),
    | Var(name) => Var(name),
    | Bool(b) => Bool(b),
    | App(Neg, e) => Int(-eval_int(arena, e)),
    | App(App(Add, e1), e2) => {
        let i1: i64 = eval_int(arena, e1);
        let i2: i64 = eval_int(arena, e2);
        Int(i1 + i2)
    }
    | App(App(Mul, e1), e2) => {
        let i1: i64 = eval_int(arena, e1);
        let i2: i64 = eval_int(arena, e2);
        Int(i1 * i2)
    }
    | _ => todo!("Add more cases"),
    }
}

fn eval_int<'arena>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    expr: &ast::Exp<'arena>,
) -> i64 {
    use ast::Exp::*;
    match eval(arena, expr) {
    | Int(n) => n,
    | _ => panic!("Expected int"),
    }
}
